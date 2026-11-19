use argon2::{Algorithm, Argon2, Params, Version};
use zeroize::Zeroizing;

use super::KEY_LEN;

/// Provisional Argon2id settings for development and benchmarking.
///
/// These values are deliberately configurable and MUST be benchmarked on
/// supported target hardware before ARK accepts them as a product policy.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Argon2idConfig {
    /// Memory cost in KiB.
    pub memory_kib: u32,
    /// Number of iterations.
    pub iterations: u32,
    /// Degree of parallelism.
    pub lanes: u32,
}

impl Default for Argon2idConfig {
    fn default() -> Self {
        Self {
            memory_kib: 64 * 1024,
            iterations: 3,
            lanes: 1,
        }
    }
}

/// Derive a 256-bit key-encryption key (KEK) from a password and random salt.
///
/// # Errors
///
/// Returns [`Argon2Error`] if the password is empty or exceeds the maximum
/// length, if the salt length is outside the accepted bounds, if the Argon2id
/// parameters are invalid, or if the underlying derivation operation fails.
pub fn derive_kek(
    password: &[u8],
    salt: &[u8],
    config: Argon2idConfig,
) -> Result<Zeroizing<[u8; KEY_LEN]>, Argon2Error> {
    if password.is_empty() {
        return Err(Argon2Error::EmptyPassword);
    }
    if password.len() > 1024 {
        return Err(Argon2Error::PasswordTooLong);
    }
    if !(16..=64).contains(&salt.len()) {
        return Err(Argon2Error::InvalidSaltLength);
    }

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
        .hash_password_into(password, salt, output.as_mut())
        .map_err(|_| Argon2Error::DerivationFailed)?;

    Ok(output)
}

/// Sanitized error classification for Argon2id KEK derivation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Argon2Error {
    /// The supplied password byte slice is empty.
    EmptyPassword,
    /// The supplied password byte slice exceeds the maximum length bound.
    PasswordTooLong,
    /// The supplied salt length is outside the accepted byte range.
    InvalidSaltLength,
    /// The supplied Argon2id cost parameters are invalid.
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
            Self::InvalidParameters => "invalid Argon2id parameters",
            Self::DerivationFailed => "Argon2id derivation failed",
        };
        f.write_str(message)
    }
}

impl std::error::Error for Argon2Error {}
