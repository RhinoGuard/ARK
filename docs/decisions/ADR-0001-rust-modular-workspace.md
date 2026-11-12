# ADR-0001: Rust Modular Workspace

- **Status:** Accepted (architectural direction)
- **Decision scope:** Primary implementation language and module organization
- **Implementation status:** Not yet implemented

## Context
ARK requires cryptographic storage, platform-specific process/filesystem mechanisms, runtime mediation, and desktop/CLI workflows. These concerns need explicit boundaries while remaining feasible for a small experienced team.

## Decision
Use Rust as the primary implementation language and a modular Cargo workspace with platform-specific backends. Start with a small number of meaningful crates. Introduce separate processes only where they create a measurable security, reliability, or privilege boundary.

## Consequences
- Shared domain contracts and typed errors can be reused.
- Platform-specific mechanisms remain isolated.
- Dependency direction must remain acyclic.
- A modular workspace is not a justification for empty placeholder crates or unnecessary microservices.
- The exact initial crate set and dependency graph are implementation tasks.

## Constraints
Core domain must not depend on UI or concrete OS backends. Policy must not depend on UI. Runtime must not bypass policy/broker. Platform adapters do not own product authorization policy.

## Verification
Review the Cargo dependency graph, enforce formatting/lints/tests in CI, and document every exception to dependency boundaries.
