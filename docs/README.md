# ARK Documentation Index

This documentation is the shared engineering baseline for human contributors and AI coding agents.

## Read order

1. Repository-root AGENTS.md — mandatory rules for developers and AI agents.
2. Repository-root DEVELOPMENT.md — developer workflow.
3. architecture/ARCHITECTURE.md — system boundaries and data flows.
4. architecture/SECURITY_INVARIANTS.md — properties that implementations must preserve.
5. security/THREAT_MODEL.md — assets, actors, trust boundaries, threats, and explicit limitations.
6. Relevant subsystem specifications.
7. decisions/ — accepted and proposed Architecture Decision Records.

## Subsystem specifications

- cryptography/CRYPTOGRAPHY.md — cryptographic goals, key hierarchy, chunk requirements, and open design gates.
- storage/STORAGE_AND_RECOVERY.md — encrypted storage, working sets, journal, transactions, and crash consistency.
- runtime/RUNTIME_MODES.md — Development Mode and Execution Mode.
- packages/PORTABLE_PACKAGE_FORMAT.md — portable encrypted export/import.
- recovery/RECOVERY.md — Recovery Key, Recovery File, and Trusted Device.

## Decision status vocabulary

- ACCEPTED: explicitly agreed architectural direction; implementation details may still require validation.
- PROPOSED: candidate direction requiring review/approval.
- OPEN: unresolved; do not implement an assumption as a settled contract.
- DEFERRED: intentionally postponed.
- SUPERSEDED: replaced by a later ADR.

Acceptance of an architectural direction does not mean the feature is implemented, audited, or production-ready.

## Project-wide decisions currently treated as accepted

- Rust is the primary implementation language.
- Cross-platform target: Windows, Linux, and macOS; implementation order is Windows, Linux, then macOS.
- Modular Rust workspace with OS-specific backends; avoid premature process separation.
- Development Mode and Execution Mode have distinct access models.
- Development Mode uses a limited active working set backed by encrypted durable storage and a crash-recoverable journal design.
- Execution Mode grants explicit resource/directory scopes rather than attempting to infer every dynamic source dependency.
- Identity verification and authorization are separate; default deny.
- UI is not a security authority.
- Cryptography uses established constructions and maintained libraries; no home-grown algorithms.
- XChaCha20-Poly1305, Argon2id, HKDF-SHA-256, and OS CSPRNG are the selected primitive families, subject to final library/version/parameter review.
- Chunked random-access authenticated storage is required; exact format, chunk sizing, and nonce allocation remain engineering gates.
- Recovery has three planned routes: independent per-workspace Recovery Key, Recovery File containing protected recovery wrappers/metadata, and Trusted Device with a device-specific private key.
- Portable packages are encrypted and independently credentialed per package. Exporting/copying already encrypted package bytes does not require decrypting the workspace.
- Standard, Elevated, and Critical classify operation sensitivity and step-up requirements; they are not separate encryption strengths.
- Target GUI direction: Tauri with Rust backend and TypeScript frontend; final capability model and implementation are still subject to review.
- MVP target: a real PHP project edited through VS Code and served by Apache using controlled ARK runtime access.

## Important non-guarantees

- No promise of zero bugs or universal compatibility.
- No protection guarantee against a fully compromised OS/kernel or privileged attacker.
- No guarantee that plaintext already delivered to an authorized application cannot be copied or exfiltrated.
- No claim that deleting a temporary plaintext file securely erases all copies from SSDs, caches, swap, snapshots, or crash dumps.
- No claim that a shutdown hook can guarantee data persistence during sudden power loss.
