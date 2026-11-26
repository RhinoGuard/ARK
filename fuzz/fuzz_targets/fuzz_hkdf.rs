#![no_main]

use ark_crypto::kdf::{
    HkdfContext, HkdfIkm, HkdfKeyPurpose, MAX_CONTEXT_LEN, derive_purpose_subkey,
};
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    if data.is_empty() {
        let _empty_ctx = HkdfContext::parse(&[]);
        let _empty_ikm = HkdfIkm::from_slice(&[]);
        return;
    }

    // Exercise the production context parser with arbitrary caller bytes.
    // It must reject unknown purposes, malformed namespace values, and
    // inputs exceeding MAX_CONTEXT_LEN.
    let context_len = data.len().min(MAX_CONTEXT_LEN.saturating_add(1));
    let _context_result = HkdfContext::parse(&data[..context_len]);

    let selector = data[0];
    let rest = &data[1..];

    // Exercise both salt = None and salt = Some(...) across empty, short,
    // 32-byte, and >64-byte HMAC-SHA-256 block-boundary lengths.
    let (salt, ikm_raw) = if (selector & 0x80) == 0 {
        (None, rest)
    } else {
        let requested_salt_len = usize::from(selector & 0x7f);
        let salt_len = requested_salt_len.min(rest.len());
        let (salt_slice, ikm_slice) = rest.split_at(salt_len);
        (Some(salt_slice), ikm_slice)
    };

    // Exercise the typed IKM constructor (including empty/short rejections)
    // and the public purpose-bound HKDF derivation path.
    let ikm_len = ikm_raw.len().min(1024);
    let Ok(ikm) = HkdfIkm::from_slice(&ikm_raw[..ikm_len]) else {
        return;
    };

    let purpose = match selector % 6 {
        0 => HkdfKeyPurpose::WorkspaceKey,
        1 => HkdfKeyPurpose::ProjectKey,
        2 => HkdfKeyPurpose::FileKey,
        3 => HkdfKeyPurpose::MetadataKey,
        4 => HkdfKeyPurpose::AuditKey,
        _ => HkdfKeyPurpose::RecoveryKey,
    };

    let _result = derive_purpose_subkey(ikm, salt, purpose);
});
