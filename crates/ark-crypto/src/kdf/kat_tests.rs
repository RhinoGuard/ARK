//! Official Known-Answer Test (KAT) and regression vectors for ARK KDFs (`[S-05]`).
//!
//! # Authoritative Specifications and Vector Identifiers
//!
//! 1. **RFC 9106 (Argon2 Memory-Hard Function for Password Hashing and Proof-of-Work):**
//!    - **Section 5.3:** Argon2id v1.3 (`Version::V0x13`) Test Vector (`m = 32 KiB`, `t = 3`,
//!      `p = 4`, 32-byte password `0x01..`, 16-byte salt `0x02..`, 8-byte secret `0x03..`,
//!      12-byte associated data `0x04..`, 32-byte tag).
//! 2. **RFC 5869 (HMAC-based Extract-and-Expand Key Derivation Function - HKDF):**
//!    - **Appendix A.1 (Test Case 1):** Basic test case with SHA-256 (`IKM = 22` octets,
//!      `salt = 13` octets, `info = 10` octets, `L = 42` octets).
//!    - **Appendix A.2 (Test Case 2):** Test with SHA-256 and longer inputs/outputs
//!      (`IKM = 80` octets, `salt = 80` octets, `info = 80` octets, `L = 82` octets).
//!    - **Appendix A.3 (Test Case 3):** Test with SHA-256 and zero-length `salt`/`info`
//!      (`IKM = 22` octets, `salt = None`, `info = empty`, `L = 42` octets).
//! 3. **Independent Cross-Implementation Conformance Vector for `derive_subkey`:**
//!    - Independently computed via Python 3.14 standard library `hmac`/`hashlib.sha256`
//!      per RFC 5869 Section 2.2/2.3 for canonical context `"ARK/v1/project-key"`.
//!
//! # Secret-Safe Failure Assertions (`ARK-SEC-028`)
//!
//! All assertions compare results using a secret-safe boolean check (`slices_equal_redacted`)
//! that never prints raw password bytes, salts, PRKs, OKMs, or derived keys on failure.

use super::argon2id::derive_argon2id_rfc9106_kat;
use super::hkdf_sha256::{HKDF_SHA256_MAX_OUTPUT_LEN, hkdf_sha256_rfc5869_expand};
use super::{
    Argon2Error, Argon2idConfig, HkdfContext, HkdfError, HkdfIkm, HkdfKeyPurpose, KEY_LEN,
    MAX_CONTEXT_LEN, MAX_ITERATIONS, MAX_LANES, MAX_MEMORY_KIB, MAX_PASSWORD_LEN, MAX_SALT_LEN,
    MIN_IKM_LEN, MIN_MEMORY_KIB, MIN_SALT_LEN, derive_kek, derive_purpose_subkey,
};

/// Secret-safe slice comparison helper that returns a boolean without formatting
/// or printing byte contents if a test assertion fails (`ARK-SEC-028`).
fn slices_equal_redacted(actual: &[u8], expected: &[u8]) -> bool {
    if actual.len() != expected.len() {
        return false;
    }
    let mut diff = 0_u8;
    for (&a, &b) in actual.iter().zip(expected.iter()) {
        diff |= a ^ b;
    }
    diff == 0
}

/// Authoritative RFC 9106 Section 5.3 Argon2id v1.3 Known-Answer Test Vector.
///
/// Specification Reference:
/// - IETF RFC 9106, Section 5.3 ("Argon2id Test Vectors")
/// - Parameters: Version = `0x13`, Memory = `32 KiB`, Iterations = `3`, Parallelism = `4`
///   lanes, Tag length = `32` bytes.
/// - Inputs:
///   - `Password[32]`: `01 01 ... 01`
///   - `Salt[16]`: `02 02 ... 02`
///   - `Secret[8]`: `03 03 ... 03`
///   - `AssociatedData[12]`: `04 04 ... 04`
/// - Expected 32-byte Tag:
///   `0d 64 0d f5 8d 78 76 6c 08 c0 37 a3 4a 8b 53 c9 d0 1e f0 45 2d 75 b6 5e b5 25 20 e9 6b 01 e6 59`
#[test]
fn kat_rfc9106_section_5_3_argon2id_v13() -> Result<(), Argon2Error> {
    let password = [0x01_u8; 32];
    let salt = [0x02_u8; 16];
    let secret = [0x03_u8; 8];
    let associated_data = [0x04_u8; 12];
    let config = Argon2idConfig {
        memory_kib: 32,
        iterations: 3,
        lanes: 4,
    };

    let expected_tag: [u8; KEY_LEN] = [
        0x0d, 0x64, 0x0d, 0xf5, 0x8d, 0x78, 0x76, 0x6c, 0x08, 0xc0, 0x37, 0xa3, 0x4a, 0x8b, 0x53,
        0xc9, 0xd0, 0x1e, 0xf0, 0x45, 0x2d, 0x75, 0xb6, 0x5e, 0xb5, 0x25, 0x20, 0xe9, 0x6b, 0x01,
        0xe6, 0x59,
    ];

    let derived = derive_argon2id_rfc9106_kat(&password, &salt, &secret, &associated_data, config)?;
    assert!(
        slices_equal_redacted(derived.as_ref(), &expected_tag),
        "RFC 9106 Section 5.3 Argon2id KAT vector mismatch (output bytes redacted)"
    );
    Ok(())
}

