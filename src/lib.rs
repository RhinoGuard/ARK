//! Focused cryptographic key-derivation component for ARK.
//!
//! This crate intentionally implements only password-based KEK derivation
//! (Argon2id) and context-separated subkey derivation (HKDF-SHA-256).
//! It does not implement key wrapping, file encryption, nonce allocation,
//! storage formats, authorization, or UI integration.

pub mod kdf;