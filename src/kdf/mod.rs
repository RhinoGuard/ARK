//! Key-derivation functions used by ARK.

mod argon2id;
mod hkdf_sha256;

pub use argon2id::{Argon2Error, Argon2idConfig, derive_kek};
pub use hkdf_sha256::{derive_subkey, HkdfError, MAX_CONTEXT_LEN};

/// Length of ARK's initial symmetric key material, in bytes.
pub const KEY_LEN: usize = 32;