/// Authoritative RFC 5869 Appendix A.1 (Test Case 1) HKDF-SHA-256 Known-Answer Test.
///
/// Specification Reference:
/// - IETF RFC 5869, Appendix A.1 ("Test Case 1 - Basic test case with SHA-256")
/// - Hash = SHA-256 (`HashLen = 32`)
/// - `IKM` = 22 octets of `0x0b`
/// - `salt` = `000102030405060708090a0b0c` (13 octets)
/// - `info` = `f0f1f2f3f4f5f6f7f8f9` (10 octets)
/// - `L` = `42` octets
#[test]
fn kat_rfc5869_appendix_a1_hkdf_sha256_test_case_1() -> Result<(), HkdfError> {
    let ikm = [0x0b_u8; 22];
    let salt: [u8; 13] = [
        0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c,
    ];
    let info: [u8; 10] = [0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9];

    let expected_prk: [u8; 32] = [
        0x07, 0x77, 0x09, 0x36, 0x2c, 0x2e, 0x32, 0xdf, 0x0d, 0xdc, 0x3f, 0x0d, 0xc4, 0x7b, 0xba,
        0x63, 0x90, 0xb6, 0xc7, 0x3b, 0xb5, 0x0f, 0x9c, 0x31, 0x22, 0xec, 0x84, 0x4a, 0xd7, 0xc2,
        0xb3, 0xe5,
    ];
    let expected_okm: [u8; 42] = [
        0x3c, 0xb2, 0x5f, 0x25, 0xfa, 0xac, 0xd5, 0x7a, 0x90, 0x43, 0x4f, 0x64, 0xd0, 0x36, 0x2f,
        0x2a, 0x2d, 0x2d, 0x0a, 0x90, 0xcf, 0x1a, 0x5a, 0x4c, 0x5d, 0xb0, 0x2d, 0x56, 0xec, 0xc4,
        0xc5, 0xbf, 0x34, 0x00, 0x72, 0x08, 0xd5, 0xb8, 0x87, 0x18, 0x58, 0x65,
    ];

    let (prk, okm) = hkdf_sha256_rfc5869_expand(&ikm, Some(&salt), &info, 42)?;
    assert!(
        slices_equal_redacted(prk.as_ref(), &expected_prk),
        "RFC 5869 Test Case 1 PRK mismatch (output redacted)"
    );
    assert!(
        slices_equal_redacted(okm.as_slice(), &expected_okm),
        "RFC 5869 Test Case 1 OKM mismatch (output redacted)"
    );
    Ok(())
}

