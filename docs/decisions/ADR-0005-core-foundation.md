# ADR-0005: Initial Rust Workspace and Core Domain Foundation

- **Status:** Proposed for team review
- **Date:** 2026-11-11
- **Decision scope:** Cargo workspace baseline and the first platform-independent domain crate
- **Decision owners:** Systems Engineering; cross-team review required
- **Implementation status:** Implemented on the feat/rust-core-foundation branch; not yet merged

## Context

ARK needs a stable, platform-independent vocabulary before runtime, storage, identity, and platform integrations can evolve independently. Creating every planned crate immediately would produce empty abstractions with no tested responsibility. Putting all behavior in one crate would make trust boundaries and dependency direction difficult to review.

The team also needs reproducible contributor conventions and a baseline CI pipeline without prematurely selecting cryptographic, storage, policy, or platform dependencies.

## Decision proposed

1. Use a Cargo workspace with resolver version 3, Rust edition 2024, and a declared minimum Rust version of 1.85.
2. Materialize only ark-core initially. Other planned crates are created when they own a concrete, testable responsibility.
3. Keep ark-core free of third-party dependencies and concrete platform, crypto, UI, storage, identity, and authorization implementations.
4. Represent Workspace, Project, File, Session, Device, Package, and Transaction identifiers as distinct opaque 16-byte domain types. The crate does not generate them and does not define their wire or persistence encoding.
5. Represent application, storage, metadata, IPC, policy, recovery, and package versions with distinct types so values from different version domains cannot be substituted accidentally. These types do not define parsers, serialization, compatibility, or migration behavior.
6. Use a fixed error-code taxonomy and a sanitized shared error type. Arbitrary caller data, paths, secrets, and OS error strings do not belong in the shared error value.
7. Define RuntimeMode as workflow vocabulary only. It is not an authorization result, security level, or proof that a process may access a resource.
8. Forbid unsafe Rust in the workspace baseline and require documentation for public APIs.
9. Run formatting, build, Clippy, and tests on Windows, Linux, and macOS in CI. Validate Conventional Commit headers on pull requests.
10. Keep cryptographic library selection, key hierarchy, key derivation, nonce allocation, storage format, identity evidence, policy decisions, and broker request/decision APIs outside this ADR.

## Alternatives considered

### Create every planned crate as an empty placeholder

Rejected. Empty crates create the appearance of architecture without enforcing real responsibilities, tests, or ownership.

### Put every domain type and subsystem in one monolithic crate

Rejected. It would blur dependency direction and make security-sensitive boundaries harder to inspect.

### Add serialization, UUID, async, logging, and crypto dependencies immediately

Rejected. No current ark-core behavior requires them. Adding them before a concrete owner-reviewed need increases dependency surface and prematurely constrains future protocol and format decisions.

### Let ark-core own key, policy, and process identity abstractions

Rejected. Those abstractions belong to subsystem owners and must be reviewed against their threat models. A lowest-level crate must not become an authority or generic service locator.

## Consequences

- The current code is a domain baseline, not a working Secure Workspace runtime.
- Identifier generation must be supplied by an appropriate owner using the approved randomness source; ark-core only wraps a supplied fixed-width value.
- Persistent and IPC representations remain open until their owning formats define canonical encodings and version rules.
- The shared error code is intentionally coarse. Subsystems may keep richer internal diagnostics but must sanitize public messages and logs.
- The crate dependency map remains proposed until reviewed by the owners of crypto, storage, identity, policy, and runtime.
- The minimum Rust version is an explicit compatibility commitment; raising it requires a documented review.
- The CI workflow validates code quality and cross-platform compilation, not runtime compatibility or security assurance.

## Security implications

- Typed IDs reduce accidental domain substitution but do not establish identity, secrecy, or authorization.
- Static error messages reduce accidental leakage through the shared error path but do not prove that subsystem logs are secret-safe.
- RuntimeMode cannot bypass broker, session, identity, or policy checks.
- No cryptographic primitive, key API, storage format, or policy semantics are accepted by this ADR.

## Validation

Required for this foundation:
- formatting check;
- workspace build;
- Clippy with warnings denied;
- unit tests for identifier representation, domain separation, version domains, and sanitized errors;
- Windows, Linux, and macOS CI jobs.

Required before this ADR can be marked Accepted:
- review by the other subsystem owners;
- confirmation that the proposed crate graph has no ownership or dependency conflicts;
- review of the public core API and MSRV commitment;
- successful CI checks on the PR commit set.
