use hkdf::Hkdf;
use sha2::Sha256;
use zeroize::Zeroizing;

use super::KEY_LEN;

/// Maximum accepted context length, to keep caller-controlled inputs bounded.
pub const MAX_CONTEXT_LEN: usize = 256;

/// Derive a 256-bit subkey from input key material using HKDF-SHA-256.
pub fn derive_subkey(
    input_key_material: &[u8],
    salt: Option<&[u8]>,
    context: &[u8],
) -> Result<Zeroizing<[u8; KEY_LEN]>, HkdfError> {
    if input_key_material.is_empty() {
        return Err(HkdfError::EmptyInputKeyMaterial);
    }
    if context.is_empty() {
        return Err(HkdfError::EmptyContext);
    }
    if context.len() > MAX_CONTEXT_LEN {
        return Err(HkdfError::ContextTooLong);
    }

    let hkdf = Hkdf::<Sha256>::new(salt, input_key_material);
    let mut output = Zeroizing::new([0_u8; KEY_LEN]);
    hkdf.expand(context, output.as_mut())
        .map_err(|_| HkdfError::ExpansionFailed)?;

    Ok(output)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HkdfError {
    EmptyInputKeyMaterial,
    EmptyContext,
    ContextTooLong,
    ExpansionFailed,
}

impl core::fmt::Display for HkdfError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let message = match self {
            Self::EmptyInputKeyMaterial => "input key material must not be empty",
            Self::EmptyContext => "HKDF context must not be empty",
            Self::ContextTooLong => "HKDF context exceeds the configured limit",
            Self::ExpansionFailed => "HKDF expansion failed",
        };
        f.write_str(message)
    }
}

impl std::error::Error for HkdfError {}