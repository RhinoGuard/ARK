use argon2::{Algorithm, Argon2, Params, Version};
use zeroize::Zeroizing;

use super::KEY_LEN;

/// Maximum accepted password length in bytes.
///
/// Passwords exceeding this threshold are rejected fail-closed before allocating
/// Argon2id memory blocks.
pub const MAX_PASSWORD_LEN: usize = 1024;

/// Minimum accepted Argon2id salt length in bytes (128 bits, per RFC 9106).
pub const MIN_SALT_LEN: usize = 16;

/// Recommended default Argon2id salt length in bytes (256 bits).
pub const DEFAULT_SALT_LEN: usize = 32;

/// Maximum accepted Argon2id salt length in bytes (512 bits).
pub const MAX_SALT_LEN: usize = 64;

/// Minimum ARK production-policy memory cost in KiB (16 MiB).
pub const MIN_MEMORY_KIB: u32 = 16 * 1024;

/// Recommended default ARK production-policy memory cost in KiB (64 MiB).
pub const DEFAULT_MEMORY_KIB: u32 = 64 * 1024;

/// Hard upper bound for ARK Argon2id memory cost in KiB (256 MiB).
///
/// Enforced unconditionally to prevent unbounded allocation or local memory
/// exhaustion denial-of-service from untrusted package headers.
pub const MAX_MEMORY_KIB: u32 = 256 * 1024;

/// Minimum ARK production-policy iteration count (time cost `t`).
pub const MIN_ITERATIONS: u32 = 1;

/// Recommended default ARK production-policy iteration count (time cost `t`).
pub const DEFAULT_ITERATIONS: u32 = 3;

/// Hard upper bound for ARK Argon2id iteration count (time cost `t`).
pub const MAX_ITERATIONS: u32 = 10;

/// Minimum ARK production-policy parallelism degree (lanes `p`).
pub const MIN_LANES: u32 = 1;

/// Recommended default ARK production-policy parallelism degree (lanes `p`).
pub const DEFAULT_LANES: u32 = 1;

/// Hard upper bound for ARK Argon2id parallelism degree (lanes `p`).
pub const MAX_LANES: u32 = 8;

/// Validated Argon2id salt wrapper enforcing ARK's salt length and non-degeneracy policy.
///
/// # Security Contract
///
/// - Length MUST be within [`MIN_SALT_LEN`]..=[`MAX_SALT_LEN`] (`16..=64` bytes).
/// - All-zero degenerate salts are rejected via [`Argon2Error::DegenerateSalt`] to
///   catch uninitialized buffer bugs at caller boundaries.
/// - Callers MUST generate a fresh, unique salt per password-derivation domain
///   using a cryptographically secure random number generator (CSPRNG). Fixed,
///   reused, or secret salts MUST NOT be treated as a substitute for password entropy.
#[derive(Clone, PartialEq, Eq)]
pub struct Argon2Salt {
    bytes: [u8; MAX_SALT_LEN],
    len: usize,
}

impl Argon2Salt {
    /// Validate and construct an [`Argon2Salt`] from a caller-supplied byte slice.
    ///
    /// # Errors
    ///
    /// Returns [`Argon2Error::InvalidSaltLength`] if `bytes.len()` is outside
    /// [`MIN_SALT_LEN`]..=[`MAX_SALT_LEN`], or [`Argon2Error::DegenerateSalt`]
    /// if `bytes` consists entirely of `0x00` bytes.
    pub fn from_slice(bytes: &[u8]) -> Result<Self, Argon2Error> {
        if !(MIN_SALT_LEN..=MAX_SALT_LEN).contains(&bytes.len()) {
            return Err(Argon2Error::InvalidSaltLength);
        }
        if bytes.iter().all(|&b| b == 0) {
            return Err(Argon2Error::DegenerateSalt);
        }
        let mut buf = [0_u8; MAX_SALT_LEN];
        buf[..bytes.len()].copy_from_slice(bytes);
        Ok(Self {
            bytes: buf,
            len: bytes.len(),
        })
    }

    /// Borrow the validated salt bytes.
    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes[..self.len]
    }
}

impl core::fmt::Debug for Argon2Salt {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Argon2Salt")
            .field("len", &self.len)
            .finish_non_exhaustive()
    }
}

/// Argon2id cost configuration with ARK production safety bounds.
///
/// All parameter combinations passed to [`derive_kek`] are checked against
/// hard upper bounds ([`MAX_MEMORY_KIB`], [`MAX_ITERATIONS`], [`MAX_LANES`])
/// and production floor bounds ([`MIN_MEMORY_KIB`], [`MIN_ITERATIONS`], [`MIN_LANES`]).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Argon2idConfig {
    /// Memory cost in KiB.
    pub memory_kib: u32,
    /// Number of passes (iterations).
    pub iterations: u32,
    /// Degree of parallelism (lanes).
    pub lanes: u32,
}

