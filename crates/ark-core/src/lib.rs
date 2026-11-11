//! Platform-independent domain primitives shared by ARK subsystems.
//!
//! This crate is the dependency-free foundation of the ARK domain model. Its initial public
//! surface is intentionally small and does not implement storage, cryptography, identity,
//! authorization, process management, or operating-system integration.

#![forbid(unsafe_code)]