/// Authoritative RFC 5869 Appendix A.2 (Test Case 2) HKDF-SHA-256 Known-Answer Test.
///
/// Specification Reference:
/// - IETF RFC 5869, Appendix A.2 ("Test Case 2 - Test with SHA-256 and longer inputs/outputs")
/// - `IKM` = 80 octets (`0x00..=0x4f`)
/// - `salt` = 80 octets (`0x60..=0xaf`)
/// - `info` = 80 octets (`0xb0..=0xff`)
/// - `L` = `82` octets
#[test]
fn kat_rfc5869_appendix_a2_hkdf_sha256_test_case_2() -> Result<(), HkdfError> {
    let mut ikm = [0_u8; 80];
    let mut salt = [0_u8; 80];
    let mut info = [0_u8; 80];
    for i in 0..80_usize {
        ikm[i] = u8::try_from(i).unwrap_or(0);
        salt[i] = u8::try_from(0x60 + i).unwrap_or(0);
        info[i] = u8::try_from(0xb0 + i).unwrap_or(0);
    }

    let expected_prk: [u8; 32] = [
        0x06, 0xa6, 0xb8, 0x8c, 0x58, 0x53, 0x36, 0x1a, 0x06, 0x10, 0x4c, 0x9c, 0xeb, 0x35, 0xb4,
        0x5c, 0xef, 0x76, 0x00, 0x14, 0x90, 0x46, 0x71, 0x01, 0x4a, 0x19, 0x3f, 0x40, 0xc1, 0x5f,
        0xc2, 0x44,
    ];
    let expected_okm: [u8; 82] = [
        0xb1, 0x1e, 0x39, 0x8d, 0xc8, 0x03, 0x27, 0xa1, 0xc8, 0xe7, 0xf7, 0x8c, 0x59, 0x6a, 0x49,
        0x34, 0x4f, 0x01, 0x2e, 0xda, 0x2d, 0x4e, 0xfa, 0xd8, 0xa0, 0x50, 0xcc, 0x4c, 0x19, 0xaf,
        0xa9, 0x7c, 0x59, 0x04, 0x5a, 0x99, 0xca, 0xc7, 0x82, 0x72, 0x71, 0xcb, 0x41, 0xc6, 0x5e,
        0x59, 0x0e, 0x09, 0xda, 0x32, 0x75, 0x60, 0x0c, 0x2f, 0x09, 0xb8, 0x36, 0x77, 0x93, 0xa9,
        0xac, 0xa3, 0xdb, 0x71, 0xcc, 0x30, 0xc5, 0x81, 0x79, 0xec, 0x3e, 0x87, 0xc1, 0x4c, 0x01,
        0xd5, 0xc1, 0xf3, 0x43, 0x4f, 0x1d, 0x87,
    ];

    let (prk, okm) = hkdf_sha256_rfc5869_expand(&ikm, Some(&salt), &info, 82)?;
    assert!(
        slices_equal_redacted(prk.as_ref(), &expected_prk),
        "RFC 5869 Test Case 2 PRK mismatch (output redacted)"
    );
    assert!(
        slices_equal_redacted(okm.as_slice(), &expected_okm),
        "RFC 5869 Test Case 2 OKM mismatch (output redacted)"
    );
    Ok(())
}

/// Authoritative RFC 5869 Appendix A.3 (Test Case 3) HKDF-SHA-256 Known-Answer Test.
///
/// Specification Reference:
/// - IETF RFC 5869, Appendix A.3 ("Test Case 3 - Test with SHA-256 and zero-length salt/info")
/// - `IKM` = 22 octets of `0x0b`
/// - `salt` = `None` (not provided; defaults to `HashLen` zero octets)
/// - `info` = `""` (empty zero-length string)
/// - `L` = `42` octets
#[test]
fn kat_rfc5869_appendix_a3_hkdf_sha256_test_case_3() -> Result<(), HkdfError> {
    let ikm = [0x0b_u8; 22];
    let info: [u8; 0] = [];

    let expected_prk: [u8; 32] = [
        0x19, 0xef, 0x24, 0xa3, 0x2c, 0x71, 0x7b, 0x16, 0x7f, 0x33, 0xa9, 0x1d, 0x6f, 0x64, 0x8b,
        0xdf, 0x96, 0x59, 0x67, 0x76, 0xaf, 0xdb, 0x63, 0x77, 0xac, 0x43, 0x4c, 0x1c, 0x29, 0x3c,
        0xcb, 0x04,
    ];
    let expected_okm: [u8; 42] = [
        0x8d, 0xa4, 0xe7, 0x75, 0xa5, 0x63, 0xc1, 0x8f, 0x71, 0x5f, 0x80, 0x2a, 0x06, 0x3c, 0x5a,
        0x31, 0xb8, 0xa1, 0x1f, 0x5c, 0x5e, 0xe1, 0x87, 0x9e, 0xc3, 0x45, 0x4e, 0x5f, 0x3c, 0x73,
        0x8d, 0x2d, 0x9d, 0x20, 0x13, 0x95, 0xfa, 0xa4, 0xb6, 0x1a, 0x96, 0xc8,
    ];

    let (prk, okm) = hkdf_sha256_rfc5869_expand(&ikm, None, &info, 42)?;
    assert!(
        slices_equal_redacted(prk.as_ref(), &expected_prk),
        "RFC 5869 Test Case 3 PRK mismatch (output redacted)"
    );
    assert!(
        slices_equal_redacted(okm.as_slice(), &expected_okm),
        "RFC 5869 Test Case 3 OKM mismatch (output redacted)"
    );
    Ok(())
}

