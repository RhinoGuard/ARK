# Architecture Decision Records

ADRs preserve why an architectural decision was made. They are part of the engineering contract for human contributors and AI agents.

## Status values

- Accepted: agreed direction; implementation details may still need validation.
- Proposed: requires explicit approval.
- Open: unresolved.
- Superseded: replaced by a later ADR.
- Rejected: considered and not selected.

## Required ADR sections

1. Title and status.
2. Context/problem.
3. Decision.
4. Alternatives considered.
5. Consequences and trade-offs.
6. Security implications and affected invariants.
7. Validation/tests required.
8. Follow-up and revisit conditions.

Do not edit an accepted ADR to silently change its meaning. Create a superseding ADR and link both documents.

## Initial ADR index

- ADR-0001: Rust modular workspace and cross-platform strategy.
- ADR-0002: Development Mode and Execution Mode separation.
- ADR-0003: Cryptographic primitive families and key hierarchy.
- ADR-0004: Encrypted random-access chunk storage and journal requirements.
- ADR-0005: Identity, policy, session, and broker boundaries.
- ADR-0006: Filesystem mediation and platform backend strategy.
- ADR-0007: Portable encrypted package behavior.
- ADR-0008: Recovery routes and operation security levels.
- ADR-0009: Desktop UI direction.
