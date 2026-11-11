# Architecture Decision Records

Material architecture and security decisions are recorded here. ADRs preserve context, alternatives, consequences, status, and review history. Do not silently change an accepted decision in code.

## Status vocabulary

- **Proposed:** under discussion; not binding.
- **Accepted:** agreed direction; implementation may still be pending.
- **Superseded:** replaced by a later ADR.
- **Rejected:** considered and not selected.

## Current records

- ADR-0001-rust-modular-workspace.md — accepted modular Rust direction.
- ADR-0002-runtime-modes.md — accepted separation of Development and Execution Modes.
- ADR-0002-runtime-mode-separation.md — additional accepted record for the same runtime-mode decision.
- ADR-0003-crypto-and-key-hierarchy.md — accepted primitive-family and key-hierarchy direction; exact constructions remain gated.
- ADR-0003-recovery-paths.md — accepted three-route recovery product model.
- ADR-0004-portable-encrypted-packages.md — portable package behavior.
- ADR-0005-core-foundation.md — proposed Cargo workspace and core domain foundation.

## Registry hygiene issue

The repository currently contains two files numbered ADR-0002 and two files numbered ADR-0003. The ADR identifiers therefore collide. This foundation change does not silently renumber or alter the existing runtime, cryptography, or recovery decisions. Until a dedicated owner-reviewed cleanup is merged, cite the full filename rather than the number alone. The records should be reconciled, assigned unique identifiers, and all references updated in a separate documentation change.

## Required ADR topics

The following are still required where implementation detail has not yet been accepted:
1. Cryptographic library and algorithm selection.
2. Key hierarchy and password/recovery wrappers.
3. Chunked/streaming encryption protocol and nonce strategy.
4. Encrypted storage format, WAL, and durability.
5. Development Mode working-set model.
6. Execution Mode process identity and scope grants.
7. Filesystem mediation backend per OS.
8. Recovery Key, Recovery File, and trusted-device lifecycle.
9. Versioning and migration policy.
10. Desktop UI and privileged process model.

## ADR template

Create files as ADR-NNNN-short-title.md with:
- Title and status.
- Date and decision owners/reviewers.
- Context and problem statement.
- Security requirements/invariants.
- Considered options.
- Decision and rationale.
- Consequences and trade-offs.
- Threat assumptions and residual risks.
- Implementation and test plan.
- Compatibility/migration impact.
- References.
