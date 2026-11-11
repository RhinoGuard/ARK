# Versioning and Migration Policy

ARK version dimensions are independent. Never overload the application version to interpret protected data.

## Independent version identifiers
- Application Version.
- Storage Format Version.
- Crypto Suite ID.
- Metadata Schema Version.
- IPC Protocol Version.
- Policy Schema Version.
- Recovery Format Version.
- Package Format Version.

Each dimension needs an explicit compatibility, migration, deprecation, and downgrade policy.

## Data headers
Protected data must identify the format/crypto versions required to interpret it. IDs must map to a fixed, reviewed implementation; never accept arbitrary algorithm names or silently negotiate a weaker suite. Authenticate version and interpretation-critical metadata.

## Migration
Migration must be staged, integrity-verified, recoverable, and atomic. Preserve the original until the new representation has been fully validated and durably committed. Interrupted migration must resume safely or roll back without destroying the only valid copy.

## Downgrade
Do not silently reinterpret newer data using older code. Reject unsupported versions clearly. Downgrade paths require explicit compatibility rules and anti-rollback consideration.

## Compatibility matrix
Maintain a tested matrix across application versions, storage formats, crypto suites, package formats, and recovery formats. Update it whenever a format or migration changes.
