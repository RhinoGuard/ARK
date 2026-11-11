# Recovery Design

## Recovery paths
Each Workspace is designed to support three recovery paths:
1. **Recovery Key:** random independent 256-bit CSPRNG secret per Workspace.
2. **Recovery File:** authenticated wrapper and required metadata; it is a protected recovery artifact, not an independent cryptographic factor and never contains a raw master key.
3. **Trusted Device:** device-specific private key with proof of possession.

## Recovery Key lifecycle
Show the Recovery Key at workspace creation and require a deliberate acknowledgement/challenge that encourages safe offline storage. Do not send or store the raw key on a server. Exact encoding, checksum, derivation/wrapping protocol, and rotation semantics require an ADR.

## Recovery File
The file contains wrapped key material and metadata needed to locate/validate the wrapper. It must be authenticated, versioned, and bound to the intended Workspace. It must not contain a raw Workspace Master Key. Export/import and corruption cases require tests.

## Trusted Devices
Each trusted device has its own private key. Device registration, proof-of-possession, rotation, revocation, backup/restore, and loss handling require explicit protocols. A device name is display metadata, not identity proof.

## Rotation and revocation
For Recovery Key rotation, create and verify a new wrapper, atomically commit it, then revoke the old wrapper according to a crash-safe protocol. A failed rotation must not destroy the only working recovery path. Trusted-device revocation updates authorization state and is auditable.

## Recovery Center
Planned user workflows: view available recovery paths, export/verify Recovery File, test recovery without destructive mutation, rotate Recovery Key, register/revoke trusted devices, and view security events.

## Loss conditions
If the password, Recovery Key, Recovery File, and all trusted devices are lost, and no separately designed escrow exists, zero-knowledge recovery cannot be guaranteed. Tell users clearly. Enterprise escrow is a future, separately threat-modeled feature, not an implicit backdoor.

## Account identity
Google/OIDC may authenticate an account identity; it does not itself unwrap Workspace keys. OIDC Authorization Code with PKCE, state, nonce, issuer/audience/expiry/signature validation, and minimal scopes are required if account login is implemented. Google subject identifier (`sub`) is the stable external identity; email is an attribute, not a primary key.
