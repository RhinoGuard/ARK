//! Negative and edge-case security tests for ark-core domain primitives.
//!
//! Validates how domain types behave under boundary, extreme, and malformed inputs.

use ark_core::errors::{ArkError, ErrorCode};
use ark_core::ids::{
    DeviceId, FileId, PackageId, ProjectId, SessionId, TransactionId, WorkspaceId,
};
use ark_core::versions::{ApplicationVersion, StorageFormatVersion};
use core::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

#[test]
fn test_negative_all_zero_and_all_ff_identifiers() {
    // Extreme boundary values for identifiers (all-zeros, all-ones)
    let zeros = [0x00u8; 16];
    let ones = [0xFFu8; 16];

    let ws_zero = WorkspaceId::from_bytes(zeros);
    let ws_ones = WorkspaceId::from_bytes(ones);

    assert_ne!(ws_zero, ws_ones);
    // Hash distinction on boundary inputs
    assert_ne!(calculate_hash(&ws_zero), calculate_hash(&ws_ones));

    // Hex representation must always be strictly 32 characters, even with all zeros
    let hex_zero = format!("{ws_zero}");
    assert_eq!(hex_zero.len(), 32);
    assert_eq!(hex_zero, "00000000000000000000000000000000");

    let hex_ones = format!("{ws_ones}");
    assert_eq!(hex_ones.len(), 32);
    assert_eq!(hex_ones, "ffffffffffffffffffffffffffffffff");
}

#[test]
fn test_negative_single_bit_flip_identifier_inequality() {
    // Single bit flip must result in completely distinct identifier value
    let b1 = [0xAAu8; 16];
    let mut b2 = [0xAAu8; 16];
    b2[15] ^= 0x01; // flip 1 bit

    let id1 = WorkspaceId::from_bytes(b1);
    let id2 = WorkspaceId::from_bytes(b2);

    assert_ne!(id1, id2);
    // Note: while hash collision is theoretically possible for 64-bit hashes,
    // distinct inputs should not trivially produce identical hashes.
    assert_ne!(calculate_hash(&id1), calculate_hash(&id2));
}

#[test]
fn test_negative_version_extreme_bounds() {
    // Ensure version handling does not overflow or panic at u16 boundaries
    let max_app_version = ApplicationVersion::new(u16::MAX, u16::MAX, u16::MAX);
    assert_eq!(max_app_version.major(), u16::MAX);
    assert_eq!(max_app_version.minor(), u16::MAX);
    assert_eq!(max_app_version.patch(), u16::MAX);

    let zero_app_version = ApplicationVersion::new(0, 0, 0);
    assert!(zero_app_version < max_app_version);

    let max_storage_version = StorageFormatVersion::new(u16::MAX, u16::MAX);
    let zero_storage_version = StorageFormatVersion::new(0, 0);
    assert!(zero_storage_version < max_storage_version);
}

#[test]
fn test_negative_error_code_exhaustiveness() {
    // Verify that every single ErrorCode variant preserves equality and distinction
    let error_variants = [
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

    for i in 0..error_variants.len() {
        for j in 0..error_variants.len() {
            let e1 = ArkError::new(error_variants[i]);
            let e2 = ArkError::new(error_variants[j]);
            if i == j {
                assert_eq!(e1, e2);
                assert_eq!(calculate_hash(&e1), calculate_hash(&e2));
            } else {
                assert_ne!(e1, e2);
            }
        }
    }
}

#[test]
fn test_negative_domain_identifier_cross_construction() {
    // Ensure all 7 official domain IDs can be safely constructed from identical bytes
    // without cross-contamination.
    let bytes = [0x11u8; 16];
    let ws = WorkspaceId::from_bytes(bytes);
    let prj = ProjectId::from_bytes(bytes);
    let fil = FileId::from_bytes(bytes);
    let ses = SessionId::from_bytes(bytes);
    let dev = DeviceId::from_bytes(bytes);
    let pkg = PackageId::from_bytes(bytes);
    let txn = TransactionId::from_bytes(bytes);

    assert_eq!(ws.as_bytes(), prj.as_bytes());
    assert_eq!(fil.as_bytes(), ses.as_bytes());
    assert_eq!(dev.as_bytes(), pkg.as_bytes());
    assert_eq!(txn.as_bytes(), &bytes);
}