/// Independently computed HKDF-SHA-256 Known-Answer Test for `derive_subkey` and
/// `derive_purpose_subkey` with canonical `"ARK/v1/project-key"` context.
///
/// Expected 32-byte OKM computed via Python 3.14 standard library `hmac.new` + `hashlib.sha256`
/// following RFC 5869 Section 2.2 & 2.3:
/// - `IKM[32]`: `0x01, 0x02, ..., 0x20`
/// - `Salt[32]`: `0x10, 0x11, ..., 0x2f`
/// - `Info`: `b"ARK/v1/project-key"`
/// - `Expected OKM[32]`:
///   `b7 3e 5b f3 2f b1 49 14 97 84 45 2d 0b 19 7f 78 cd 06 dc 1c ea 96 67 17 a4 ff 3d fb e7 3d c9 1d`
#[test]
fn kat_ark_v1_project_key_subkey_derivation() -> Result<(), HkdfError> {
    let mut ikm = [0_u8; 32];
    let mut salt = [0_u8; 32];
    for i in 0..32_usize {
        ikm[i] = u8::try_from(i + 1).unwrap_or(0);
        salt[i] = u8::try_from(0x10 + i).unwrap_or(0);
    }

    let expected_subkey: [u8; KEY_LEN] = [
        0xb7, 0x3e, 0x5b, 0xf3, 0x2f, 0xb1, 0x49, 0x14, 0x97, 0x84, 0x45, 0x2d, 0x0b, 0x19, 0x7f,
        0x78, 0xcd, 0x06, 0xdc, 0x1c, 0xea, 0x96, 0x67, 0x17, 0xa4, 0xff, 0x3d, 0xfb, 0xe7, 0x3d,
        0xc9, 0x1d,
    ];

    let typed_ikm = HkdfIkm::from_key_array(&ikm);
    let subkey_typed = derive_purpose_subkey(typed_ikm, Some(&salt), HkdfKeyPurpose::ProjectKey)?;
    assert!(
        slices_equal_redacted(subkey_typed.as_ref(), &expected_subkey),
        "ARK/v1/project-key derive_purpose_subkey KAT vector mismatch (output redacted)"
    );
    Ok(())
}

/// Locks the exact ARK/v1 bytes and registered semantics for all six HKDF purposes.
#[test]
fn hkdf_registered_purpose_registry_is_stable() -> Result<(), HkdfError> {
    let cases: [(HkdfKeyPurpose, &[u8]); 6] = [
        (HkdfKeyPurpose::WorkspaceKey, b"ARK/v1/workspace-key"),
        (HkdfKeyPurpose::ProjectKey, b"ARK/v1/project-key"),
        (HkdfKeyPurpose::FileKey, b"ARK/v1/file-key"),
        (HkdfKeyPurpose::MetadataKey, b"ARK/v1/metadata-key"),
        (HkdfKeyPurpose::AuditKey, b"ARK/v1/audit-key"),
        (HkdfKeyPurpose::RecoveryKey, b"ARK/v1/recovery-key"),
    ];

    let root_key = [0x42_u8; KEY_LEN];
    let ikm = HkdfIkm::from_key_array(&root_key);
    let mut derived = Vec::with_capacity(cases.len());
    for (purpose, expected_context) in cases {
        let context = HkdfContext::from_purpose(purpose);
        assert_eq!(context.as_bytes(), expected_context);
        assert_eq!(
            HkdfContext::parse(expected_context)?.as_bytes(),
            expected_context
        );
        derived.push(derive_purpose_subkey(ikm, None, purpose)?);
    }

    for left in 0..derived.len() {
        for right in (left + 1)..derived.len() {
            assert!(
                !slices_equal_redacted(derived[left].as_ref(), derived[right].as_ref()),
                "distinct registered purposes must not share the same test output (bytes redacted)"
            );
        }
    }
    Ok(())
}

/// Verifies secret-bearing wrappers do not expose their bytes through Debug.
#[test]
fn kdf_secret_wrappers_redact_debug_output() -> Result<(), Argon2Error> {
    let salt_bytes = [0xA5_u8; MIN_SALT_LEN];
    let salt = super::Argon2Salt::from_slice(&salt_bytes)?;
    let salt_debug = format!("{salt:?}");
    assert!(salt_debug.contains("Argon2Salt"));
    assert!(salt_debug.contains("len"));
    assert!(!salt_debug.contains("165"));

    let ikm_bytes = [0xB6_u8; KEY_LEN];
    let ikm = HkdfIkm::from_key_array(&ikm_bytes);
    let ikm_debug = format!("{ikm:?}");
    assert!(ikm_debug.contains("HkdfIkm"));
    assert!(ikm_debug.contains("len"));
    assert!(!ikm_debug.contains("182"));
    Ok(())
}

