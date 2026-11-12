#![no_main]

use ark_core::ids::{DeviceId, PackageId, ProcessGrantId, SessionId, StorageId, WorkspaceId};
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    if data.len() < 16 {
        return;
    }

    let mut raw = [0u8; 16];
    raw.copy_from_slice(&data[..16]);

    let ws = WorkspaceId::from_bytes(raw);
    let st = StorageId::from_bytes(raw);
    let ss = SessionId::from_bytes(raw);
    let dev = DeviceId::from_bytes(raw);
    let pkg = PackageId::from_bytes(raw);
    let grt = ProcessGrantId::from_bytes(raw);

    // Verify invariants under fuzzing:
    // 1. Serialization must always be exactly 32 hex characters
    let hex_ws = format!("{ws}");
    assert_eq!(hex_ws.len(), 32);

    let hex_st = format!("{st}");
    assert_eq!(hex_st.len(), 32);

    let hex_ss = format!("{ss}");
    assert_eq!(hex_ss.len(), 32);

    let hex_dev = format!("{dev}");
    assert_eq!(hex_dev.len(), 32);

    let hex_pkg = format!("{pkg}");
    assert_eq!(hex_pkg.len(), 32);

    let hex_grt = format!("{grt}");
    assert_eq!(hex_grt.len(), 32);

    // 2. Round-trip byte integrity
    assert_eq!(ws.as_bytes(), &raw);
    assert_eq!(ws.into_bytes(), raw);
});
