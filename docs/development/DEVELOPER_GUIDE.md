# Developer Guide

Read this document with the repository-root AGENTS.md. This is the onboarding entry point for contributors and AI coding assistants.

## Repository status

The initial Cargo workspace and the platform-independent ark-core foundation are implemented in the foundation branch. The core currently contains typed domain identifiers, runtime-mode vocabulary, independent version-domain types, and a sanitized shared error taxonomy.

The runtime, cryptographic engine, encrypted storage, identity/session/policy/broker services, filesystem mediation, platform backends, package/recovery engines, and user-facing applications remain unimplemented. Design documents are not evidence of implementation.

## First implementation milestones

1. Review and accept the initial crate boundary and dependency proposal.
2. Complete owner-reviewed cryptographic API, library, key hierarchy, and nonce decisions.
3. Specify and prototype encrypted storage format, journal, transaction ordering, and recovery.
4. Define identity, session, policy, and broker interfaces with negative-test coverage.
5. Prototype Windows process identity and real filesystem mediation.
6. Implement Development Mode working-set lifecycle through broker-controlled access.
7. Implement scoped Execution Mode and validate the PHP/Apache MVP.
8. Integrate portable package and recovery workflows.
9. Integrate desktop UI after core contracts stabilize.
10. Add Linux and macOS backends incrementally.

Do not treat this sequence as permission to implement another owner's cryptographic, threat-model, policy, or verification decisions independently.

## Current workspace structure

- ark-core: implemented platform-independent IDs, runtime mode, versions, and safe error vocabulary.
- ark-crypto: planned; cryptographic construction and API decisions are owned by the cryptography workstream.
- ark-storage: planned; encrypted format, journal, transactions, and recovery.
- ark-identity, ark-session, ark-policy, ark-broker: planned security and authorization boundaries.
- ark-filesystem, ark-runtime, platform backends: planned systems/runtime boundaries.
- ark-package, ark-recovery, ark-audit: planned supporting subsystems.
- apps: planned composition roots and user workflows.

See docs/architecture/CRATE_BOUNDARIES_AND_DEPENDENCIES.md before adding a crate or dependency.

## Core crate contract

ark-core must remain independent of concrete OS, GUI, storage, crypto, and authorization implementations. Its IDs are not credentials; RuntimeMode is not permission; version values do not define a file encoding; and ArkError contains no caller-controlled diagnostics or secret material.

Do not add serde, UUID generation, async runtimes, logging frameworks, crypto libraries, or platform SDKs to ark-core without a concrete requirement and an owner-reviewed decision.

## Validation gates

Run the checks available for the change:
- cargo fmt --all -- --check
- cargo build --workspace --all-targets --all-features --locked
- cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
- cargo test --workspace --all-features --locked

CI runs the baseline checks on Windows, Linux, and macOS. These checks do not replace dependency review, fuzzing, platform integration tests, fault injection, or independent security review. Report only commands actually run and results actually observed.

## Required engineering loop

For each change:
1. State the behavior and threat assumptions.
2. Identify affected invariants and trust boundaries.
3. Inspect existing APIs and tests.
4. Make the smallest coherent change.
5. Add positive and negative tests.
6. Run formatting, linting, tests, and relevant security/fuzz checks.
7. Update docs, ADRs, and compatibility notes.
8. Request independent review for security-sensitive code.
9. Report exact commands and results.

## No-go conditions

Do not merge changes that:
- bypass authorization or expose raw keys;
- introduce unreviewed cryptography;
- turn unknown identity or policy errors into allow;
- trust paths, PIDs, or process names as sole identity;
- silently alter persistent formats or key hierarchy;
- claim complete application mediation without a platform enforcement mechanism;
- lose committed data under supported crash-recovery guarantees;
- add secrets, real user data, or generated artifacts to source control.
