#![no_main]

use ark_core::versions::{ApplicationVersion, IpcProtocolVersion, PackageFormatVersion, StorageFormatVersion};
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    if data.len() < 6 {
        return;
    }

    let major = u16::from_le_bytes([data[0], data[1]]);
    let minor = u16::from_le_bytes([data[2], data[3]]);
    let patch = u16::from_le_bytes([data[4], data[5]]);

    let app_ver = ApplicationVersion::new(major, minor, patch);
    assert_eq!(app_ver.major(), major);
    assert_eq!(app_ver.minor(), minor);
    assert_eq!(app_ver.patch(), patch);

    let storage_ver = StorageFormatVersion::new(major, minor);
    assert_eq!(storage_ver.major(), major);
    assert_eq!(storage_ver.minor(), minor);

    let pkg_ver = PackageFormatVersion::new(major, minor);
    assert_eq!(pkg_ver.major(), major);
    assert_eq!(pkg_ver.minor(), minor);

    let ipc_ver = IpcProtocolVersion::new(major, minor);
    assert_eq!(ipc_ver.major(), major);
    assert_eq!(ipc_ver.minor(), minor);

    // Formatted debug output must never panic
    let _ = format!("{app_ver:?}");
    let _ = format!("{storage_ver:?}");
    let _ = format!("{pkg_ver:?}");
    let _ = format!("{ipc_ver:?}");
});
