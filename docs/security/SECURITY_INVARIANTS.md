# Security Invariants

These invariants are design constraints. Each implementation change must identify which invariants it affects and add tests/evidence. An invariant is not considered implemented merely because it is documented.

| ID | Invariant |
|---|---|
| ARK-SEC-001 | Security by default: no implicit grant of access. |
| ARK-SEC-002 | Default deny when identity, policy, session, or resource evidence is absent, invalid, stale, or ambiguous. |
| ARK-SEC-003 | Convenience features must not create a path that bypasses security enforcement. |
| ARK-SEC-004 | Identity, authentication, session validity, and authorization are distinct decisions. |
| ARK-SEC-005 | Grants follow least privilege across process identity, resource scope, operation, and duration. |
| ARK-SEC-006 | UI is presentation and intent capture, never the security authority. |
| ARK-SEC-007 | Cached identity/policy decisions have bounded lifetime and revocation-aware invalidation. |
| ARK-SEC-008 | Security failures do not silently fall back to weaker behavior. |
| ARK-SEC-009 | Performance optimizations preserve the security property and durability contract. |
| ARK-SEC-010 | Trust boundaries and out-of-scope threats are explicit. |
| ARK-SEC-011 | No nonce is reused with the same AEAD key. |
| ARK-SEC-012 | Every ciphertext and security-relevant metadata field is authenticated in a context-bound design. |
| ARK-SEC-013 | Durable commit is acknowledged only after the defined data and metadata ordering is durably satisfied. |
| ARK-SEC-014 | Recovery artifacts and portable packages never contain raw workspace master keys. |
| ARK-SEC-015 | Import does not publish partially verified content as a successful restore. |
| ARK-SEC-016 | Revocation claims never exceed the selected backend's proven handle/mapping semantics. |
| ARK-SEC-017 | Audit events never contain passwords, key material, recovery secrets, tokens, or file plaintext. |
| ARK-SEC-018 | Version and migration handling must not silently reinterpret or downgrade protected data. |

## Traceability requirement

For each invariant, maintain a trace from:
**Invariant → owning module/API → enforcement point → positive/negative tests → review evidence.**

Tests should include adversarial cases, not only happy paths. Security-sensitive changes require independent review. If an invariant cannot be enforced on a platform, document the limitation and prevent product copy from implying otherwise.

## Failure policy

When a security-critical dependency or state cannot be verified, fail closed where safe and preserve recoverable data. Do not destroy the only copy of encrypted data in an attempt to recover. Error handling must not leak secrets.
