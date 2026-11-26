#![no_main]

use ark_crypto::kdf::{
    Argon2Salt, Argon2idConfig, DEFAULT_LANES, MIN_ITERATIONS, MIN_MEMORY_KIB, derive_kek,
    derive_kek_with_salt,
};
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    // Exercise config parameter validation across arbitrary u32 boundaries
    // without allocating Argon2id memory blocks.
    if data.len() >= 12 {
        let memory_kib = u32::from_le_bytes([data[0], data[1], data[2], data[3]]);
        let iterations = u32::from_le_bytes([data[4], data[5], data[6], data[7]]);
        let lanes = u32::from_le_bytes([data[8], data[9], data[10], data[11]]);
        let _config_result = Argon2idConfig::new(memory_kib, iterations, lanes);
    }

    let Some((&selector, rest)) = data.split_first() else {
        let Ok(config) = Argon2idConfig::new(MIN_MEMORY_KIB, MIN_ITERATIONS, DEFAULT_LANES) else {
            return;
        };
        let _empty_result = derive_kek(&[], &[], config);
        return;
    };

    // Exercise salt lengths across 0..=80 bytes (below MIN_SALT_LEN, valid 16..=64,
    // and above MAX_SALT_LEN) and passwords across 0..=1050 bytes (empty, valid
    // 1..=1024, and above MAX_PASSWORD_LEN).
    let requested_salt_len = usize::from(selector) % 81;
    let salt_len = requested_salt_len.min(rest.len());
    let (salt, password_full) = rest.split_at(salt_len);
    let password = &password_full[..password_full.len().min(1050)];

    // Keep valid derivations bounded to MIN_MEMORY_KIB (16 MiB) and 1..=2 iterations
    // so sustained fuzzing exercises real Argon2id work within the RSS budget.
    let iterations = MIN_ITERATIONS + u32::from((selector >> 6) & 1);
    let Ok(config) = Argon2idConfig::new(MIN_MEMORY_KIB, iterations, DEFAULT_LANES) else {
        return;
    };

    if (selector & 0x80) != 0 {
        if let Ok(validated_salt) = Argon2Salt::from_slice(salt) {
            let _result = derive_kek_with_salt(password, &validated_salt, config);
        }
    } else {
        let _result = derive_kek(password, salt, config);
    }
});
