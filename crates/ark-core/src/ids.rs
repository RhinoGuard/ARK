//! Strongly typed identifiers for ARK domain entities.
//!
//! Identifier values are labels, not credentials, capabilities, or authorization evidence.
//! This crate does not generate identifiers. Generators belong in the appropriate subsystem
//! and must use an approved source of randomness when unpredictability is required.
//!
//! The 16-byte in-memory representation is not a persistence or wire-format decision. Any
//! serialized representation must be specified and versioned by the subsystem that owns it.

use core::fmt;

macro_rules! define_id {
    ($name:ident, $doc:literal) => {
        #[doc = $doc]
        #[derive(Clone, Copy, Eq, Hash, PartialEq)]
        pub struct $name([u8; 16]);

        impl $name {
            /// The fixed width of this identifier in bytes.
            pub const WIDTH: usize = 16;

            /// Creates an identifier from its fixed-width byte representation.
            #[must_use]
            pub const fn from_bytes(bytes: [u8; 16]) -> Self {
                Self(bytes)
            }

            /// Returns the fixed-width byte representation.
            #[must_use]
            pub const fn as_bytes(&self) -> &[u8; 16] {
                &self.0
            }

            /// Consumes the identifier and returns its fixed-width byte representation.
            #[must_use]
            pub const fn into_bytes(self) -> [u8; 16] {
                self.0
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                for byte in self.0.iter() {
                    write!(formatter, "{byte:02x}")?;
                }
                Ok(())
            }
        }

        impl fmt::Debug for $name {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(formatter, "{}({self})", stringify!($name))
            }
        }
    };
}

define_id!(
    WorkspaceId,
    "Identifies one ARK workspace. It is not proof of workspace ownership or authorization."
);

define_id!(
    ProjectId,
    "Identifies one project within the ARK domain model."
);

define_id!(
    FileId,
    "Identifies a file independently of caller-controlled path strings."
);

define_id!(
    SessionId,
    "Identifies a session. The identifier alone does not prove that the session is valid."
);

define_id!(
    DeviceId,
    "Identifies a registered device. The identifier alone is not device identity evidence."
);

define_id!(
    PackageId,
    "Identifies one portable package independently of its user-visible filename."
);

define_id!(
    TransactionId,
    "Identifies one logical transaction. It does not imply that the transaction committed."
);

#[cfg(test)]
mod tests {
    use super::{FileId, PackageId, WorkspaceId};

    #[test]
    fn identifier_bytes_round_trip_without_transformation() {
        let bytes = [
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d,
            0x0e, 0x0f,
        ];
        let identifier = WorkspaceId::from_bytes(bytes);

        assert_eq!(identifier.as_bytes(), &bytes);
        assert_eq!(identifier.into_bytes(), bytes);
    }

    #[test]
    fn identifier_display_is_fixed_width_lowercase_hex() {
        let identifier = WorkspaceId::from_bytes([0xab; WorkspaceId::WIDTH]);

        assert_eq!(
            identifier.to_string(),
            "abababababababababababababababab"
        );
        assert_eq!(
            format!("{identifier:?}"),
            "WorkspaceId(abababababababababababababababab)"
        );
    }

    #[test]
    fn different_identifier_domains_remain_distinct_types() {
        let bytes = [0x42; 16];
        let package = PackageId::from_bytes(bytes);
        let file = FileId::from_bytes(bytes);

        assert_eq!(package.to_string(), file.to_string());
        // These values have the same bytes but cannot be passed interchangeably to APIs that
        // require PackageId or FileId respectively; the wrapper types enforce that at compile time.
    }
}
