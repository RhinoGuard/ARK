//! Key-derivation functions used by ARK.

mod argon2id;
mod hkdf_sha256;

pub use argon2id::{
    Argon2Error, Argon2Salt, Argon2idConfig, DEFAULT_ITERATIONS, DEFAULT_LANES, DEFAULT_MEMORY_KIB,
    DEFAULT_SALT_LEN, MAX_ITERATIONS, MAX_LANES, MAX_MEMORY_KIB, MAX_PASSWORD_LEN, MAX_SALT_LEN,
    MIN_ITERATIONS, MIN_LANES, MIN_MEMORY_KIB, MIN_SALT_LEN, derive_argon2id_rfc9106_kat,
    derive_kek, derive_kek_with_salt,
};
pub use hkdf_sha256::{
    ARK_V1_CONTEXT_PREFIX, HKDF_SHA256_MAX_OUTPUT_LEN, HkdfContext, HkdfError, HkdfIkm,
    HkdfKeyPurpose, HkdfRfc5869Output, MAX_CONTEXT_LEN, MIN_IKM_LEN, RECOMMENDED_IKM_LEN,
    derive_purpose_subkey, derive_subkey, hkdf_sha256_rfc5869_expand,
};

/// Length of ARK's initial symmetric key material, in bytes.
pub const KEY_LEN: usize = 32;
