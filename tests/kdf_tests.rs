use ark_crypto::kdf::{
    Argon2idConfig, HkdfError, derive_kek, derive_subkey,
};

#[test]
fn argon2id_is_deterministic_for_same_inputs() {
    let config = Argon2idConfig {
        memory_kib: 8 * 1024,
        iterations: 1,
        lanes: 1,
    };
    let first = derive_kek(b"test-password", b"0123456789abcdef", config).unwrap();
    let second = derive_kek(b"test-password", b"0123456789abcdef", config).unwrap();
    assert_eq!(&*first, &*second);
}

#[test]
fn argon2id_changes_when_salt_changes() {
    let config = Argon2idConfig {
        memory_kib: 8 * 1024,
        iterations: 1,
        lanes: 1,
    };
    let first = derive_kek(b"test-password", b"0123456789abcdef", config).unwrap();
    let second = derive_kek(b"test-password", b"fedcba9876543210", config).unwrap();
    assert_ne!(&*first, &*second);
}

#[test]
fn argon2id_rejects_invalid_salt() {
    let result = derive_kek(b"password", b"short", Argon2idConfig::default());
    assert!(result.is_err());
}

#[test]
fn hkdf_is_deterministic_for_same_inputs() {
    let first = derive_subkey(b"workspace-master-key", None, b"ARK/v1/project-key").unwrap();
    let second = derive_subkey(b"workspace-master-key", None, b"ARK/v1/project-key").unwrap();
    assert_eq!(&*first, &*second);
}

#[test]
fn hkdf_separates_distinct_contexts() {
    let project = derive_subkey(b"workspace-master-key", None, b"ARK/v1/project-key").unwrap();
    let file = derive_subkey(b"workspace-master-key", None, b"ARK/v1/file-key").unwrap();
    assert_ne!(&*project, &*file);
}

#[test]
fn hkdf_rejects_empty_context() {
    let result = derive_subkey(b"some-key-material", None, b"");
    assert_eq!(result.unwrap_err(), HkdfError::EmptyContext);
}