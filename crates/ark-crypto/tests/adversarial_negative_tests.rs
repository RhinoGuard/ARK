//! Adversarial negative tests for the current public KDF API.
//!
//! These tests assert fail-closed behavior and secret-safe error formatting.
//! They do not establish IKM entropy/provenance or prove the absence of
//! vulnerabilities outside the exercised inputs.

use ark_crypto::kdf::{
    Argon2Error, Argon2idConfig, HkdfContext, HkdfError, HkdfIkm, HkdfKeyPurpose, MAX_PASSWORD_LEN,
    MIN_IKM_LEN, MIN_MEMORY_KIB, derive_kek,
};

const SENTINEL_PASSWORD: &[u8] = b"CONFIDENTIAL_ARK_ROOT_PASSPHRASE_SENTINEL_#9999";
const SENTINEL_SALT: &[u8] = b"SECRET_SALT_123";
const SENTINEL_IKM: &[u8] = b"SECRET_IKM";
const VALID_TEST_SALT: &[u8] = b"0123456789abcdef";

#[test]
fn argon2_rejects_empty_and_oversized_passwords() {
    assert!(matches!(
        derive_kek(b"", SENTINEL_SALT, Argon2idConfig::default()),
        Err(Argon2Error::EmptyPassword)
    ));

    let oversized = vec![0x41_u8; MAX_PASSWORD_LEN + 1];
    assert!(matches!(
        derive_kek(&oversized, SENTINEL_SALT, Argon2idConfig::default()),
        Err(Argon2Error::PasswordTooLong)
    ));
}

#[test]
fn argon2_rejects_invalid_and_degenerate_salts() {
    assert!(matches!(
        derive_kek(SENTINEL_PASSWORD, b"", Argon2idConfig::default()),
        Err(Argon2Error::InvalidSaltLength)
    ));

    let short_salt = [0x55_u8; 15];
    assert!(matches!(
        derive_kek(SENTINEL_PASSWORD, &short_salt, Argon2idConfig::default()),
        Err(Argon2Error::InvalidSaltLength)
    ));

    let long_salt = [0x55_u8; 65];
    assert!(matches!(
        derive_kek(SENTINEL_PASSWORD, &long_salt, Argon2idConfig::default()),
        Err(Argon2Error::InvalidSaltLength)
    ));

    let zero_salt = [0_u8; 16];
    assert!(matches!(
        derive_kek(SENTINEL_PASSWORD, &zero_salt, Argon2idConfig::default()),
        Err(Argon2Error::DegenerateSalt)
    ));
}

#[test]
fn argon2_revalidates_publicly_constructible_cost_parameters() {
    let below_floor = Argon2idConfig {
        memory_kib: MIN_MEMORY_KIB - 1,
        iterations: 1,
        lanes: 1,
    };
    assert!(matches!(
        derive_kek(SENTINEL_PASSWORD, VALID_TEST_SALT, below_floor),
        Err(Argon2Error::InvalidParameters)
    ));

    let excessive_memory = Argon2idConfig {
        memory_kib: u32::MAX,
        iterations: 1,
        lanes: 1,
    };
    assert!(matches!(
        derive_kek(SENTINEL_PASSWORD, VALID_TEST_SALT, excessive_memory),
        Err(Argon2Error::InvalidParameters)
    ));

    let excessive_iterations = Argon2idConfig {
        memory_kib: MIN_MEMORY_KIB,
        iterations: u32::MAX,
        lanes: 1,
    };
    assert!(matches!(
        derive_kek(SENTINEL_PASSWORD, VALID_TEST_SALT, excessive_iterations),
        Err(Argon2Error::InvalidParameters)
    ));

    let excessive_lanes = Argon2idConfig {
        memory_kib: MIN_MEMORY_KIB,
        iterations: 1,
        lanes: u32::MAX,
    };
    assert!(matches!(
        derive_kek(SENTINEL_PASSWORD, VALID_TEST_SALT, excessive_lanes),
        Err(Argon2Error::InvalidParameters)
    ));
}

#[test]
fn argon2_error_formatting_does_not_disclose_password_or_salt() {
    // Pass both sentinels into the API and force an early validation error.
    let result = derive_kek(SENTINEL_PASSWORD, SENTINEL_SALT, Argon2idConfig::default());
    assert!(result.is_err(), "short salt unexpectedly derived a key");
    let Err(error) = result else {
        return;
    };

    let display = format!("{error}");
    let debug = format!("{error:?}");
    let password_sentinel = String::from_utf8_lossy(SENTINEL_PASSWORD);
    let salt_sentinel = String::from_utf8_lossy(SENTINEL_SALT);

    assert!(!display.contains(password_sentinel.as_ref()));
    assert!(!debug.contains(password_sentinel.as_ref()));
    assert!(!display.contains(salt_sentinel.as_ref()));
    assert!(!debug.contains(salt_sentinel.as_ref()));
}