impl Argon2idConfig {
    /// Construct and validate an [`Argon2idConfig`] against ARK's production policy bounds.
    ///
    /// # Errors
    ///
    /// Returns [`Argon2Error::InvalidParameters`] if any parameter falls outside
    /// the allowed policy range (`memory_kib` in `16_384..=262_144`, `iterations`
    /// in `1..=10`, `lanes` in `1..=8`, and `memory_kib >= 8 * lanes`).
    pub fn new(memory_kib: u32, iterations: u32, lanes: u32) -> Result<Self, Argon2Error> {
        let config = Self {
            memory_kib,
            iterations,
            lanes,
        };
        config.validate()?;
        Ok(config)
    }

    /// Return the memory cost in KiB.
    #[must_use]
    pub const fn memory_kib(&self) -> u32 {
        self.memory_kib
    }

    /// Return the iteration count.
    #[must_use]
    pub const fn iterations(&self) -> u32 {
        self.iterations
    }

    /// Return the parallelism degree (lanes).
    #[must_use]
    pub const fn lanes(&self) -> u32 {
        self.lanes
    }

    /// Validate that this configuration satisfies ARK's production parameter bounds
    /// as well as RFC 9106 structural constraints.
    ///
    /// # Errors
    ///
    /// Returns [`Argon2Error::InvalidParameters`] if any cost parameter violates
    /// the production floor or ceiling bounds.
    pub fn validate(&self) -> Result<(), Argon2Error> {
        if !(MIN_MEMORY_KIB..=MAX_MEMORY_KIB).contains(&self.memory_kib) {
            return Err(Argon2Error::InvalidParameters);
        }
        if !(MIN_ITERATIONS..=MAX_ITERATIONS).contains(&self.iterations) {
            return Err(Argon2Error::InvalidParameters);
        }
        if !(MIN_LANES..=MAX_LANES).contains(&self.lanes) {
            return Err(Argon2Error::InvalidParameters);
        }
        let _ = Params::new(self.memory_kib, self.iterations, self.lanes, Some(KEY_LEN))
            .map_err(|_| Argon2Error::InvalidParameters)?;
        Ok(())
    }

    /// Validate that this configuration satisfies RFC 9106 structural minimums
    /// and ARK's hard upper DoS ceilings, allowing low-memory official RFC 9106
    /// Known-Answer Test (KAT) vectors (`m >= 8 * p`).
    #[cfg(test)]
    fn validate_rfc_kat_bounds(&self, associated_data: &[u8]) -> Result<Params, Argon2Error> {
        if self.memory_kib > MAX_MEMORY_KIB
            || !(MIN_ITERATIONS..=MAX_ITERATIONS).contains(&self.iterations)
            || !(MIN_LANES..=MAX_LANES).contains(&self.lanes)
        {
            return Err(Argon2Error::InvalidParameters);
        }
        let ad = argon2::AssociatedData::new(associated_data)
            .map_err(|_| Argon2Error::InvalidParameters)?;
        let mut builder = argon2::ParamsBuilder::new();
        builder
            .m_cost(self.memory_kib)
            .t_cost(self.iterations)
            .p_cost(self.lanes)
            .data(ad)
            .output_len(KEY_LEN);
        builder.build().map_err(|_| Argon2Error::InvalidParameters)
    }
}

impl Default for Argon2idConfig {
    fn default() -> Self {
        Self {
            memory_kib: DEFAULT_MEMORY_KIB,
            iterations: DEFAULT_ITERATIONS,
            lanes: DEFAULT_LANES,
        }
    }
}

/// Derive a 256-bit key-encryption key (KEK) from a password and validated [`Argon2Salt`].
///
/// Enforces ARK's full production parameter policy ([`Argon2idConfig::validate`])
/// and non-degenerate salt requirements ([`Argon2Salt`]).
///
/// # Errors
///
/// Returns [`Argon2Error`] if the password is empty or exceeds [`MAX_PASSWORD_LEN`],
/// if `config` violates ARK's production bounds, or if derivation fails.
pub fn derive_kek_with_salt(
    password: &[u8],
    salt: &Argon2Salt,
    config: Argon2idConfig,
) -> Result<Zeroizing<[u8; KEY_LEN]>, Argon2Error> {
    derive_kek(password, salt.as_bytes(), config)
}

