use hkdf::Hkdf;
use sha2::Sha256;
use zeroize::Zeroizing;

use super::KEY_LEN;

/// Canonical ARK v1 domain-separation prefix required for all HKDF context strings.
pub const ARK_V1_CONTEXT_PREFIX: &[u8] = b"ARK/v1/";

/// Minimum accepted input key material (IKM) length in bytes for ARK subkey derivation.
///
/// While input length alone does not prove cryptographic entropy (callers must
/// pass high-entropy key material rather than raw user passwords), enforcing a
/// 16-byte (128-bit) minimum prevents degenerate short tokens from being passed
/// to [`derive_subkey`].
pub const MIN_IKM_LEN: usize = 16;

/// Recommended 256-bit input key material (IKM) length in bytes for ARK root keys.
pub const RECOMMENDED_IKM_LEN: usize = 32;

/// Maximum accepted context length, to keep caller-controlled inputs bounded.
pub const MAX_CONTEXT_LEN: usize = 256;

/// Maximum theoretical output length in bytes for HKDF-SHA-256 (`255 * 32 = 8160` bytes),
/// per RFC 5869 Section 2.3.
pub const HKDF_SHA256_MAX_OUTPUT_LEN: usize = 255 * 32;

/// Enumeration of canonical ARK v1 subkey derivation purposes (`ARK-SEC-016`).
///
/// Using [`HkdfKeyPurpose`] with [`derive_purpose_subkey`] prevents cross-domain
/// context confusion by binding each derived subkey to a distinct, versioned
/// protocol label (`"ARK/v1/<purpose>"`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HkdfKeyPurpose {
    /// Subkey used for workspace master key wrapping (`"ARK/v1/workspace-key"`).
    WorkspaceKey,
    /// Subkey used for project-level key isolation (`"ARK/v1/project-key"`).
    ProjectKey,
    /// Subkey used for file payload encryption (`"ARK/v1/file-key"`).
    FileKey,
    /// Subkey used for authenticated metadata protection (`"ARK/v1/metadata-key"`).
    MetadataKey,
    /// Subkey used for local audit log integrity (`"ARK/v1/audit-key"`).
    AuditKey,
    /// Subkey used for recovery bundle wrapping (`"ARK/v1/recovery-key"`).
    RecoveryKey,
}

impl HkdfKeyPurpose {
    /// Return the canonical `"ARK/v1/<purpose>"` domain-separation label as bytes.
    #[must_use]
    pub const fn as_context_bytes(self) -> &'static [u8] {
        match self {
            Self::WorkspaceKey => b"ARK/v1/workspace-key",
            Self::ProjectKey => b"ARK/v1/project-key",
            Self::FileKey => b"ARK/v1/file-key",
            Self::MetadataKey => b"ARK/v1/metadata-key",
            Self::AuditKey => b"ARK/v1/audit-key",
            Self::RecoveryKey => b"ARK/v1/recovery-key",
        }
    }
}

/// Validated HKDF-SHA-256 domain-separation context string.
///
/// Enforces that every ARK HKDF `info` parameter:
/// 1. Is non-empty and at most [`MAX_CONTEXT_LEN`] (`256`) bytes.
/// 2. Starts with the canonical versioned namespace prefix [`ARK_V1_CONTEXT_PREFIX`] (`"ARK/v1/"`).
/// 3. Has a non-empty purpose suffix after `"ARK/v1/"` containing only printable
///    ASCII graphic characters (`0x21..=0x7E`, excluding whitespace and control bytes).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HkdfContext<'a> {
    bytes: &'a [u8],
}

impl<'a> HkdfContext<'a> {
    /// Construct a validated [`HkdfContext`] from a canonical [`HkdfKeyPurpose`].
    #[must_use]
    pub const fn from_purpose(purpose: HkdfKeyPurpose) -> Self {
        Self {
            bytes: purpose.as_context_bytes(),
        }
    }

