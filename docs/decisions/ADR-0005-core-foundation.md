# ADR-0005: Initial Rust Workspace and Core Domain Foundation

- **Status:** Accepted
- **Date:** 2026-11-11
- **Decision date:** 2026-11-19
- **Decision scope:** Cargo workspace baseline and the first platform-independent domain crate
- **Decision owners:** Systems Engineering; cross-team review completed
- **Implementation status:** Implemented and merged to `main` via PR #2 (merge commit `bafa677d6b42e7b04b736fcc386cd8e4ea4d9943`)

## Context

ARK needs a stable, platform-independent vocabulary before runtime, storage, identity, and platform integrations can evolve independently. Creating every planned crate immediately would produce empty abstractions with no tested responsibility. Putting all behavior in one crate would make trust boundaries and dependency direction difficult to review.

The team also needs reproducible contributor conventions and a baseline CI pipeline without prematurely selecting cryptographic, storage, policy, or platform dependencies.

## Decision

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
- The broader crate dependency map remains a planning artifact; this ADR accepts only the workspace/Core foundation and does not approve future subsystem implementations.
- The minimum Rust version is an explicit compatibility commitment; raising it requires a documented review.
- The CI workflow validates code quality and cross-platform compilation, not runtime compatibility or security assurance.

## Security implications

- Typed IDs reduce accidental domain substitution but do not establish identity, secrecy, or authorization.
- Static error messages reduce accidental leakage through the shared error path but do not prove that subsystem logs are secret-safe.
- RuntimeMode cannot bypass broker, session, identity, or policy checks.
- No cryptographic primitive, key API, storage format, or policy semantics are accepted by this ADR.

## Validation

Completed for this foundation:
- formatting check;
- workspace build;
- Clippy with warnings denied;
- unit tests for identifier representation, domain separation, version domains, and sanitized errors;
- Windows, Linux, and macOS CI jobs, plus the Rust 1.85 MSRV job and Conventional Commit validation.

The final PR #2 head was `03336335ae5f87dfc93f70ea56f5448265cb1749`; all recorded GitHub Actions checks passed. Validation was performed by GitHub Actions, not claimed as a local run.

## Decision record — 2026-11-19

ADR-0005 is **Accepted** for the workspace and `ark-core` foundation scope after review of the merged PR #2 implementation and explicit subsystem-owner confirmations.

- **Systems Engineering:** ElmanCod110 — completed the implementation/compatibility cross-check and records the final decision.
- **Security Validation:** alieliteworks-debug — explicitly approved the technical content, confirmed current Core API sufficiency and the non-overclaiming test boundaries, and requested no speculative Core expansion. Evidence: [PR #2 comment 7012352146](https://github.com/RhinoGuard/ARK/pull/2#issuecomment-7012352146).
- **Crypto:** ZeroTrace79 — explicitly approved with no requested ADR amendments; confirmed dependency direction, IKM/context and error boundaries, and workspace/MSRV/lint compatibility. Evidence: [Issue #15 comment 7017176050](https://github.com/RhinoGuard/ARK/issues/15#issuecomment-7017176050) and [PR #2 comment 7017190351](https://github.com/RhinoGuard/ARK/pull/2#issuecomment-7017190351).

No owner objection or requested amendment remains for the ADR-0005 Core foundation decision. The confirmations do not constitute approval of PR #3 as a whole, do not close the separate Core-to-Crypto integration contract (E-03 / Issue #17), and do not claim that deferred runtime, storage, identity, authorization, recovery, or platform controls are implemented. The broader crate map and future subsystem contracts remain subject to their own owner reviews and ADRs.
