# Sensitive Operations and Step-up Authentication

## Classification
Sensitive operations are assigned policy levels, not separate encryption passwords.

- **Standard:** routine operations within a valid session.
- **Elevated:** export, import, permission changes, sensitive settings, Recovery File operations.
- **Critical:** Recovery Key rotation, primary key changes, workspace deletion, ownership changes, and revocation of a primary trusted device.

The policy engine is authoritative. UI labels and client-side checks are not enforcement.

## Authentication versus intent
Step-up authentication proves the required credential/device control. A typed workspace/project name, summary screen, or confirmation phrase is a separate proof-of-intent measure; it is not authentication.

## Export
Export requires a consistent snapshot, complete authentication/integrity verification, safe path handling, staging output, and atomic publish. An incomplete export is never reported as success. Once plaintext is exported, ARK no longer controls that copy.

## Import
Import parses under strict resource limits, validates all authenticated metadata and objects, rejects unsafe paths and conflicts, stages data, then atomically publishes only after full verification.

## Recovery and key changes
Recovery operations must preserve at least one valid path until the replacement has been verified and durably committed. Rotation/revocation must be crash-safe and auditable. Never log secrets.

## Deletion
Deletion must clearly distinguish deleting a workspace reference, encrypted storage, backups, exported copies, and OS-managed remnants. Do not promise secure erasure from SSDs, snapshots, backups, or privileged OS caches without evidence.
