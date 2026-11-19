//! Key-derivation functions used by ARK.

mod argon2id;
mod hkdf_sha256;

pub use argon2id::{
    Argon2Error, Argon2Salt, Argon2idConfig, DEFAULT_ITERATIONS, DEFAULT_LANES, DEFAULT_MEMORY_KIB,
    DEFAULT_SALT_LEN, MAX_ITERATIONS, MAX_LANES, MAX_MEMORY_KIB, MAX_PASSWORD_LEN, MAX_SALT_LEN,
    MIN_ITERATIONS, MIN_LANES, MIN_MEMORY_KIB, MIN_SALT_LEN, derive_argon2id_rfc9106_kat,
    derive_kek, derive_kek_with_salt,
};
pub use hkdf_sha256::{HkdfError, MAX_CONTEXT_LEN, derive_subkey};

/// Length of ARK's initial symmetric key material, in bytes.
pub const KEY_LEN: usize = 32;
