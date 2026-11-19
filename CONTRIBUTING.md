# Contributing to ARK

## Project status

ARK is an early-stage security-sensitive project. Architecture and specifications are being established before production implementation. A green build or passing unit tests do not constitute a security audit.

## Before starting

1. Read AGENTS.md.
2. Read docs/README.md and the relevant architecture/security specifications.
3. Search existing code, issues, branches, and ADRs before proposing a new abstraction.
4. Confirm whether the requested behavior is an accepted decision, a proposal, or an unresolved design question.
5. For security-sensitive changes, agree on the threat-model impact before implementation.

## Branch and pull-request workflow

- Use a focused branch named by area, such as feat/runtime-lifecycle, fix/policy-cache-invalidation, or docs/architecture-map.
- Do not push directly to main.
- Keep each PR focused and explain the user-visible behavior, design rationale, security implications, compatibility impact, and migration needs.
- Link related issues and ADRs.
- Include tests and documentation updates.
- Do not merge a PR with unresolved critical/high security findings, failing required checks, or unreviewed changes to cryptographic, authorization, recovery, or storage-format code.
- At least one independent review is required for ordinary changes. Cryptography, key management, broker, policy, identity, recovery, and storage-format changes require review by at least two qualified team members whenever team availability permits; the author cannot count as their own reviewer.

## Commit history

Follow docs/development/COMMIT_CONVENTIONS.md. Commit headers must use Conventional Commits, and pull-request CI checks the header format.

For core architecture and substantial engineering work, prefer Rebase and Merge only when every commit is a clean, coherent unit. A retained commit must not be a WIP, fixup, known-broken state, or unrelated bundle. Use Squash Merge when the branch history does not meet that standard.

## Rust quality

- Follow rustfmt and Clippy; do not suppress lints without a documented reason.
- Prefer safe Rust. Every unsafe block needs a local safety comment and a documented invariant.
- Avoid panics in externally reachable or security-critical paths. Handle poisoned locks, I/O errors, allocation limits, and cancellation explicitly.
- Use typed identifiers and narrow APIs. Avoid stringly typed security decisions.
- Avoid unnecessary dependencies. Pin and audit dependency changes; review licenses, maintenance, feature flags, and transitive dependencies.
- Keep public APIs small and document invariants, error behavior, ownership, and lifetime semantics.
- Do not expose secret material through Debug, Display, error messages, logs, metrics, or tracing.

## Tests

Add appropriate unit, integration, negative, property, fuzz, compatibility, and crash-recovery tests. Test failure paths, not only successful flows. Security-sensitive tests should include malformed inputs, unauthorized requests, stale evidence, replay, truncation, reordering, concurrency, and interrupted transactions where applicable.

Do not label simulated process termination as a real power-loss test. Clearly distinguish deterministic fault injection, process-crash tests, VM power-cut tests, and hardware testing.

## Documentation and decisions

- Update subsystem documentation when behavior changes.
- Create an ADR for changes to architecture, trust boundaries, cryptographic constructions, key hierarchy, storage/package formats, authorization semantics, or compatibility guarantees.
- Keep the decision status explicit: Accepted, Proposed, Superseded, or Rejected.
- Document assumptions, alternatives, consequences, and migration strategy.
- Update CHANGELOG.md when user-visible behavior or compatibility changes.

## Security reporting

Do not file public issues for exploitable vulnerabilities. Follow SECURITY.md. Never include live credentials, recovery keys, private workspace contents, or sensitive production data in issues, PRs, fixtures, or logs.
