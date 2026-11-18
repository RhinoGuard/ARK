#![no_main]

use libfuzzer_sys::fuzz_target;

/// Parameter bound simulation struct for Argon2id fuzz scaffolding.
/// NOTE: This target evaluates parameter decoding and bounding logic;
/// it does NOT directly invoke `derive_kek` in the current un-integrated workspace layout.
#[derive(Clone, Copy, Debug)]
struct FuzzArgon2Config {
    memory_kib: u32,
    iterations: u32,
    lanes: u32,
}

fuzz_target!(|data: &[u8]| {
    // Structure:
    // [0..2]: memory_kib seed (bounded to 8..=1024 KiB)
    // [2]: iterations seed (bounded to 1..=2)
    // [3]: lanes seed (bounded to 1..=2)
    // [4]: password length byte
    // [5..]: salt & password buffer
    if data.len() < 6 {
        return;
    }

    let raw_mem = u16::from_le_bytes([data[0], data[1]]);
    let memory_kib = (8 + (raw_mem % 1017)) as u32; // strictly bounded
    let iterations = 1 + (data[2] % 2) as u32;      // 1 or 2 iterations
    let lanes = 1 + (data[3] % 2) as u32;           // 1 or 2 lanes

    let config = FuzzArgon2Config {
        memory_kib,
        iterations,
        lanes,
    };

    let pw_len = (data[4] as usize) % (data.len() - 5 + 1);
    let password = &data[5..5 + pw_len];
    let salt = &data[5 + pw_len..];

    // Evaluate input bounds without panicking:
    // - empty password rejected
    // - password > 1024 rejected
    // - salt not in 16..=64 rejected
    let is_valid_pw = !password.is_empty() && password.len() <= 1024;
    let is_valid_salt = (16..=64).contains(&salt.len());
    let is_valid_params = config.memory_kib >= 8 && config.iterations >= 1 && config.lanes >= 1;

    let should_succeed = is_valid_pw && is_valid_salt && is_valid_params;
    let _ = should_succeed;

    // Output representation validation
    let _ = format!("{config:?}");
});
