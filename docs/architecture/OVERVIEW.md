# ARK Architecture Overview

## Product
ARK is a cross-platform Secure Workspace for developers. It stores project data in encrypted form and mediates access by developer tools/runtime applications through explicit identity, session, and policy checks.

The implementation direction is modular Rust with OS-specific backends. Separate processes are introduced only where they create a meaningful security or reliability boundary.

## Components
- Core domain: IDs, models, typed errors, versioned contracts.
- Crypto: key derivation, authenticated encryption, wrapping, nonce/AAD.
- Storage: manifests, chunks, encrypted journal, transactions, snapshots, integrity.
- Identity: process, executable, user, device, and session evidence.
- Policy: default-deny authorization over identity, resource, operation, and time.
- Session: bounded lifecycle, binding, expiry, revocation.
- Broker: controlled authorization/access coordination.
- Runtime: Development and Execution Mode.
- Filesystem: common semantics with OS-specific backends.
- Package/recovery: portable encrypted packages and recovery.
- Audit: security events without secrets or plaintext.
- CLI/desktop/rescue: user workflows; never authoritative for security decisions.

## Trust boundaries
1. UI to Rust core: inputs are untrusted; core validates.
2. External process to broker/backend: verify process identity and IPC peer credentials.
3. Broker to policy: authorization inputs must be complete, fresh, and request-bound.
4. Runtime to storage: durable writes obey authenticated transactional rules.
5. Platform backend to OS: assumptions are documented and tested.
6. Workspace to export: export crosses the workspace protection boundary.
7. Recovery artifact to key hierarchy: no raw master keys in artifacts.

## Development Mode
Expose only files needed for active editing, not automatically the entire workspace. Durable state remains encrypted. Working data may exist in RAM or OS-managed temporary locations. Closing a tab does not prove handles, mappings, indexers, children, or async writes have ended.

The target model is encrypted-backed working data with bounded buffers and durable encrypted journaling. Commit durability must not depend on long debounce timers or shutdown callbacks.

## Execution Mode
The user grants a defined resource scope and duration to an authorized application identity. The app requests files dynamically. A directory grant exposes every covered file to that application during the grant. Communicate this clearly.

Authorized applications can copy/transmit plaintext. Filesystem authorization alone is not DLP.

## Platform strategy
Target Windows, Linux, macOS in that order. Evaluate native filesystem mediation; candidate technologies include WinFsp, FUSE, and current Apple filesystem/provider mechanisms. Prototype each and publish a supported-operation matrix. Do not assume full OS-level mediation.

## Limits
No protection promise against compromised kernel, malicious privileged administrator, or malicious authorized process. Do not promise all OS access paths are mediated or that revocation instantly invalidates existing handles/mappings.
