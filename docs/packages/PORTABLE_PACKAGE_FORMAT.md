# ARK Portable Encrypted Package

Status: ACCEPTED product behavior; binary format, exact cryptographic envelope, and migration rules remain OPEN until separately specified and reviewed.

## 1. Goal

A user can copy an encrypted project/workspace package to removable or external storage and import it on another supported system. Copying an already-created encrypted package must not require opening or decrypting the workspace or running the full ARK application.

Proposed extension: .arkvault. The extension is a product proposal, not a finalized format identifier.

## 2. Package types

- Project Package: one project and required project metadata.
- Workspace Package: selected workspace contents and required metadata.
- Recovery Package: recovery wrappers/metadata only as explicitly defined by the recovery specification; never raw master keys.

## 3. Per-package isolation

Each independently created package has:
- a fresh random Package Key;
- a unique package credential/password;
- a unique salt and versioned KDF parameters;
- a package-specific KEK derived with Argon2id;
- an authenticated wrapper that protects the Package Key;
- separated keys for content and metadata through a reviewed derivation scheme.

Do not reuse a package key or package password wrapper across independent packages. A password is not used directly as a file-encryption key.

## 4. Export behavior

Exporting/copying an existing encrypted package does not require the workspace password because the content is already encrypted. Creating a new package requires a package credential to be established/configured so that the receiving installation can unwrap its Package Key. The UX must distinguish package creation from copying an existing package.

The builder should take a consistent snapshot of durable committed state, include required authenticated metadata/journal state, verify the resulting package, and publish it atomically. Do not include volatile uncommitted buffers and do not claim they were exported.

## 5. Import behavior

- Treat package input as hostile.
- Parse with strict length/count/depth/resource limits.
- Verify format version and supported algorithm identifiers.
- Authenticate header, manifest, metadata, and content.
- Reject path traversal, absolute paths, duplicate/conflicting entries, link escapes, and unsupported file types.
- Stage restored content in a private destination.
- Verify all required integrity checks before publishing.
- Use atomic publication or a recoverable staged commit.
- Never overwrite an existing workspace silently.
- Preserve the source package; import is not destructive.
- Report unsupported versions without attempting unsafe downgrade.

## 6. Offline rescue

The package bytes must be copyable without the full GUI/runtime. A future minimal ARK Rescue utility may inspect format/version and integrity or copy the encrypted package without unlocking it. Rescue tooling must not expose plaintext or raw keys by default.

## 7. Password and recovery

The package password is required to import/unlock a newly created package. If forgotten, recovery is possible only if a separately configured supported recovery route exists. Do not promise recovery from password alone. Do not store the password in plaintext or place it beside the package.

## 8. Versioning

The package must carry independent package-format, metadata-schema, crypto-suite, and application compatibility identifiers. Hashes alone do not authenticate content; use authenticated cryptographic integrity. Downgrade and migration behavior must be explicit.
