# ADR-0004: Portable Encrypted Packages

- **Status:** Accepted (product model)
- **Implementation status:** Not yet implemented

## Context
Users must be able to copy encrypted workspace data to another machine even if the source ARK installation cannot run. Copying must not require unlocking plaintext or entering the Workspace password.

## Decision
Provide self-contained, versioned portable encrypted packages (working extension: `.arkvault`). Each package uses a random independent Package Key protected by a package-specific password-derived KEK (Argon2id). The package password differs from the Workspace password. Package data is authenticated/encrypted using a reviewed format derived from the Package Key.

Export operates on a consistent durable encrypted snapshot. Import requires the package password and verifies/stages all content before atomic publication.

## Consequences
- One package's password/key must not unlock other packages.
- Package password change should rewrap Package Key rather than re-encrypt all data, if the selected format supports it.
- Package/recovery formats require independent versioning and migration.
- Lost package password may make the package unrecoverable if no valid recovery path exists.
- A planned ARK Rescue utility may inspect/verify/copy encrypted bytes without decrypting them.

## Verification
Test offline export, snapshot consistency, tampered package rejection, wrong password, path traversal, resource exhaustion, interrupted import, cross-platform restore, and package isolation.