    /// Validate a caller-supplied context byte slice against ARK's v1 domain-separation
    /// grammar (`"ARK/v1/<purpose>"`).
    ///
    /// # Errors
    ///
    /// Returns:
    /// - [`HkdfError::EmptyContext`] if `context` is empty.
    /// - [`HkdfError::ContextTooLong`] if `context.len() > MAX_CONTEXT_LEN`.
    /// - [`HkdfError::InvalidContextFormat`] if `context` does not start with
    ///   `"ARK/v1/"`, has an empty purpose suffix, or contains non-printable/whitespace
    ///   ASCII bytes.
    pub fn parse(context: &'a [u8]) -> Result<Self, HkdfError> {
        if context.is_empty() {
            return Err(HkdfError::EmptyContext);
        }
        if context.len() > MAX_CONTEXT_LEN {
            return Err(HkdfError::ContextTooLong);
        }
        if !context.starts_with(ARK_V1_CONTEXT_PREFIX) {
            return Err(HkdfError::InvalidContextFormat);
        }
        let suffix = &context[ARK_V1_CONTEXT_PREFIX.len()..];
        if suffix.is_empty() || !suffix.iter().all(|&b| (0x21..=0x7E).contains(&b)) {
            return Err(HkdfError::InvalidContextFormat);
        }
        Ok(Self { bytes: context })
    }

    /// Borrow the validated context byte slice.
    #[must_use]
    pub const fn as_bytes(&self) -> &'a [u8] {
        self.bytes
    }
}

/// Validated input key material (IKM) wrapper for HKDF-SHA-256 (`SEC-KDF-03`).
///
/// Enforces [`MIN_IKM_LEN`] (`16` bytes minimum) and redacts key material from
/// [`core::fmt::Debug`] output.
///
/// # Security Note
///
/// Input length validation is a supplementary defensive check and does not by
/// itself prove cryptographic entropy. Callers MUST supply high-entropy key
/// material (such as a 32-byte Argon2id-derived KEK or CSPRNG-generated root key)
/// and MUST NEVER pass a raw human password directly as HKDF IKM.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct HkdfIkm<'a> {
    bytes: &'a [u8],
}

impl<'a> HkdfIkm<'a> {
    /// Construct a validated [`HkdfIkm`] from an input key material slice.
    ///
    /// # Errors
    ///
    /// Returns [`HkdfError::EmptyInputKeyMaterial`] if `bytes.is_empty()`, or
    /// [`HkdfError::InputKeyMaterialTooShort`] if `bytes.len() < MIN_IKM_LEN`.
    pub const fn from_slice(bytes: &'a [u8]) -> Result<Self, HkdfError> {
        if bytes.is_empty() {
            return Err(HkdfError::EmptyInputKeyMaterial);
        }
        if bytes.len() < MIN_IKM_LEN {
            return Err(HkdfError::InputKeyMaterialTooShort);
        }
        Ok(Self { bytes })
    }

    /// Wrap a fixed 32-byte key array (`[u8; KEY_LEN]`) as validated HKDF IKM.
    #[must_use]
    pub const fn from_key_array(key: &'a [u8; KEY_LEN]) -> Self {
        Self { bytes: key }
    }

    /// Borrow the underlying IKM byte slice.
    #[must_use]
    pub const fn as_bytes(&self) -> &'a [u8] {
        self.bytes
    }
}

impl core::fmt::Debug for HkdfIkm<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("HkdfIkm")
            .field("len", &self.bytes.len())
            .finish_non_exhaustive()
    }
}

/// Derive a 256-bit subkey for a canonical [`HkdfKeyPurpose`] from validated [`HkdfIkm`].
///
/// # Errors
///
/// Returns [`HkdfError::ExpansionFailed`] only if the underlying HKDF expand step fails.
pub fn derive_purpose_subkey(
    ikm: HkdfIkm<'_>,
    salt: Option<&[u8]>,
    purpose: HkdfKeyPurpose,
) -> Result<Zeroizing<[u8; KEY_LEN]>, HkdfError> {
    let ctx = HkdfContext::from_purpose(purpose);
    derive_subkey(ikm.as_bytes(), salt, ctx.as_bytes())
}

/// Derive a 256-bit subkey from input key material using HKDF-SHA-256.
///
/// Enforces ARK's domain-separation contract (`context` must start with `"ARK/v1/"`
/// and satisfy [`HkdfContext::parse`]) and minimum IKM length ([`MIN_IKM_LEN`]).
///
/// # Errors
///
/// Returns [`HkdfError`] if `input_key_material` is empty or shorter than
/// [`MIN_IKM_LEN`], if `context` is empty, exceeds [`MAX_CONTEXT_LEN`], or does
/// not conform to the `"ARK/v1/<purpose>"` domain-separation format, or if
/// HKDF expansion fails.
pub fn derive_subkey(
    input_key_material: &[u8],
    salt: Option<&[u8]>,
    context: &[u8],
) -> Result<Zeroizing<[u8; KEY_LEN]>, HkdfError> {
    let validated_ikm = HkdfIkm::from_slice(input_key_material)?;
    let validated_context = HkdfContext::parse(context)?;

    let hkdf = Hkdf::<Sha256>::new(salt, validated_ikm.as_bytes());
    let mut output = Zeroizing::new([0_u8; KEY_LEN]);
    hkdf.expand(validated_context.as_bytes(), output.as_mut())
        .map_err(|_| HkdfError::ExpansionFailed)?;

    Ok(output)
}

