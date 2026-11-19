//! Integration tests for ARK cryptographic key-derivation functions.

use ark_crypto::kdf::{Argon2Error, Argon2idConfig, HkdfError, derive_kek, derive_subkey};

#[test]
fn argon2id_is_deterministic_for_same_inputs() -> Result<(), Argon2Error> {
    let config = Argon2idConfig {
        memory_kib: 8 * 1024,
        iterations: 1,
        lanes: 1,
    };
    let first = derive_kek(b"test-password", b"0123456789abcdef", config)?;
    let second = derive_kek(b"test-password", b"0123456789abcdef", config)?;
    assert_eq!(&*first, &*second);
    Ok(())
}

#[test]
fn argon2id_changes_when_salt_changes() -> Result<(), Argon2Error> {
    let config = Argon2idConfig {
        memory_kib: 8 * 1024,
        iterations: 1,
        lanes: 1,
    };
    let first = derive_kek(b"test-password", b"0123456789abcdef", config)?;
    let second = derive_kek(b"test-password", b"fedcba9876543210", config)?;
    assert_ne!(&*first, &*second);
    Ok(())
}

#[test]
fn argon2id_rejects_invalid_salt() {
    let result = derive_kek(b"password", b"short", Argon2idConfig::default());
    assert_eq!(result, Err(Argon2Error::InvalidSaltLength));
}

#[test]
fn hkdf_is_deterministic_for_same_inputs() -> Result<(), HkdfError> {
    let first = derive_subkey(b"workspace-master-key", None, b"ARK/v1/project-key")?;
    let second = derive_subkey(b"workspace-master-key", None, b"ARK/v1/project-key")?;
    assert_eq!(&*first, &*second);
    Ok(())
}

#[test]
fn hkdf_separates_distinct_contexts() -> Result<(), HkdfError> {
    let project = derive_subkey(b"workspace-master-key", None, b"ARK/v1/project-key")?;
    let file = derive_subkey(b"workspace-master-key", None, b"ARK/v1/file-key")?;
    assert_ne!(&*project, &*file);
    Ok(())
}

#[test]
fn hkdf_rejects_empty_context() {
    let result = derive_subkey(b"some-key-material", None, b"");
    assert_eq!(result, Err(HkdfError::EmptyContext));
}