/// Demonstrates that minimum-length validation is not an entropy/provenance check.
///
/// An all-zero 16-byte value satisfies the syntactic length wrapper. This is
/// intentional: the primitive cannot infer where bytes came from or their entropy.
/// Production callers must enforce approved high-entropy key provenance upstream.
#[test]
fn hkdf_ikm_length_validation_does_not_claim_entropy() {
    let low_entropy_fixture = [0_u8; MIN_IKM_LEN];
    assert!(HkdfIkm::from_slice(&low_entropy_fixture).is_ok());
}

/// Comprehensive boundary, output-length, and secret-safe error-path regression tests
/// covering the finalized `[S-02]`, `[S-03]`, `[S-04]`, and `[S-05]` contracts.
#[test]
fn kdf_boundary_and_error_path_regression_suite() -> Result<(), HkdfError> {
    // 1. HKDF RFC 5869 output length boundaries (1..=8160 bytes)
    let ikm = [0x5A_u8; MIN_IKM_LEN];
    assert_eq!(
        hkdf_sha256_rfc5869_expand(&ikm, None, b"ARK/v1/test", 0),
        Err(HkdfError::InvalidOutputLength)
    );
    assert_eq!(
        hkdf_sha256_rfc5869_expand(&ikm, None, b"ARK/v1/test", HKDF_SHA256_MAX_OUTPUT_LEN + 1),
        Err(HkdfError::InvalidOutputLength)
    );

    let (_, min_okm) = hkdf_sha256_rfc5869_expand(&ikm, None, b"ARK/v1/test", 1)?;
    assert_eq!(min_okm.len(), 1);

    let (_, max_okm) =
        hkdf_sha256_rfc5869_expand(&ikm, None, b"ARK/v1/test", HKDF_SHA256_MAX_OUTPUT_LEN)?;
    assert_eq!(max_okm.len(), HKDF_SHA256_MAX_OUTPUT_LEN);

    // 2. The production parser rejects overlong context values before derivation.
    let oversized_context = [b'x'; MAX_CONTEXT_LEN + 1];
    assert_eq!(
        HkdfContext::parse(&oversized_context),
        Err(HkdfError::ContextTooLong)
    );

    // 3. Argon2id exact boundary checks (min/max password, min/max salt, parameter bounds)
    let min_pw = [b'p'; 1];
    let max_pw = [b'p'; MAX_PASSWORD_LEN];
    let min_salt = [0x01_u8; MIN_SALT_LEN];
    let max_salt = [0xFE_u8; MAX_SALT_LEN];
    let min_cfg = Argon2idConfig {
        memory_kib: MIN_MEMORY_KIB,
        iterations: 1,
        lanes: 1,
    };

    let k_min = derive_kek(&min_pw, &min_salt, min_cfg);
    assert!(k_min.is_ok(), "1-byte password and 16-byte salt must pass");

    let k_max = derive_kek(&max_pw, &max_salt, min_cfg);
    assert!(
        k_max.is_ok(),
        "1024-byte password and 64-byte salt must pass"
    );

    assert_eq!(
        derive_argon2id_rfc9106_kat(
            &min_pw,
            &min_salt,
            b"",
            b"",
            Argon2idConfig {
                memory_kib: MAX_MEMORY_KIB + 1,
                iterations: 1,
                lanes: 1,
            }
        ),
        Err(Argon2Error::InvalidParameters)
    );
    assert_eq!(
        derive_argon2id_rfc9106_kat(
            &min_pw,
            &min_salt,
            b"",
            b"",
            Argon2idConfig {
                memory_kib: 32,
                iterations: MAX_ITERATIONS + 1,
                lanes: 1,
            }
        ),
        Err(Argon2Error::InvalidParameters)
    );
    assert_eq!(
        derive_argon2id_rfc9106_kat(
            &min_pw,
            &min_salt,
            b"",
            b"",
            Argon2idConfig {
                memory_kib: 128,
                iterations: 1,
                lanes: MAX_LANES + 1,
            }
        ),
        Err(Argon2Error::InvalidParameters)
    );

    Ok(())
}
