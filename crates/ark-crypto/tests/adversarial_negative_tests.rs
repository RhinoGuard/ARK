//! Adversarial negative tests for the current public KDF API.
//!
//! These tests assert fail-closed behavior and secret-safe error formatting.
//! They do not establish IKM entropy/provenance or prove the absence of
//! vulnerabilities outside the exercised inputs.

use ark_crypto::kdf::{
    Argon2Error, Argon2idConfig, HkdfContext, HkdfError, HkdfIkm, HkdfKeyPurpose,
    MAX_PASSWORD_LEN, MIN_IKM_LEN, MIN_MEMORY_KIB, derive_kek, derive_purpose_subkey,
};

const SENTINEL_PASSWORD: &[u8] = b"CONFIDENTIAL_ARK_ROOT_PASSPHRASE_SENTINEL_#9999";
const SENTINEL_SALT: &[u8] = b"UNTRUSTED_ATTACKER_CONTROLLED_SALT_123456";
const SENTINEL_IKM: &[u8] = b"CONFIDENTIAL_ARK_IKM_SENTINEL";

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
        derive_kek(SENTINEL_PASSWORD, SENTINEL_SALT, below_floor),
        Err(Argon2Error::InvalidParameters)
    ));

    let excessive_memory = Argon2idConfig {
        memory_kib: u32::MAX,
        iterations: 1,
        lanes: 1,
    };
    assert!(matches!(
        derive_kek(SENTINEL_PASSWORD, SENTINEL_SALT, excessive_memory),
        Err(Argon2Error::InvalidParameters)
    ));

    let excessive_iterations = Argon2idConfig {
        memory_kib: MIN_MEMORY_KIB,
        iterations: u32::MAX,
        lanes: 1,
    };
    assert!(matches!(
        derive_kek(SENTINEL_PASSWORD, SENTINEL_SALT, excessive_iterations),
        Err(Argon2Error::InvalidParameters)
    ));

    let excessive_lanes = Argon2idConfig {
        memory_kib: MIN_MEMORY_KIB,
        iterations: 1,
        lanes: u32::MAX,
    };
    assert!(matches!(
        derive_kek(SENTINEL_PASSWORD, SENTINEL_SALT, excessive_lanes),
        Err(Argon2Error::InvalidParameters)
    ));
}

#[test]
fn argon2_error_formatting_does_not_disclose_password_or_salt() {
    let oversized = vec![0x41_u8; MAX_PASSWORD_LEN + 1];
    let Err(error) = derive_kek(&oversized, SENTINEL_SALT, Argon2idConfig::default()) else {
        assert!(false, "oversized password unexpectedly derived a key");
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
fn hkdf_public_api_derives_distinct_registered_purposes() {
    let ikm_bytes = [0x3C_u8; 32];
    let Ok(ikm) = HkdfIkm::from_slice(&ikm_bytes) else {
        assert!(false, "32-byte IKM should satisfy the shape constraint");
        return;
    };

    let project = derive_purpose_subkey(ikm, None, HkdfKeyPurpose::ProjectKey);
    let file = derive_purpose_subkey(ikm, None, HkdfKeyPurpose::FileKey);
    assert!(project.is_ok());
    assert!(file.is_ok());

    if let (Ok(project_key), Ok(file_key)) = (project, file) {
        assert_ne!(&*project_key, &*file_key);
    }
}

#[test]
fn hkdf_error_formatting_does_not_disclose_short_ikm() {
    let Err(error) = HkdfIkm::from_slice(SENTINEL_IKM) else {
        assert!(false, "short sentinel IKM unexpectedly passed the length check");
        return;
    };

    let display = format!("{error}");
    let debug = format!("{error:?}");
    let sentinel = String::from_utf8_lossy(SENTINEL_IKM);
    assert!(!display.contains(sentinel.as_ref()));
    assert!(!debug.contains(sentinel.as_ref()));
}
