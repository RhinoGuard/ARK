//! Security-relevant vocabulary and contract tests for ark-core domain primitives.
//!
//! Validates:
//! - Low-detail classification formatting for ArkError (Display/Debug representations).
//! - Strict domain type isolation (WorkspaceId != ProjectId != FileId != SessionId).
//! - Domain version segregation and numeric ordering comparisons.
//! - RuntimeMode distinctness (workflow vocabulary, not access authorization).

use ark_core::errors::{ArkError, ErrorCode};
use ark_core::ids::{
    DeviceId, FileId, PackageId, ProjectId, SessionId, TransactionId, WorkspaceId,
};
use ark_core::runtime::RuntimeMode;
use ark_core::versions::{
    ApplicationVersion, IpcProtocolVersion, MetadataSchemaVersion, PackageFormatVersion,
    PolicySchemaVersion, RecoveryFormatVersion, StorageFormatVersion,
};

#[test]
fn test_invariant_sanitized_error_display_format() {
    // Tests that ErrorCode classification maps to standard static label text.
    // NOTE: This checks coarse classification formatting only; comprehensive secret sanitization
    // across complex runtime error chains requires explicit sentinel injection testing.
    let error = ArkError::new(ErrorCode::IntegrityFailure);

    assert_eq!(error.code(), ErrorCode::IntegrityFailure);

    let display_str = format!("{error}");
    assert!(
        display_str.contains("integrity"),
        "Display representation must match static classification label"
    );

    let debug_str = format!("{error:?}");
    assert!(debug_str.starts_with("ArkError"));
}

#[test]
fn test_invariant_all_error_codes_classified() {
    // Verifies that every domain failure code provides a non-empty static Display label.
    let codes = [
        ErrorCode::InvalidArgument,
        ErrorCode::InvalidIdentifier,
        ErrorCode::InvalidVersion,
        ErrorCode::InvalidStateTransition,
        ErrorCode::NotFound,
        ErrorCode::Conflict,
        ErrorCode::Unauthenticated,
        ErrorCode::AccessDenied,
        ErrorCode::Expired,
        ErrorCode::Revoked,
        ErrorCode::Unsupported,
        ErrorCode::IntegrityFailure,
        ErrorCode::ResourceExhausted,
        ErrorCode::Unavailable,
        ErrorCode::Cancelled,
        ErrorCode::Internal,
    ];

    for code in codes {
        let err = ArkError::new(code);
        assert_eq!(err.code(), code);
        let msg = format!("{err}");
        assert!(!msg.is_empty());
    }
}

#[test]
fn test_invariant_identifier_type_safety() {
    // Verifies domain identifier type isolation across all 7 officially supported domain IDs.
    // NOTE: Typed identifiers provide domain vocabulary separation; they do not grant authorization.
    let raw_bytes = [0x42u8; 16];

    let ws_id = WorkspaceId::from_bytes(raw_bytes);
    let proj_id = ProjectId::from_bytes(raw_bytes);
    let file_id = FileId::from_bytes(raw_bytes);
    let ss_id = SessionId::from_bytes(raw_bytes);
    let dev_id = DeviceId::from_bytes(raw_bytes);
    let pkg_id = PackageId::from_bytes(raw_bytes);
    let tx_id = TransactionId::from_bytes(raw_bytes);

    assert_eq!(ws_id.into_bytes(), [0x42u8; 16]);
    assert_eq!(proj_id.into_bytes(), [0x42u8; 16]);
    assert_eq!(file_id.into_bytes(), [0x42u8; 16]);
    assert_eq!(ss_id.into_bytes(), [0x42u8; 16]);
    assert_eq!(dev_id.into_bytes(), [0x42u8; 16]);
    assert_eq!(pkg_id.into_bytes(), [0x42u8; 16]);
    assert_eq!(tx_id.into_bytes(), [0x42u8; 16]);

    let hex_expected = "42424242424242424242424242424242";
    assert_eq!(format!("{ws_id}"), hex_expected);
    assert_eq!(format!("{proj_id}"), hex_expected);
    assert_eq!(format!("{file_id}"), hex_expected);
    assert_eq!(format!("{ss_id}"), hex_expected);
    assert_eq!(format!("{dev_id}"), hex_expected);
    assert_eq!(format!("{pkg_id}"), hex_expected);
    assert_eq!(format!("{tx_id}"), hex_expected);
}

#[test]
fn test_invariant_runtime_mode_distinctness() {
    // Verifies that RuntimeMode variants are distinct workflow descriptors.
    // NOTE: RuntimeMode describes access patterns (Development vs Execution);
    // it does NOT establish a security boundary, enforce sandbox isolation, or grant access.
    let dev = RuntimeMode::Development;
    let exec = RuntimeMode::Execution;

    assert_ne!(dev, exec);
    assert_eq!(dev, RuntimeMode::Development);
    assert_eq!(exec, RuntimeMode::Execution);
}

#[test]
fn test_invariant_version_monotonic_ordering() {
    // Tests component-wise numeric Ord comparison for versions.
    // NOTE: Numeric version ordering is a mathematical property and does NOT alone
    // provide anti-rollback protection. Anti-rollback requires trusted monotonic state anchoring.
    let v1 = ApplicationVersion::new(1, 0, 0);
    let v1_patch = ApplicationVersion::new(1, 0, 1);
    let v1_minor = ApplicationVersion::new(1, 1, 0);
    let v2 = ApplicationVersion::new(2, 0, 0);

    assert!(v1 < v1_patch);
    assert!(v1_patch < v1_minor);
    assert!(v1_minor < v2);

    let storage_v1 = StorageFormatVersion::new(1, 0);
    let storage_v2 = StorageFormatVersion::new(2, 0);
    assert!(storage_v1 < storage_v2);
}

#[test]
fn test_invariant_version_domain_separation() {
    // Verifies that distinct format version newtypes prevent accidental cross-domain substitution.
    let storage_ver = StorageFormatVersion::new(1, 0);
    let metadata_ver = MetadataSchemaVersion::new(1, 0);
    let policy_ver = PolicySchemaVersion::new(1, 0);
    let recovery_ver = RecoveryFormatVersion::new(1, 0);
    let package_ver = PackageFormatVersion::new(1, 0);
    let ipc_ver = IpcProtocolVersion::new(1, 0);

    assert_eq!((storage_ver.major(), storage_ver.minor()), (1, 0));
    assert_eq!((metadata_ver.major(), metadata_ver.minor()), (1, 0));
    assert_eq!((policy_ver.major(), policy_ver.minor()), (1, 0));
    assert_eq!((recovery_ver.major(), recovery_ver.minor()), (1, 0));
    assert_eq!((package_ver.major(), package_ver.minor()), (1, 0));
    assert_eq!((ipc_ver.major(), ipc_ver.minor()), (1, 0));
}