/// Pair of zeroizing RFC 5869 `(PRK, OKM)` outputs returned by [`hkdf_sha256_rfc5869_expand`].
pub type HkdfRfc5869Output = (Zeroizing<[u8; KEY_LEN]>, Zeroizing<Vec<u8>>);

/// Perform raw RFC 5869 HKDF-SHA-256 Extract-and-Expand into a variable-length
/// zeroizing output buffer (`1..=8160` bytes) for official RFC 5869 conformance
/// and boundary testing (`[S-05]`).
///
/// Unlike [`derive_subkey`], this low-level conformance helper permits empty
/// `info` strings and arbitrary RFC 5869 test vector IKM lengths while strictly
/// enforcing RFC 5869 output length limits (`1..=255 * HashLen`).
///
/// # Errors
///
/// Returns [`HkdfError::EmptyInputKeyMaterial`] if `ikm` is empty,
/// [`HkdfError::InvalidOutputLength`] if `output_len` is `0` or exceeds
/// [`HKDF_SHA256_MAX_OUTPUT_LEN`] (`8160` bytes), or [`HkdfError::ExpansionFailed`]
/// if expansion fails.
pub fn hkdf_sha256_rfc5869_expand(
    ikm: &[u8],
    salt: Option<&[u8]>,
    info: &[u8],
    output_len: usize,
) -> Result<HkdfRfc5869Output, HkdfError> {
    if ikm.is_empty() {
        return Err(HkdfError::EmptyInputKeyMaterial);
    }
    if output_len == 0 || output_len > HKDF_SHA256_MAX_OUTPUT_LEN {
        return Err(HkdfError::InvalidOutputLength);
    }

    let (prk_bytes, hkdf) = Hkdf::<Sha256>::extract(salt, ikm);
    let mut prk = Zeroizing::new([0_u8; KEY_LEN]);
    prk.copy_from_slice(prk_bytes.as_slice());

    let mut okm = Zeroizing::new(vec![0_u8; output_len]);
    hkdf.expand(info, okm.as_mut_slice())
        .map_err(|_| HkdfError::ExpansionFailed)?;

    Ok((prk, okm))
}

/// Sanitized error classification for HKDF-SHA-256 subkey derivation.
///
/// Intentionally carries no key material, salt bytes, or context strings to
/// comply with `ARK-SEC-028`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HkdfError {
    /// The supplied input key material slice is empty.
    EmptyInputKeyMaterial,
    /// The supplied input key material slice is shorter than [`MIN_IKM_LEN`].
    InputKeyMaterialTooShort,
    /// The supplied HKDF context/info slice is empty.
    EmptyContext,
    /// The supplied HKDF context/info slice exceeds [`MAX_CONTEXT_LEN`].
    ContextTooLong,
    /// The supplied HKDF context/info slice does not match the canonical `"ARK/v1/<purpose>"` format.
    InvalidContextFormat,
    /// The requested HKDF output length is `0` or exceeds [`HKDF_SHA256_MAX_OUTPUT_LEN`] (`8160` bytes).
    InvalidOutputLength,
    /// The underlying HKDF expand operation failed.
    ExpansionFailed,
}

impl core::fmt::Display for HkdfError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let message = match self {
            Self::EmptyInputKeyMaterial => "input key material must not be empty",
            Self::InputKeyMaterialTooShort => {
                "input key material is shorter than the 16-byte minimum bound"
            }
            Self::EmptyContext => "HKDF context must not be empty",
            Self::ContextTooLong => "HKDF context exceeds the configured limit",
            Self::InvalidContextFormat => {
                "HKDF context must start with 'ARK/v1/' followed by printable ASCII purpose text"
            }
            Self::InvalidOutputLength => {
                "HKDF output length must be between 1 and 8160 bytes (255 * HashLen)"
            }
            Self::ExpansionFailed => "HKDF expansion failed",
        };
        f.write_str(message)
    }
}

impl std::error::Error for HkdfError {}
