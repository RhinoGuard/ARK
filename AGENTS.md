# ARK — Instructions for AI Coding Agents

This file is mandatory context for every AI coding agent, assistant, or automation working in this repository. Read it before inspecting, planning, editing, testing, or committing code.

## 1. Product definition

ARK is a cross-platform Secure Workspace for developers. Its intended capabilities include encrypted project/workspace storage, controlled application access, development and execution runtimes, recovery, and portable encrypted packages.

ARK is not a promise of protection against a fully compromised operating system, kernel, hypervisor, administrator account, or a process that is legitimately authorized to read plaintext and can exfiltrate it. Never make claims stronger than the implemented and tested security boundary.

## 2. Source-of-truth order

When information conflicts, use this order:
1. Explicitly accepted decisions recorded in an Architecture Decision Record (ADR).
2. Security invariants and approved threat model.
3. Architecture and subsystem specifications.
4. Current implementation and tests (which may reveal undocumented behavior or drift).
5. Roadmap and proposals.
6. Chat history, issue discussions, comments, and AI-generated suggestions.

A proposal is not an accepted decision. A planned feature is not implemented. A test stub is not a security guarantee. If documents conflict, stop and report the conflict; do not silently choose or rewrite the security model.

## 3. Mandatory reading

Before changing a subsystem, read:
- docs/README.md
- docs/architecture/ARCHITECTURE.md
- docs/architecture/SECURITY_INVARIANTS.md
- docs/security/THREAT_MODEL.md
- the relevant subsystem specification
- relevant ADRs under docs/decisions/

Also inspect the current code, tests, dependency graph, and repository state. Do not rely on an earlier AI summary as a substitute.

## 4. Non-negotiable engineering rules

- Default deny. Unknown, stale, malformed, ambiguous, or unverifiable authorization evidence must not grant access.
- Separate identity verification from authorization decisions.
- The UI is never a security authority. All security decisions are enforced in trusted Rust components.
- Never authorize by path, filename, process name, PID, or executable path alone.
- Do not expose storage keys or bypass the broker/policy boundary to simplify an implementation.
- Do not invent cryptographic algorithms, protocols, nonce schemes, or key derivation.
- Use reviewed, maintained cryptographic libraries and document exact versions/features and rationale.
- Authenticate metadata and ciphertext. Reject tampering, truncation, reordering, invalid versions, and malformed lengths.
- Never reuse a nonce with the same AEAD key. The nonce allocation design must be reviewed before implementation.
- Do not log passwords, keys, plaintext, recovery secrets, or sensitive paths/content.
- Bound all untrusted input, allocations, recursion, concurrency, timeouts, and parser work.
- Treat errors as security-relevant. Fail closed without leaking secrets in diagnostics.
- No unsafe Rust without a documented safety argument, narrow scope, and review.
- No silent format, policy, or key-hierarchy changes. Use an ADR and version/migration plan.
- Do not claim universal application compatibility or complete mediation unless the platform mechanism and tests demonstrate it.
- Never claim secure deletion of plaintext from SSDs, journals, snapshots, swap, crash dumps, or backups without a defensible platform-specific guarantee.
- Do not weaken security to make a test pass or improve a benchmark.

## 5. Architecture boundaries

The intended logical boundaries are:
- ark-core: platform-independent domain types and contracts.
- ark-crypto: cryptographic primitives, key derivation/wrapping, authenticated chunk operations.
- ark-storage: encrypted storage format, chunk index, transactional writes, journal, snapshots, recovery.
- ark-identity: OS-backed user, device, process, and executable identity evidence.
- ark-policy: default-deny policy evaluation and security-level requirements.
- ark-session: bounded, scoped, expiring, revocable sessions.
- ark-broker: trusted authorization orchestration; the controlled route to protected resources.
- ark-runtime: Development Mode and Execution Mode orchestration.
- ark-filesystem: common filesystem semantics and mediation contracts.
- platform backends: OS-specific enforcement and identity evidence.
- ark-package: portable encrypted package creation, verification, and import.
- ark-recovery: recovery methods and key-wrapper lifecycle.
- ark-audit: privacy-conscious, tamper-evident event records.

These are intended boundaries, not permission to create empty crates or speculative abstractions. Implement only when a real dependency and tested responsibility exist. Avoid cyclic dependencies.

## 6. Development and execution behavior

Development Mode uses a limited active working set. A file is made available to an authorized editor only when needed. The durable source of truth remains encrypted. Changed data is persisted through an authenticated, crash-recoverable journal/transaction design. Closing a tab is not proof that every process handle or mapping is gone.

Execution Mode uses user-defined directory/resource scopes and application permissions so dynamically resolved dependencies can be served without guessing every source-level include/import. Scopes must be explicit, bounded, time-limited where appropriate, and tied to verified process identity and session state.

The filesystem layer must account for open handles, rename/unlink semantics, mmap, locks, directory enumeration, notifications, flush, truncation, symlinks, hard links, alternate paths, child processes, and process restarts. A passive file watcher is not complete mediation.

## 7. Portable package rules

- Exporting an already encrypted package or copying encrypted vault bytes must not require decrypting the workspace.
- Each independently created portable package has an independent package key and independent package credential/wrapper.
- The package password is used through a memory-hard KDF to wrap a random package key; it is not used directly as the file-encryption key.
- A package credential must be established when creating/configuring a new package. Copying an existing encrypted package does not require the credential.
- Do not store the password or raw package key in the package.
- Package format must be versioned, authenticated, bounded, portable across supported operating systems, and safely importable.
- Import must verify structure and integrity before publishing restored data; restore must be staged and atomic where possible.
- An encrypted export cannot include uncommitted data that never reached durable storage. Document the durability boundary.

## 8. Security levels

Standard, Elevated, and Critical classify operation sensitivity and required step-up authentication. They are not three encryption strengths, not three workspace master passwords, and not a substitute for authorization policy. Every sensitive operation must map to an explicit policy and authentication requirement.

## 9. Collaboration and repository hygiene

- Work on a focused feature branch; do not push directly to main.
- Read current branch state before writing. Do not overwrite concurrent work.
- Keep pull requests small and reviewable.
- Add or update tests with every behavior change.
- Update docs and ADRs when behavior or security assumptions change.
- Record security-sensitive changes and unresolved risks in the PR.
- Do not add generated binaries, secrets, private test data, real recovery material, or user workspaces.
- Do not run destructive commands or change repository visibility/settings without explicit authorization.
- Never claim tests, audits, fuzzing, or builds ran unless they actually ran and their results are available.

## 10. Required response format for AI agents

For each non-trivial task, report:
1. What was inspected.
2. What changed and why.
3. Security invariants affected.
4. Tests actually run and exact results.
5. Known limitations and unresolved questions.
6. Files/ADRs updated.
7. Whether the work is ready for human review (not whether it is “secure” in the abstract).

If a requested change conflicts with this document, stop and ask for a decision rather than silently weakening the architecture.
