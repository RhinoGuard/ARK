# ADR-0003: Workspace Recovery Paths

- **Status:** Accepted (product model)
- **Implementation status:** Not yet implemented

## Context
A password-only recovery design risks permanent loss. Recovery must not rely solely on an account identity provider or store raw master keys on a server.

## Decision
Support three recovery paths:
1. Random independent 256-bit Recovery Key per Workspace.
2. Recovery File containing authenticated wrapped key material and required metadata, never a raw Workspace Master Key. The file is a protected wrapper/artifact, not an independent cryptographic factor.
3. Trusted Device with its own private key and proof of possession.

Google/OIDC account authentication may identify an account but does not itself unlock Workspace keys.

## Consequences
- Recovery creation, test, rotation, revocation, and audit are first-class workflows.
- Recovery Key and Trusted Device lifecycle need independent protocols and tests.
- If password, Recovery Key, Recovery File, and all trusted devices are lost, zero-knowledge recovery cannot be guaranteed without a separately designed escrow mechanism.
- Enterprise escrow is a future feature requiring a separate threat model.

## Verification
Test recovery from each path, invalid/revoked material, interrupted rotation, metadata tampering, device loss, and complete-loss messaging.
