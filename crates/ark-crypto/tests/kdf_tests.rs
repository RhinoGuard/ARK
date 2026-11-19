//! Integration tests for the public ARK KDF API.
//!
//! The suite intentionally uses only registered HKDF purposes. Raw context-based
//! derivation remains an internal implementation detail.

use ark_crypto::kdf::{
    Argon2Error, Argon2Salt, Argon2idConfig, DEFAULT_ITERATIONS, DEFAULT_LANES,
    DEFAULT_MEMORY_KIB, HkdfContext, HkdfError, HkdfIkm, HkdfKeyPurpose,
    MAX_CONTEXT_LEN, MAX_ITERATIONS, MAX_LANES, MAX_MEMORY_KIB, MAX_PASSWORD_LEN,
    MAX_SALT_LEN, MIN_IKM_LEN, MIN_MEMORY_KIB, MIN_SALT_LEN, derive_kek,
    derive_kek_with_salt, derive_purpose_subkey,
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
    assert!(matches!(
        derive_kek(b"password", b"short", Argon2idConfig::default()),
        Err(Argon2Error::InvalidSaltLength)
    ));

    let oversized_salt = [0xAA_u8; MAX_SALT_LEN + 1];
    assert!(matches!(
        derive_kek(b"password", &oversized_salt, Argon2idConfig::default()),
        Err(Argon2Error::InvalidSaltLength)
    ));

    let all_zero_salt = [0_u8; MIN_SALT_LEN];
    assert!(matches!(
        Argon2Salt::from_slice(&all_zero_salt),
        Err(Argon2Error::DegenerateSalt)
    ));
    assert!(matches!(
        derive_kek(b"password", &all_zero_salt, Argon2idConfig::default()),
        Err(Argon2Error::DegenerateSalt)
    ));

    let valid_salt = Argon2Salt::from_slice(b"0123456789abcdef")?;
    let config = Argon2idConfig::new(MIN_MEMORY_KIB, 1, 1)?;
    let key = derive_kek_with_salt(b"password", &valid_salt, config)?;
    assert_eq!(key.len(), 32);
    Ok(())
}

#[test]
fn argon2id_enforces_password_length_bounds() {
    assert!(matches!(
        derive_kek(b"", b"0123456789abcdef", Argon2idConfig::default()),
        Err(Argon2Error::EmptyPassword)
    ));

    let oversized_password = [0x41_u8; MAX_PASSWORD_LEN + 1];
    assert!(matches!(
        derive_kek(
            &oversized_password,
            b"0123456789abcdef",
            Argon2idConfig::default()
        ),
        Err(Argon2Error::PasswordTooLong)
    ));
}

#[test]
fn argon2id_enforces_parameter_policy_bounds() {
    let default_cfg = Argon2idConfig::default();
    assert_eq!(default_cfg.memory_kib(), DEFAULT_MEMORY_KIB);
    assert_eq!(default_cfg.iterations(), DEFAULT_ITERATIONS);
    assert_eq!(default_cfg.lanes(), DEFAULT_LANES);
    assert_eq!(default_cfg.validate(), Ok(()));

    assert!(matches!(
        Argon2idConfig::new(MIN_MEMORY_KIB - 1, 1, 1),
        Err(Argon2Error::InvalidParameters)
    ));
    assert!(matches!(
        Argon2idConfig::new(MAX_MEMORY_KIB + 1, 1, 1),
        Err(Argon2Error::InvalidParameters)
    ));

    // Direct public-field construction must still be rejected at the KDF boundary.
    let unbounded_config = Argon2idConfig {
        memory_kib: u32::MAX,
        iterations: 3,
        lanes: 1,
    };
    assert!(matches!(
        derive_kek(b"password", b"0123456789abcdef", unbounded_config),
        Err(Argon2Error::InvalidParameters)
    ));

    assert!(matches!(
        Argon2idConfig::new(MIN_MEMORY_KIB, 0, 1),
        Err(Argon2Error::InvalidParameters)
    ));
    assert!(matches!(
        Argon2idConfig::new(MIN_MEMORY_KIB, MAX_ITERATIONS + 1, 1),
        Err(Argon2Error::InvalidParameters)
    ));
    assert!(matches!(
        Argon2idConfig::new(MIN_MEMORY_KIB, 1, 0),
        Err(Argon2Error::InvalidParameters)
    ));
    assert!(matches!(
        Argon2idConfig::new(MIN_MEMORY_KIB, 1, MAX_LANES + 1),
        Err(Argon2Error::InvalidParameters)
    ));
}

#[test]
fn hkdf_is_deterministic_for_same_inputs() -> Result<(), HkdfError> {
    let ikm_bytes = [0x5A_u8; 32];
    let ikm = HkdfIkm::from_key_array(&ikm_bytes);
    let first = derive_purpose_subkey(ikm, None, HkdfKeyPurpose::ProjectKey)?;
    let second = derive_purpose_subkey(ikm, None, HkdfKeyPurpose::ProjectKey)?;
    assert_eq!(&*first, &*second);
    Ok(())
}

#[test]
fn hkdf_purposes_use_distinct_registered_context_labels() {
    let purposes = [
        HkdfKeyPurpose::WorkspaceKey,
        HkdfKeyPurpose::ProjectKey,
        HkdfKeyPurpose::FileKey,
        HkdfKeyPurpose::MetadataKey,
        HkdfKeyPurpose::AuditKey,
        HkdfKeyPurpose::RecoveryKey,
    ];
    for (index, first_purpose) in purposes.iter().enumerate() {
        for second_purpose in &purposes[index + 1..] {
            assert_ne!(
                first_purpose.as_context_bytes(),
                second_purpose.as_context_bytes()
            );
        }
    }
}

#[test]
fn hkdf_enforces_ikm_and_registered_context_contract() {
    assert!(matches!(
        HkdfIkm::from_slice(b""),
        Err(HkdfError::EmptyInputKeyMaterial)
    ));
    let short_ikm = [0x11_u8; MIN_IKM_LEN - 1];
    assert!(matches!(
        HkdfIkm::from_slice(&short_ikm),
        Err(HkdfError::InputKeyMaterialTooShort)
    ));

    assert!(matches!(
        HkdfContext::parse(b""),
        Err(HkdfError::EmptyContext)
    ));
    let oversized_context = [b'x'; MAX_CONTEXT_LEN + 1];
    assert!(matches!(
        HkdfContext::parse(&oversized_context),
        Err(HkdfError::ContextTooLong)
    ));
    assert!(matches!(
        HkdfContext::parse(b"project-key"),
        Err(HkdfError::InvalidContextFormat)
    ));
    assert!(matches!(
        HkdfContext::parse(b"ARK/v1/unregistered-purpose"),
        Err(HkdfError::InvalidContextFormat)
    ));
    assert!(HkdfContext::parse(b"ARK/v1/project-key").is_ok());
}
