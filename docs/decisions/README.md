# Architecture Decision Records

Material architecture and security decisions are recorded here. ADRs preserve context, alternatives, consequences, status, and review history. Do not silently change an accepted decision in code.

## Status vocabulary
- **Proposed:** under discussion; not binding.
- **Accepted:** agreed direction; implementation may still be pending.
- **Superseded:** replaced by a later ADR.
- **Rejected:** considered and not selected.

## Required ADR topics
The following are planned records; do not fabricate an accepted decision where implementation details remain open:
1. Rust modular workspace and dependency boundaries.
2. Cryptographic library and algorithm selection.
3. Key hierarchy and password/recovery wrappers.
4. Chunked/streaming encryption protocol and nonce strategy.
5. Encrypted storage format, WAL, and durability.
6. Development Mode working-set model.
7. Execution Mode process identity and scope grants.
8. Filesystem mediation backend per OS.
9. Portable package format and independent package password.
10. Recovery Key, Recovery File, and trusted-device lifecycle.
11. Versioning and migration policy.
12. Desktop UI and privileged process model.

## ADR template

Create files as `ADR-NNNN-short-title.md` with:
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