/// Derive a 256-bit key-encryption key (KEK) from a password and caller-supplied salt.
///
/// This function does not generate randomness. The caller must supply a fresh salt
/// generated by the credential-lifecycle layer using the platform OS CSPRNG when a
/// new password-derived wrapper is created. For unlock/verification, supply the exact
/// validated salt loaded from that wrapper's persisted metadata.
///
/// Enforces ARK's strict production parameter policy (`memory_kib` in `16 MiB..=256 MiB`,
/// `iterations` in `1..=10`, `lanes` in `1..=8`), password length bounds (`1..=1024` bytes),
/// and salt policy (`16..=64` bytes, non-all-zero).
///
/// # Errors
///
/// Returns [`Argon2Error`] if the password is empty or exceeds [`MAX_PASSWORD_LEN`],
/// if the salt violates length or degeneracy rules, if `config` falls outside ARK's
/// production parameter bounds, or if the underlying Argon2id operation fails.
pub fn derive_kek(
    password: &[u8],
    salt: &[u8],
    config: Argon2idConfig,
) -> Result<Zeroizing<[u8; KEY_LEN]>, Argon2Error> {
    if password.is_empty() {
        return Err(Argon2Error::EmptyPassword);
    }
    if password.len() > MAX_PASSWORD_LEN {
        return Err(Argon2Error::PasswordTooLong);
    }
    let validated_salt = Argon2Salt::from_slice(salt)?;
    config.validate()?;

    let params = Params::new(
        config.memory_kib,
        config.iterations,
        config.lanes,
        Some(KEY_LEN),
    )
    .map_err(|_| Argon2Error::InvalidParameters)?;

    let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);
    let mut output = Zeroizing::new([0_u8; KEY_LEN]);
    argon2
        .hash_password_into(password, validated_salt.as_bytes(), output.as_mut())
        .map_err(|_| Argon2Error::DerivationFailed)?;

    Ok(output)
}

/// Derive a 256-bit Argon2id tag using RFC 9106 structural parameter bounds
/// (including optional secret key `K` and associated data `X`) for official
/// RFC 9106 Known-Answer Test (KAT) conformance verification.
///
/// Unlike [`derive_kek`], this function permits RFC 9106 low-memory test vectors
/// (such as `m = 32 KiB`) and arbitrary valid RFC salt byte patterns while still
/// enforcing ARK's hard upper memory and iteration ceilings.
///
/// # Errors
///
/// Returns [`Argon2Error`] if inputs or parameters violate RFC 9106 or ARK upper
/// ceilings.
#[cfg(test)]
pub(super) fn derive_argon2id_rfc9106_kat(
    password: &[u8],
    salt: &[u8],
    secret: &[u8],
    associated_data: &[u8],
    config: Argon2idConfig,
) -> Result<Zeroizing<[u8; KEY_LEN]>, Argon2Error> {
    if password.is_empty() {
        return Err(Argon2Error::EmptyPassword);
    }
    if password.len() > MAX_PASSWORD_LEN {
        return Err(Argon2Error::PasswordTooLong);
    }
    if !(8..=MAX_SALT_LEN).contains(&salt.len()) {
        return Err(Argon2Error::InvalidSaltLength);
    }
    let params = config.validate_rfc_kat_bounds(associated_data)?;
    let argon2 = Argon2::new_with_secret(secret, Algorithm::Argon2id, Version::V0x13, params)
        .map_err(|_| Argon2Error::InvalidParameters)?;
    let mut output = Zeroizing::new([0_u8; KEY_LEN]);
    argon2
        .hash_password_into(password, salt, output.as_mut())
        .map_err(|_| Argon2Error::DerivationFailed)?;
    Ok(output)
}

/// Sanitized error classification for Argon2id KEK derivation.
///
/// Intentionally carries no input data, password bytes, salt bytes, or derived
/// key material to satisfy `ARK-SEC-028`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Argon2Error {
    /// The supplied password byte slice is empty.
    EmptyPassword,
    /// The supplied password byte slice exceeds [`MAX_PASSWORD_LEN`].
    PasswordTooLong,
    /// The supplied salt length is outside [`MIN_SALT_LEN`]..=[`MAX_SALT_LEN`].
    InvalidSaltLength,
    /// The supplied salt consists entirely of zero bytes (`0x00`).
    DegenerateSalt,
    /// The supplied Argon2id cost parameters violate ARK policy or RFC 9106 bounds.
    InvalidParameters,
    /// The underlying Argon2id hashing operation failed.
    DerivationFailed,
}

impl core::fmt::Display for Argon2Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let message = match self {
            Self::EmptyPassword => "password must not be empty",
            Self::PasswordTooLong => "password exceeds the configured input limit",
            Self::InvalidSaltLength => "salt length must be between 16 and 64 bytes",
            Self::DegenerateSalt => "salt must not be an all-zero degenerate buffer",
            Self::InvalidParameters => "invalid Argon2id parameters",
            Self::DerivationFailed => "Argon2id derivation failed",
        };
        f.write_str(message)
    }
}

impl std::error::Error for Argon2Error {}
