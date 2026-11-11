//! Safe, low-detail errors shared by ARK domain components.
//!
//! The shared error type carries a fixed code only. It deliberately does not store arbitrary
//! strings, paths, request payloads, secrets, or operating-system error messages. Subsystems may
//! maintain richer private diagnostics, but anything shown to a caller or written to a log must
//! be sanitized by that subsystem.

use core::fmt;

/// A stable, coarse classification of a domain or subsystem failure.
///
/// The code is not a wire-protocol value. IPC and persisted formats must define their own
/// explicitly versioned error representation.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum ErrorCode {
    /// An argument violates a documented domain constraint.
    InvalidArgument,
    /// An identifier or identifier representation is invalid.
    InvalidIdentifier,
    /// A supplied version is malformed or outside the supported range.
    InvalidVersion,
    /// A requested lifecycle or domain transition is not permitted.
    InvalidStateTransition,
    /// The requested entity does not exist or is not visible in the current context.
    NotFound,
    /// The operation conflicts with the current domain state.
    Conflict,
    /// Required authentication evidence is absent or invalid.
    Unauthenticated,
    /// The operation is not authorized by the applicable policy.
    AccessDenied,
    /// A bounded authorization or session lifetime has elapsed.
    Expired,
    /// The referenced session, grant, or credential has been revoked.
    Revoked,
    /// The requested feature or format is not supported.
    Unsupported,
    /// An integrity or authenticity check failed.
    IntegrityFailure,
    /// A documented resource limit was reached.
    ResourceExhausted,
    /// A required dependency or platform capability is unavailable.
    Unavailable,
    /// The operation was cancelled before completion.
    Cancelled,
    /// An internal failure occurred; sensitive implementation details are not exposed.
    Internal,
}

/// A sanitized error suitable for crossing ARK subsystem boundaries.
///
/// This type intentionally omits arbitrary diagnostic text. Use ErrorCode for programmatic
/// handling and keep any lower-level source error inside the subsystem that owns it.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[must_use]
pub struct ArkError {
    code: ErrorCode,
}

impl ArkError {
    /// Creates a sanitized error from a fixed classification code.
    pub const fn new(code: ErrorCode) -> Self {
        Self { code }
    }

    /// Returns the fixed classification code.
    #[must_use]
    pub const fn code(self) -> ErrorCode {
        self.code
    }
}

impl fmt::Display for ArkError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self.code {
            ErrorCode::InvalidArgument => "invalid argument",
            ErrorCode::InvalidIdentifier => "invalid identifier",
            ErrorCode::InvalidVersion => "invalid version",
            ErrorCode::InvalidStateTransition => "invalid state transition",
            ErrorCode::NotFound => "resource not found",
            ErrorCode::Conflict => "operation conflicts with current state",
            ErrorCode::Unauthenticated => "authentication required or invalid",
            ErrorCode::AccessDenied => "access denied",
            ErrorCode::Expired => "authorization or session expired",
            ErrorCode::Revoked => "authorization or credential revoked",
            ErrorCode::Unsupported => "operation or format not supported",
            ErrorCode::IntegrityFailure => "integrity check failed",
            ErrorCode::ResourceExhausted => "resource limit exceeded",
            ErrorCode::Unavailable => "required capability unavailable",
            ErrorCode::Cancelled => "operation cancelled",
            ErrorCode::Internal => "internal error",
        };

        formatter.write_str(message)
    }
}

impl std::error::Error for ArkError {}

/// The standard result type for platform-independent ARK domain operations.
pub type CoreResult<T> = Result<T, ArkError>;

#[cfg(test)]
mod tests {
    use super::{ArkError, CoreResult, ErrorCode};

    #[test]
    fn error_code_survives_safe_wrapping() {
        let error = ArkError::new(ErrorCode::IntegrityFailure);

        assert_eq!(error.code(), ErrorCode::IntegrityFailure);
        assert_eq!(error.to_string(), "integrity check failed");
    }

    #[test]
    fn error_display_does_not_depend_on_caller_supplied_text() {
        let error = ArkError::new(ErrorCode::AccessDenied);
        let rendered = error.to_string();

        assert_eq!(rendered, "access denied");
        assert!(!rendered.contains("password"));
        assert!(!rendered.contains("secret"));
    }

    #[test]
    fn core_result_uses_the_shared_error_contract() {
        let result: CoreResult<u8> = Err(ArkError::new(ErrorCode::Cancelled));

        assert_eq!(result.err().map(ArkError::code), Some(ErrorCode::Cancelled));
    }
}
