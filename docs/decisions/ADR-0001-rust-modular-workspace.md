# ADR-0001: Rust Modular Workspace and Cross-Platform Strategy

Status: Accepted

## Context

ARK needs a maintainable security-sensitive core across Windows, Linux, and macOS. Platform-specific process identity and filesystem enforcement differ substantially.

## Decision

Use Rust as the primary language and a modular Cargo workspace. Keep domain logic and contracts platform-independent. Place OS-specific implementations behind narrow backend interfaces. Start in-process and split into separate processes only when a concrete security, reliability, or privilege boundary justifies it.

Platform implementation order: Windows, Linux, macOS.

## Alternatives

- Separate services/microservices from the start: rejected as premature complexity for a three-person team.
- One monolithic crate: rejected because it obscures responsibility and makes independent testing/review harder.
- Separate implementation per OS: rejected because it duplicates domain and storage logic.

## Consequences

- Dependency direction and crate ownership must be documented.
- Avoid cyclic dependencies and premature empty crates.
- OS backends require platform-specific tests and compatibility notes.

## Security implications

Core must not trust UI or platform-specific paths. Platform identity evidence must be normalized without losing the evidence required for authorization.

## Validation

Cargo dependency graph checks, cross-target compile checks where available, and platform backend tests.
