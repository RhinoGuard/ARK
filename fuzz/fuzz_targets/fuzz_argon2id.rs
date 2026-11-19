#![no_main]

use ark_crypto::kdf::{
    Argon2idConfig, DEFAULT_LANES, DEFAULT_ITERATIONS, MIN_MEMORY_KIB, derive_kek,
};
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    // Keep each invocation bounded: the final 16 bytes are the salt and the
    // preceding 1..=1024 bytes are the password. The minimum production memory
    // cost is intentionally used so malformed inputs cannot request arbitrary
    // resource consumption through this harness.
    if !(17..=1040).contains(&data.len()) {
        return;
    }

    let password_len = data.len() - 16;
    let (password, salt) = data.split_at(password_len);
    let Ok(config) = Argon2idConfig::new(MIN_MEMORY_KIB, DEFAULT_ITERATIONS, DEFAULT_LANES)
    else {
        return;
    };

    // Exercise the real production entry point. Invalid password/salt values
    // are expected to return sanitized errors; fuzzing must not panic.
    let _result = derive_kek(password, salt, config);
});
