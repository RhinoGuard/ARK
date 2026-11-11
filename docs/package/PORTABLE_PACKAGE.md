# Portable Encrypted Package

## Goal
Allow users to copy an encrypted Workspace/Project package to another system even if the source ARK installation is broken, then install ARK elsewhere and import it. Copying the encrypted package must not require opening the Workspace or entering its Workspace password.

## Package protection
- Each package has a random independent Package Key.
- A package-specific password is distinct from the Workspace password.
- Argon2id derives a package KEK from the package password.
- The KEK wraps the Package Key.
- Package data is encrypted/authenticated under keys derived from the Package Key using a reviewed format.
- A password change rewraps the Package Key; it need not re-encrypt all payload data if format design permits.
- One package's password/key must not unlock other packages.

The exact KDF parameters, serialization, key derivation context, AEAD streaming format, and versioning require an ADR. Do not store raw Package Key or Workspace Master Key in plaintext.

## Offline export
Export reads a consistent durable encrypted snapshot without unlocking file contents. Snapshot must include all committed data and the journal state needed for a consistent restore. Export should be atomic and clearly distinguish incomplete output from success.

## Import
Import on another machine requires the package password. Parse under strict size/resource limits, validate version and authenticated metadata, verify all package objects, reject path traversal and duplicate/conflicting identities, stage output, and atomically publish only after verification.

## Package types
- Project Package.
- Workspace Package.
- Recovery Package containing valid wrappers and metadata, never raw master keys.

## ARK Rescue
A planned lightweight independent utility may inspect, verify, and copy encrypted vault/package bytes without opening plaintext. It must not become a bypass for decryption or policy. Its parser and supported versions require independent testing.

## Loss warning
If package password is lost and no valid recovery path exists, package data may be unrecoverable. Warn users before export and require them to verify restore readiness.
