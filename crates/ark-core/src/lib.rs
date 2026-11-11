//! Platform-independent domain primitives shared by ARK subsystems.
//!
//! This crate is the dependency-free foundation of the ARK domain model. Its initial public
//! surface is intentionally small and does not implement storage, cryptography, identity,
//! authorization, process management, or operating-system integration.
//!
//! Domain identifiers and workflow types describe values; they do not establish identity,
//! confer authority, or grant access.

#![forbid(unsafe_code)]

pub mod ids;
pub mod runtime;
