//! Integration tests for ARK cryptographic key-derivation functions.

use ark_crypto::kdf::{
    Argon2Error, Argon2Salt, Argon2idConfig, DEFAULT_ITERATIONS, DEFAULT_LANES, DEFAULT_MEMORY_KIB,
    HkdfError, MAX_ITERATIONS, MAX_LANES, MAX_MEMORY_KIB, MAX_PASSWORD_LEN, MAX_SALT_LEN,
    MIN_MEMORY_KIB, MIN_SALT_LEN, derive_kek, derive_kek_with_salt, derive_subkey,
};

#[test]
fn argon2id_is_deterministic_for_same_inputs() -> Result<(), Argon2Error> {
    let config = Argon2idConfig::new(MIN_MEMORY_KIB, 1, 1)?;
    let first = derive_kek(b"test-password", b"0123456789abcdef", config)?;
    let second = derive_kek(b"test-password", b"0123456789abcdef", config)?;
    assert_eq!(&*first, &*second);
    Ok(())
}

#[test]
fn argon2id_changes_when_salt_changes() -> Result<(), Argon2Error> {
    let config = Argon2idConfig::new(MIN_MEMORY_KIB, 1, 1)?;
    let first = derive_kek(b"test-password", b"0123456789abcdef", config)?;
    let second = derive_kek(b"test-password", b"fedcba9876543210", config)?;
    assert_ne!(&*first, &*second);
    Ok(())
}

#[test]
fn argon2id_validates_salt_contract_and_rejects_degenerate_salt() -> Result<(), Argon2Error> {
    assert_eq!(
        derive_kek(b"password", b"short", Argon2idConfig::default()),
        Err(Argon2Error::InvalidSaltLength)
    );

    let oversized_salt = [0xAA_u8; MAX_SALT_LEN + 1];
    assert_eq!(
        derive_kek(b"password", &oversized_salt, Argon2idConfig::default()),
        Err(Argon2Error::InvalidSaltLength)
    );

    let all_zero_salt = [0_u8; MIN_SALT_LEN];
    assert_eq!(
        Argon2Salt::from_slice(&all_zero_salt),
        Err(Argon2Error::DegenerateSalt)
    );
    assert_eq!(
        derive_kek(b"password", &all_zero_salt, Argon2idConfig::default()),
        Err(Argon2Error::DegenerateSalt)
    );

    let valid_salt = Argon2Salt::from_slice(b"0123456789abcdef")?;
    let config = Argon2idConfig::new(MIN_MEMORY_KIB, 1, 1)?;
    let key = derive_kek_with_salt(b"password", &valid_salt, config)?;
    assert_eq!(key.len(), 32);
    Ok(())
}

#[test]
fn argon2id_enforces_password_length_bounds() {
    assert_eq!(
        derive_kek(b"", b"0123456789abcdef", Argon2idConfig::default()),
        Err(Argon2Error::EmptyPassword)
    );

    let oversized_password = [0x41_u8; MAX_PASSWORD_LEN + 1];
    assert_eq!(
        derive_kek(
            &oversized_password,
            b"0123456789abcdef",
            Argon2idConfig::default()
        ),
        Err(Argon2Error::PasswordTooLong)
    );
}

#[test]
fn argon2id_enforces_parameter_policy_bounds() {
    let default_cfg = Argon2idConfig::default();
    assert_eq!(default_cfg.memory_kib(), DEFAULT_MEMORY_KIB);
    assert_eq!(default_cfg.iterations(), DEFAULT_ITERATIONS);
    assert_eq!(default_cfg.lanes(), DEFAULT_LANES);
    assert_eq!(default_cfg.validate(), Ok(()));

    // Below minimum memory floor (e.g., 8 MiB < 16 MiB)
    assert_eq!(
        Argon2idConfig::new(MIN_MEMORY_KIB - 1, 1, 1),
        Err(Argon2Error::InvalidParameters)
    );
    // Above maximum memory ceiling (DoS prevention)
    assert_eq!(
        Argon2idConfig::new(MAX_MEMORY_KIB + 1, 1, 1),
        Err(Argon2Error::InvalidParameters)
    );
    // Direct struct construction bypassing Argon2idConfig::new must also be caught by derive_kek
    let unbounded_dos_cfg = Argon2idConfig {
        memory_kib: 4 * 1024 * 1024,
        iterations: 3,
        lanes: 1,
    };
    assert_eq!(
        derive_kek(b"password", b"0123456789abcdef", unbounded_dos_cfg),
        Err(Argon2Error::InvalidParameters)
    );

    // Iterations below/above bounds
    assert_eq!(
        Argon2idConfig::new(MIN_MEMORY_KIB, 0, 1),
        Err(Argon2Error::InvalidParameters)
    );
    assert_eq!(
        Argon2idConfig::new(MIN_MEMORY_KIB, MAX_ITERATIONS + 1, 1),
        Err(Argon2Error::InvalidParameters)
    );

    // Lanes below/above bounds
    assert_eq!(
        Argon2idConfig::new(MIN_MEMORY_KIB, 1, 0),
        Err(Argon2Error::InvalidParameters)
    );
    assert_eq!(
        Argon2idConfig::new(MIN_MEMORY_KIB, 1, MAX_LANES + 1),
        Err(Argon2Error::InvalidParameters)
    );
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
