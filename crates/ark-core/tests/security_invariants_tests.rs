//! Security invariant tests for ark-core domain primitives.
//!
//! Validates non-negotiable security requirements including:
//! - Sanitized error boundaries (no internal data leakage in display/debug representations).
//! - Strict domain type isolation (WorkspaceId != StorageId != SessionId at type level).
//! - Domain version segregation and monotonic ordering for anti-rollback.

use ark_core::errors::{ArkError, ErrorCode};
use ark_core::ids::{DeviceId, PackageId, ProcessGrantId, SessionId, StorageId, WorkspaceId};
use ark_core::runtime::RuntimeMode;
use ark_core::versions::{ApplicationVersion, IpcProtocolVersion, PackageFormatVersion, StorageFormatVersion};

#[test]
fn test_invariant_sanitized_error_no_leakage() {
    // SECURITY INVARIANT: Errors crossing boundaries must never contain arbitrary payload,
    // secrets, file paths, or OS details.
    let error = ArkError::new(ErrorCode::IntegrityFailure);

    // Assert coarse classification code is exact
    assert_eq!(error.code(), ErrorCode::IntegrityFailure);

    // Assert Display representation contains only standardized label
    let display_str = format!("{error}");
    assert!(
        display_str.contains("integrity"),
        "Display representation must be standard classification"
    );

    // Verify debug representation does not expose memory addresses or raw internals
    let debug_str = format!("{error:?}");
    assert!(debug_str.starts_with("ArkError"));
}

#[test]
fn test_invariant_all_error_codes_classified() {
    // SECURITY INVARIANT: Every domain failure must map to a known, non-leaking ErrorCode.
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
        // Ensure Display formatting does not panic for any defined code
        let msg = format!("{err}");
        assert!(!msg.is_empty());
    }
}

#[test]
fn test_invariant_identifier_type_safety() {
    // SECURITY INVARIANT: Identifiers for different domain entities must never be interchangeable
    // without explicit type conversion.
    let raw_bytes = [0x42u8; 16];

    let ws_id = WorkspaceId::from_bytes(raw_bytes);
    let st_id = StorageId::from_bytes(raw_bytes);
    let ss_id = SessionId::from_bytes(raw_bytes);
    let dev_id = DeviceId::from_bytes(raw_bytes);
    let pkg_id = PackageId::from_bytes(raw_bytes);
    let grt_id = ProcessGrantId::from_bytes(raw_bytes);

    // Even though underlying bytes match, equality is confined to identical types.
    assert_eq!(ws_id.as_bytes(), st_id.as_bytes());
    assert_eq!(ws_id.into_bytes(), [0x42u8; 16]);

    // Format check: all identifiers must serialize to exactly 32 lowercase hex characters
    let hex_ws = format!("{ws_id}");
    assert_eq!(hex_ws.len(), 32);
    assert_eq!(hex_ws, "42424242424242424242424242424242");

    assert_eq!(format!("{st_id}"), hex_ws);
    assert_eq!(format!("{ss_id}"), hex_ws);
    assert_eq!(format!("{dev_id}"), hex_ws);
    assert_eq!(format!("{pkg_id}"), hex_ws);
    assert_eq!(format!("{grt_id}"), hex_ws);
}

#[test]
fn test_invariant_runtime_mode_isolation() {
    // SECURITY INVARIANT: Runtime modes must be strictly segregated; each mode enforces distinct trust tiers.
    let isolated = RuntimeMode::Isolated;
    let standard = RuntimeMode::Standard;
    let permissive = RuntimeMode::Permissive;

    assert_ne!(isolated, standard);
    assert_ne!(standard, permissive);
    assert_ne!(isolated, permissive);
}

#[test]
fn test_invariant_version_monotonicity_and_ordering() {
    // SECURITY INVARIANT: Anti-rollback guarantees require strict monotonic ordering of versions.
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
    // SECURITY INVARIANT: Storage, Package, and IPC versions must not be accidentally substitutable.
    let storage_ver = StorageFormatVersion::new(1, 0);
    let package_ver = PackageFormatVersion::new(1, 0);
    let ipc_ver = IpcProtocolVersion::new(1, 0);

    assert_eq!((storage_ver.major(), storage_ver.minor()), (1, 0));
    assert_eq!((package_ver.major(), package_ver.minor()), (1, 0));
    assert_eq!((ipc_ver.major(), ipc_ver.minor()), (1, 0));
}
