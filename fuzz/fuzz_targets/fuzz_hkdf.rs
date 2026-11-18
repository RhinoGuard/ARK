#![no_main]

use libfuzzer_sys::fuzz_target;

const MAX_CONTEXT_LEN: usize = 256;

/// Context and IKM boundary simulation struct for HKDF fuzz scaffolding.
/// NOTE: This target evaluates context length and non-empty slice validation;
/// it does NOT directly invoke `derive_subkey` in the current un-integrated workspace layout.
fuzz_target!(|data: &[u8]| {
    // Structure:
    // [0]: ikm length
    // [1]: salt length
    // [2..]: buffers for ikm, salt, and context
    if data.len() < 3 {
        return;
    }

    let ikm_len = (data[0] as usize) % (data.len() - 2 + 1);
    let rem1 = &data[2..];
    let ikm = &rem1[..ikm_len];

    let rem2 = &rem1[ikm_len..];
    let salt_len = (data[1] as usize) % (rem2.len() + 1);
    let salt = if salt_len > 0 {
        Some(&rem2[..salt_len])
    } else {
        None
    };

    let context = &rem2[salt_len..];

    // Evaluate input bounds matching HKDF-SHA-256 specification:
    // - empty IKM rejected
    // - empty context rejected
    // - context > MAX_CONTEXT_LEN rejected
    let is_valid_ikm = !ikm.is_empty();
    let is_valid_context = !context.is_empty() && context.len() <= MAX_CONTEXT_LEN;

    let should_succeed = is_valid_ikm && is_valid_context;
    let _ = (should_succeed, salt);
});
