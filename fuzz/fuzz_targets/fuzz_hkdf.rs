#![no_main]

use ark_crypto::kdf::{
    HkdfContext, HkdfIkm, HkdfKeyPurpose, MAX_CONTEXT_LEN, derive_purpose_subkey,
};
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    if data.is_empty() {
        return;
    }

    // Exercise the production context parser with arbitrary caller bytes.
    // It must reject unknown purposes and malformed namespace values.
    let context_len = data.len().min(MAX_CONTEXT_LEN.saturating_add(1));
    let _context_result = HkdfContext::parse(&data[..context_len]);

    // Exercise the typed IKM constructor and the public purpose-bound HKDF
    // derivation path. Input length alone is not an entropy/provenance proof;
    // the harness tests API robustness, not key-source policy.
    let ikm_len = data.len().min(1024);
    let Ok(ikm) = HkdfIkm::from_slice(&data[..ikm_len]) else {
        return;
    };

    let purpose = match data[0] % 6 {
        0 => HkdfKeyPurpose::WorkspaceKey,
        1 => HkdfKeyPurpose::ProjectKey,
        2 => HkdfKeyPurpose::FileKey,
        3 => HkdfKeyPurpose::MetadataKey,
        4 => HkdfKeyPurpose::AuditKey,
        _ => HkdfKeyPurpose::RecoveryKey,
    };

    let _result = derive_purpose_subkey(ikm, None, purpose);
});
