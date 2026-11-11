# ARK Developer Guide

This is the entry point for engineers and AI agents contributing to ARK.

## Start here

1. Read AGENTS.md for mandatory engineering and AI-agent rules.
2. Read docs/README.md for the documentation map and decision status.
3. Read docs/architecture/ARCHITECTURE.md for subsystem boundaries.
4. Read docs/architecture/CRATE_BOUNDARIES_AND_DEPENDENCIES.md before adding a crate or dependency.
5. Read docs/architecture/SECURITY_INVARIANTS.md and docs/security/THREAT_MODEL.md.
6. Read the relevant subsystem specification and ADRs.
7. Inspect current code and tests; documentation describes intent and may be ahead of implementation.

## Development principles

- Security by default; deny by default; fail closed.
- Identity verification is distinct from authorization.
- UI is presentation and user intent capture, never a security authority.
- Protected storage is accessed only through controlled, authorized interfaces.
- Use stable logical IDs rather than caller-controlled paths as authority.
- Treat all IPC, filesystem metadata, package data, and persisted formats as untrusted input.
- Make crash consistency, concurrency, revocation, and migration first-class design concerns.
- Optimize only after correctness and invariants are testable.
- Keep platform-specific behavior behind narrow backend interfaces.

## Current implementation status

The repository now contains a minimal Rust Cargo workspace and the platform-independent ark-core foundation. The core defines typed domain identifiers, runtime-mode vocabulary, independent version-domain types, and a sanitized shared error taxonomy.

No runtime engine, cryptographic engine, encrypted storage, identity/session/policy/broker service, filesystem mediation backend, package/recovery engine, GUI, or installer is implemented yet. The specifications define target behavior; they do not imply that those controls exist.

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