#[test]
fn hkdf_rejects_empty_and_short_ikm() {
    assert!(matches!(
        HkdfIkm::from_slice(b""),
        Err(HkdfError::EmptyInputKeyMaterial)
    ));

    let short = [0x42_u8; MIN_IKM_LEN - 1];
    assert!(matches!(
        HkdfIkm::from_slice(&short),
        Err(HkdfError::InputKeyMaterialTooShort)
    ));
}

#[test]
fn hkdf_context_parser_rejects_unknown_and_malformed_purposes() {
    assert!(matches!(
        HkdfContext::parse(b""),
        Err(HkdfError::EmptyContext)
    ));
    assert!(matches!(
        HkdfContext::parse(b"project-key"),
        Err(HkdfError::InvalidContextFormat)
    ));
    assert!(matches!(
        HkdfContext::parse(b"ARK/v1/unregistered-purpose"),
        Err(HkdfError::InvalidContextFormat)
    ));
}

#[test]
fn hkdf_registered_purposes_have_distinct_context_labels() {
    assert_ne!(
        HkdfKeyPurpose::ProjectKey.as_context_bytes(),
        HkdfKeyPurpose::FileKey.as_context_bytes()
    );
}

#[test]
fn hkdf_error_formatting_does_not_disclose_short_ikm() {
    let result = HkdfIkm::from_slice(SENTINEL_IKM);
    assert!(
        result.is_err(),
        "short sentinel IKM unexpectedly passed the length check"
    );
    let Err(error) = result else {
        return;
    };

    let display = format!("{error}");
    let debug = format!("{error:?}");
    let sentinel = String::from_utf8_lossy(SENTINEL_IKM);
    assert!(!display.contains(sentinel.as_ref()));
    assert!(!debug.contains(sentinel.as_ref()));
}

#[test]
fn argon2_salt_debug_redacts_sentinel_bytes() {
    let sentinel = b"ARGON2_SALT_SENTINEL_2026";
    let salt = match ark_crypto::kdf::Argon2Salt::from_slice(sentinel) {
        Ok(salt) => salt,
        Err(error) => panic!("valid test salt was rejected: {error}"),
    };

    let debug = format!("{salt:?}");
    assert!(debug.contains("Argon2Salt"));
    assert!(debug.contains(&format!("len: {}", sentinel.len())));
    assert!(!debug.contains(String::from_utf8_lossy(sentinel).as_ref()));
}

#[test]
fn hkdf_ikm_debug_redacts_sentinel_bytes() {
    let sentinel = [b'K'; 32];
    let ikm = HkdfIkm::from_key_array(&sentinel);

    let debug = format!("{ikm:?}");
    assert!(debug.contains("HkdfIkm"));
    assert!(debug.contains("len: 32"));
    assert!(!debug.contains(String::from_utf8_lossy(&sentinel).as_ref()));
}

#[test]
fn hkdf_parser_accepts_exactly_all_registered_contexts() {
    let purposes = [
        HkdfKeyPurpose::WorkspaceKey,
        HkdfKeyPurpose::ProjectKey,
        HkdfKeyPurpose::FileKey,
        HkdfKeyPurpose::MetadataKey,
        HkdfKeyPurpose::AuditKey,
        HkdfKeyPurpose::RecoveryKey,
    ];

    for purpose in purposes {
        let canonical = purpose.as_context_bytes();
        let parsed = match HkdfContext::parse(canonical) {
            Ok(parsed) => parsed,
            Err(error) => panic!("registered HKDF purpose was rejected: {error}"),
        };
        assert_eq!(parsed.as_bytes(), canonical);

        let mut mutated = canonical.to_vec();
        let last = mutated.len() - 1;
        mutated[last] ^= 0x01;
        assert!(matches!(
            HkdfContext::parse(&mutated),
            Err(HkdfError::InvalidContextFormat)
        ));
    }
}

#[test]
fn hkdf_parser_rejects_malformed_and_oversized_contexts() {
    assert!(matches!(
        HkdfContext::parse(b"ARK/v1/\xff"),
        Err(HkdfError::InvalidContextFormat)
    ));
    assert!(matches!(
        HkdfContext::parse(b"ARK/v"),
        Err(HkdfError::InvalidContextFormat)
    ));

    let mut oversized = b"ARK/v1/".to_vec();
    oversized.resize(257, b'x');
    assert!(matches!(
        HkdfContext::parse(&oversized),
        Err(HkdfError::ContextTooLong)
    ));
}
