//! Domain-separated version identifiers.
//!
//! Application, storage, metadata, IPC, policy, recovery, and package versions evolve
//! independently. Distinct types prevent accidental comparison or substitution between
//! domains. This module defines numeric in-memory values only; parsers, encodings, compatibility
//! rules, and migration behavior belong to the subsystem that owns each format.

macro_rules! define_format_version {
    ($name:ident, $doc:literal) => {
        #[doc = $doc]
        #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub struct $name {
            major: u16,
            minor: u16,
        }

        impl $name {
            /// Creates a version value without assigning serialization semantics.
            #[must_use]
            pub const fn new(major: u16, minor: u16) -> Self {
                Self { major, minor }
            }

            /// Returns the major component.
            #[must_use]
            pub const fn major(self) -> u16 {
                self.major
            }

            /// Returns the minor component.
            #[must_use]
            pub const fn minor(self) -> u16 {
                self.minor
            }
        }
    };
}

define_format_version!(
    StorageFormatVersion,
    "Identifies the on-disk encrypted storage format version."
);

define_format_version!(
    MetadataSchemaVersion,
    "Identifies the schema used to interpret ARK metadata."
);

define_format_version!(
    IpcProtocolVersion,
    "Identifies the version of an ARK inter-process communication protocol."
);

define_format_version!(
    PolicySchemaVersion,
    "Identifies the version of a persisted or exchanged policy schema."
);

define_format_version!(
    RecoveryFormatVersion,
    "Identifies the version of a recovery artifact format."
);

define_format_version!(
    PackageFormatVersion,
    "Identifies the version of a portable package format."
);

/// A numeric application release version.
///
/// Pre-release labels and build metadata are intentionally not represented here. Release
/// packaging and user-facing version parsing must define those rules separately.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ApplicationVersion {
    major: u16,
    minor: u16,
    patch: u16,
}

impl ApplicationVersion {
    /// Creates an application version from its numeric components.
    #[must_use]
    pub const fn new(major: u16, minor: u16, patch: u16) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }

    /// Returns the major component.
    #[must_use]
    pub const fn major(self) -> u16 {
        self.major
    }

    /// Returns the minor component.
    #[must_use]
    pub const fn minor(self) -> u16 {
        self.minor
    }

    /// Returns the patch component.
    #[must_use]
    pub const fn patch(self) -> u16 {
        self.patch
    }
}

#[cfg(test)]
mod tests {
    use super::{
        ApplicationVersion, IpcProtocolVersion, PackageFormatVersion, StorageFormatVersion,
    };

    #[test]
    fn format_versions_preserve_their_numeric_components() {
        let storage = StorageFormatVersion::new(2, 7);
        let package = PackageFormatVersion::new(1, 3);

        assert_eq!((storage.major(), storage.minor()), (2, 7));
        assert_eq!((package.major(), package.minor()), (1, 3));
    }

    #[test]
    fn version_domains_have_distinct_types() {
        fn accept_storage_version(_: StorageFormatVersion) {}
        fn accept_ipc_version(_: IpcProtocolVersion) {}

        accept_storage_version(StorageFormatVersion::new(1, 0));
        accept_ipc_version(IpcProtocolVersion::new(1, 0));
    }

    #[test]
    fn application_version_has_an_independent_patch_component() {
        let version = ApplicationVersion::new(3, 4, 5);

        assert_eq!(
            (version.major(), version.minor(), version.patch()),
            (3, 4, 5)
        );
    }
}
