//! Platform-independent domain primitives shared by ARK subsystems.
//!
//! This crate intentionally has no third-party dependencies and contains no operating-system,
//! cryptographic, persistence, or authorization implementation. Its types describe domain
//! vocabulary; they do not grant access or prove identity.
//!
//! # Dependency contract
//!
//! The ark-core crate is the bottom of the Rust crate dependency graph. It may use the Rust
//! standard library, but it must not depend on another ARK crate, an operating-system API, a GUI
//! framework, or a concrete cryptographic implementation.

#![forbid(unsafe_code)]

pub mod errors;
pub mod ids;
pub mod runtime;
pub mod versions;

pub use errors::{ArkError, CoreResult, ErrorCode};
