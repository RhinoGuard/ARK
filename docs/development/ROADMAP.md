# ARK Engineering Roadmap

Status: Planning baseline. Order may change only through an explicit decision and updated dependencies.

## Phase 0 — Repository and architecture foundation

- Contributor and AI-agent instructions.
- Threat model, security invariants, architecture map, ADR process.
- Rust toolchain/workspace policy and CI baseline.
- Dependency/license policy and security reporting.
- Requirement-to-test traceability.

## Phase 1 — Core domain contracts

- Typed IDs and domain models.
- Error taxonomy and fail-closed result semantics.
- Version identifiers and narrow traits.
- No OS-specific implementation in core.

## Phase 2 — Cryptographic engine

- Select and review maintained libraries.
- Finalize key hierarchy and context labels.
- Finalize KDF parameters and bounds.
- Finalize nonce allocation and authenticated chunk construction.
- Test vectors, negative tests, fuzzing, independent review.

## Phase 3 — Encrypted storage

- Versioned header and authenticated metadata.
- Chunk index and random-access reads/writes.
- Journal, transactions, checkpoint, atomic metadata publication.
- Crash fault injection and recovery tests.
- Concurrency and durability contract.

## Phase 4 — Identity, policy, sessions, and broker

- Platform identity evidence.
- Default-deny policy evaluation.
- Scoped/expiring sessions and revocation.
- Authenticated IPC and bounded parsing.
- Independent security review.

## Phase 5 — Development Runtime

- Bounded active working set.
- Editor save-burst behavior.
- Encrypted durable journal.
- Handle/mapping lifecycle.
- Recovery and plaintext-exposure assessment.

## Phase 6 — Execution Runtime and filesystem mediation

- Windows-first real filesystem prototype.
- Directory/resource scopes and execution profiles.
- Process/child-process identity behavior.
- Filesystem semantics and compatibility matrix.
- PHP/Apache integration.

## Phase 7 — Portable packages and recovery

- Project/workspace package specification.
- Per-package key and credential wrapping.
- Safe import and atomic restore.
- Offline copy/rescue workflow.
- Recovery routes and rotation.

## Phase 8 — Desktop UI and product delivery

- Tauri/Rust/TypeScript prototype.
- UI as an untrusted control surface.
- Installer, signed update, rollback, repair, and data-preserving uninstall.
- Audit viewer and user-facing security warnings.

## Phase 9 — Platform expansion and release hardening

- Linux backend and compatibility validation.
- macOS backend and compatibility validation.
- Fuzzing, SAST, dependency review, SBOM, reproducible/controlled build work.
- Independent security review and release gate.

## MVP acceptance target

A real PHP project can be imported into encrypted storage, edited through VS Code using Development Mode, served through Apache using a scoped Execution Mode, recovered after supported crash scenarios, exported as a portable encrypted package, and imported on another supported machine.

A roadmap item is not complete until implementation, tests, documentation, and review evidence exist.
