# ARK Security Invariants

Status: ACCEPTED baseline. Every invariant must map to implementation evidence and tests before a release claim is made.

An invariant is a property that must remain true across all supported execution paths, including error, race, restart, migration, and recovery paths.

## Authorization and identity

- ARK-SEC-001 — Default Deny: no access is granted unless an explicit valid policy decision allows it.
- ARK-SEC-002 — Fail Closed: unknown, stale, malformed, unavailable, or ambiguous security evidence never becomes Allow.
- ARK-SEC-003 — Identity Is Not Authorization: verified process/device/user identity does not itself grant access.
- ARK-SEC-004 — No Path-as-Authority: caller-controlled paths and filenames are not authorization identities; resolve through trusted workspace/project/file identities and validated filesystem semantics.
- ARK-SEC-005 — Strong Process Evidence: process name, path, or PID alone is insufficient. Use platform evidence with lifetime/freshness checks.
- ARK-SEC-006 — UI Is Not Authority: UI can express intent but cannot issue grants or bypass backend policy.
- ARK-SEC-007 — Least Privilege: grants are bound to subject, workspace, resource scope, operation set, and validity interval.
- ARK-SEC-008 — Explicit Deny Wins: conflicting policy rules must resolve deterministically with deny precedence.

## Sessions and revocation

- ARK-SEC-009 — Session Binding: sessions bind user, device, workspace, application identity, permissions, and expiry as applicable.
- ARK-SEC-010 — Revocation-Aware Caching: cached decisions must have bounded lifetime and reliable invalidation.
- ARK-SEC-011 — Honest Revocation: revocation blocks future mediated operations; already-open handles, mappings, copied plaintext, and OS caches require explicit backend semantics and may not be instantly retractable.
- ARK-SEC-012 — Restart Safety: sessions and process identity are not assumed valid across restart or PID reuse.

## Cryptography and data integrity

- ARK-SEC-013 — No Home-Grown Cryptography: use reviewed constructions and maintained implementations.
- ARK-SEC-014 — AEAD Everywhere Appropriate: encrypted content and security-critical metadata must be authenticated.
- ARK-SEC-015 — Nonce Uniqueness: never reuse a nonce with the same AEAD key; define and test allocation, crash, clone, and rollback behavior.
- ARK-SEC-016 — Key Separation: independent contexts and purposes use separated keys via reviewed derivation/wrapping.
- ARK-SEC-017 — No Raw Master-Key Persistence: raw workspace master keys and passwords are not stored unprotected.
- ARK-SEC-018 — Bounded Parsing: every untrusted format has strict length, count, nesting, allocation, and version bounds.
- ARK-SEC-019 — Anti-Rollback Is Separate: AEAD authenticity alone does not prevent replay of an older valid snapshot; trusted generation/state anchoring is required where rollback resistance is claimed.
- ARK-SEC-020 — No Plaintext Success on Integrity Failure: unauthenticated or incomplete content is never presented as valid data.

## Storage and recovery

- ARK-SEC-021 — Durable Commit Contract: success is reported only after required data and metadata reach the documented durability boundary.
- ARK-SEC-022 — Recoverable Transactions: interrupted writes are detected and resolved through authenticated transaction/journal semantics.
- ARK-SEC-023 — Atomic Publication: imports, migrations, rotations, and snapshots use staged publication and rollback/recovery where supported.
- ARK-SEC-024 — No Shutdown-Only Durability: correctness must not depend on a shutdown callback running.
- ARK-SEC-025 — Recovery Is Verified: restore validates format, authentication, integrity, and compatibility before publishing restored state.

## Platform and operational security

- ARK-SEC-026 — Explicit Trust Boundaries: OS, kernel, administrator, hypervisor, authorized applications, and external storage assumptions are documented.
- ARK-SEC-027 — Bounded Resource Use: requests, queues, caches, parsers, concurrency, retries, and timeouts have limits.
- ARK-SEC-028 — Secret-Safe Diagnostics: logs, errors, telemetry, and crash reports never disclose passwords, keys, recovery secrets, or plaintext.
- ARK-SEC-029 — No False Secure-Erase Claim: deletion is not described as secure erasure unless the platform and storage medium provide a validated guarantee.
- ARK-SEC-030 — Compatibility Is Evidence-Based: support claims require tests against named application/platform/version combinations.
- ARK-SEC-031 — No Silent Downgrade: cryptographic, storage, policy, IPC, and package format versions cannot be silently downgraded.
- ARK-SEC-032 — Security Review Gate: changes to crypto, key management, storage formats, identity, broker, policy, sessions, runtime mediation, and recovery require independent review.

## Required traceability

Every invariant must be mapped to:
- enforcing component(s);
- positive and negative tests;
- failure behavior;
- known limitations;
- release gate, where applicable.

An invariant without enforcement or test evidence is a requirement, not a proven property.
