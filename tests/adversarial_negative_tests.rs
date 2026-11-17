//! Adversarial and negative security tests for the KDF and Core foundation.
//!
//! Evaluates security-critical boundaries under hostile and malformed inputs:
//! - Sentinel secret non-leakage in error representations (ARK-SEC-028).
//! - Strict input bound enforcement (password length, salt length, context length).
//! - Fail-safe rejection of empty inputs without panic.
//! - Parameter boundary validation for Argon2id.

use ark_crypto::kdf::{
    derive_kek, derive_subkey, Argon2Error, Argon2idConfig, HkdfError, MAX_CONTEXT_LEN,
};

const SENTINEL_SECRET: &[u8] = b"CONFIDENTIAL_ARK_ROOT_PASSPHRASE_SENTINEL_#9999";
const SENTINEL_SALT: &[u8] = b"UNTRUSTED_ATTACKER_CONTROLLED_SALT_123456";

#[test]
fn test_adversarial_argon2_empty_password_rejection() {
    // SECURITY PROPERTY: Zero-length passwords must be rejected fail-closed.
    let result = derive_kek(b"", SENTINEL_SALT, Argon2idConfig::default());
    assert!(matches!(result, Err(Argon2Error::EmptyPassword)));
}

#[test]
fn test_adversarial_argon2_oversized_password_rejection() {
    // SECURITY PROPERTY: Passwords exceeding 1024 bytes must be rejected fail-closed
    // to prevent memory/CPU exhaustion.
    let oversized = vec![0x41u8; 1025];
    let result = derive_kek(&oversized, SENTINEL_SALT, Argon2idConfig::default());
    assert!(matches!(result, Err(Argon2Error::PasswordTooLong)));
}

#[test]
fn test_adversarial_argon2_invalid_salt_lengths() {
    // SECURITY PROPERTY: Salts outside 16..=64 bytes must be rejected fail-closed.
    // Under-length salt (15 bytes)
    let short_salt = [0x55u8; 15];
    let res_short = derive_kek(SENTINEL_SECRET, &short_salt, Argon2idConfig::default());
    assert!(matches!(res_short, Err(Argon2Error::InvalidSaltLength)));

    // Empty salt (0 bytes)
    let empty_salt = [];
    let res_empty = derive_kek(SENTINEL_SECRET, &empty_salt, Argon2idConfig::default());
    assert!(matches!(res_empty, Err(Argon2Error::InvalidSaltLength)));

    // Over-length salt (65 bytes)
    let long_salt = [0x55u8; 65];
    let res_long = derive_kek(SENTINEL_SECRET, &long_salt, Argon2idConfig::default());
    assert!(matches!(res_long, Err(Argon2Error::InvalidSaltLength)));
}

#[test]
fn test_adversarial_argon2_invalid_parameter_handling() {
    // SECURITY PROPERTY: Absurd parameter combinations must return a typed error,
    // never panic or allocate unbounded memory.
    let zero_memory_config = Argon2idConfig {
        memory_kib: 0,
        iterations: 1,
        lanes: 1,
    };
    let res = derive_kek(SENTINEL_SECRET, SENTINEL_SALT, zero_memory_config);
    assert!(matches!(res, Err(Argon2Error::InvalidParameters)));

    let zero_lanes_config = Argon2idConfig {
        memory_kib: 1024,
        iterations: 1,
        lanes: 0,
    };
    let res_lanes = derive_kek(SENTINEL_SECRET, SENTINEL_SALT, zero_lanes_config);
    assert!(matches!(res_lanes, Err(Argon2Error::InvalidParameters)));
}

#[test]
fn test_adversarial_sentinel_secret_leakage_in_argon2_errors() {
    // SECURITY PROPERTY (ARK-SEC-028): Error objects, Display, and Debug formatters
    // must never contain or reflect the sensitive input password or salt.
    let oversized = [SENTINEL_SECRET, &[0x42u8; 1000]].concat();
    let err = derive_kek(&oversized, SENTINEL_SALT, Argon2idConfig::default())
        .expect_err("Must fail due to oversized password");

    let display_output = format!("{err}");
    let debug_output = format!("{err:?}");

    let sentinel_str = std::str::from_utf8(SENTINEL_SECRET).unwrap();
    let salt_str = std::str::from_utf8(SENTINEL_SALT).unwrap();

    assert!(
        !display_output.contains(sentinel_str),
        "Sentinel password leaked in Argon2Error Display representation!"
    );
    assert!(
        !debug_output.contains(sentinel_str),
        "Sentinel password leaked in Argon2Error Debug representation!"
    );
    assert!(
        !display_output.contains(salt_str),
        "Salt leaked in Argon2Error Display representation!"
    );
    assert!(
        !debug_output.contains(salt_str),
        "Salt leaked in Argon2Error Debug representation!"
    );
}

#[test]
fn test_adversarial_hkdf_empty_ikm_rejection() {
    // SECURITY PROPERTY: Zero-length IKM must be rejected fail-closed.
    let result = derive_subkey(b"", None, b"ARK/v1/test-context");
    assert!(matches!(result, Err(HkdfError::EmptyInputKeyMaterial)));
}

#[test]
fn test_adversarial_hkdf_empty_and_oversized_context() {
    // SECURITY PROPERTY: Empty context and contexts > MAX_CONTEXT_LEN must be rejected.
    let res_empty = derive_subkey(SENTINEL_SECRET, None, b"");
    assert!(matches!(res_empty, Err(HkdfError::EmptyContext)));

    let oversized_context = vec![0x61u8; MAX_CONTEXT_LEN + 1];
    let res_oversized = derive_subkey(SENTINEL_SECRET, None, &oversized_context);
    assert!(matches!(res_oversized, Err(HkdfError::ContextTooLong)));
}

#[test]
fn test_adversarial_sentinel_secret_leakage_in_hkdf_errors() {
    // SECURITY PROPERTY (ARK-SEC-028): HKDF error representations must never expose IKM or context.
    let oversized_context = vec![0x61u8; MAX_CONTEXT_LEN + 1];
    let err = derive_subkey(SENTINEL_SECRET, None, &oversized_context)
        .expect_err("Must fail due to oversized context");

    let display_output = format!("{err}");
    let debug_output = format!("{err:?}");
    let sentinel_str = std::str::from_utf8(SENTINEL_SECRET).unwrap();

    assert!(
        !display_output.contains(sentinel_str),
        "Sentinel secret leaked in HkdfError Display representation!"
    );
    assert!(
        !debug_output.contains(sentinel_str),
        "Sentinel secret leaked in HkdfError Debug representation!"
    );
}
