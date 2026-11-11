# Developer Guide

Read this document with [AGENTS.md](../../AGENTS.md). It is the onboarding entry point for contributors and AI coding assistants.

## Repository status
This branch establishes architecture and contributor guidance. A Rust workspace and runtime are not yet established. The design below is not evidence of implementation.

## First implementation milestone
1. Establish Cargo workspace metadata, toolchain policy, formatting/lint configuration, CI, and docs conventions.
2. Implement core domain IDs, operation models, version types, and typed errors.
3. Select maintained crypto dependencies through review and ADR.
4. Implement/test authenticated encryption and key derivation boundaries.
5. Specify encrypted storage format.
6. Implement chunk operations and authenticated metadata.
7. Add encrypted WAL, durable commit ordering, crash recovery, and fault injection.
8. Add process identity, policy, session, and broker.
9. Implement Development Mode working-set lifecycle.
10. Implement scoped Execution Mode.
11. Prototype/evaluate Windows filesystem mediation.
12. Implement portable package and recovery.
13. Integrate desktop UI after core contracts stabilize.
14. Add Linux and macOS backends incrementally.

## Proposed module responsibilities
| Area | Responsibility |
|---|---|
| ark-core | Domain IDs/models/errors/contracts/versioning |
| ark-crypto | AEAD, KDF, HKDF, key hierarchy, nonce/AAD |
| ark-storage | Format, manifest, chunks, WAL, transactions, snapshots |
| ark-identity | Process/executable/device/user identity evidence |
| ark-policy | Default-deny policy evaluation and scopes |
| ark-session | Session lifecycle, binding, expiry, revocation |
| ark-broker | Authorization coordination and access boundary |
| ark-runtime | Development and Execution Mode |
| ark-filesystem | Common filesystem semantics |
| ark-package | Portable encrypted package |
| ark-recovery | Recovery Key/File, trusted devices, rotation |
| ark-audit | Security events without secrets |
| platform/* | OS-specific identity, IPC, filesystem, key store |
| apps/* | CLI, desktop, rescue utilities |

Do not create every proposed crate as an empty placeholder. Keep dependency direction acyclic. Core must not depend on concrete platform/UI. Runtime must not bypass policy. Platform adapters do not own product policy.

## Validation gates
As tooling becomes available:
- `cargo fmt --all -- --check`
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- `cargo test --workspace --all-features`
- Dependency, license, and security checks.
- Fuzz parsers, file formats, IPC decoders, and untrusted-input surfaces.
- Integration tests for allow/deny behavior.
- Fault injection and crash-recovery tests.
- Cross-platform tests for every claimed backend.

These are future gates until the workspace exists. Report only commands actually run.

## Team review
Offensive-security review participates from design through implementation. Sensitive modules require independent review; no author self-approval. Keep changes focused and reviewable.

## Documentation
Use descriptive uppercase filenames for major design docs, numbered ADRs for material decisions, explicit unresolved questions, and separate design status from implementation status.
