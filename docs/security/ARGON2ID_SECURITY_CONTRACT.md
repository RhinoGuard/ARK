# Proposed Argon2id Security Contract

**Status:** PROPOSED — requires Crypto-owner and independent security review. This document is not an accepted ADR and does not authorize a production behavior change.

**Scope:** S-02 / CCB-03. This contract describes the current KDF API and proposes how secure salt generation should be owned. It does not define password storage, authentication throttling, key wrapping, workspace persistence, or recovery.

## 1. Current implementation

The current `ark-crypto::kdf` API derives a 32-byte key-encryption key (KEK) with Argon2id v1.3. The caller supplies password bytes, salt bytes, and a validated cost configuration. `Argon2Salt::from_slice` validates salt length and rejects an all-zero value; it does not generate randomness or establish provenance.

The crate currently has no operating-system random-number dependency and has no production caller/service layer in this repository. No current code path should be described as generating a secure salt.

## 2. Proposed salt policy

- Generate a fresh 32-byte salt for each independently created password-derived key wrapper / derivation record.
- Treat salt as public, non-secret metadata. Persist the exact salt alongside the KDF parameters and wrapper metadata; do not attempt to regenerate it during unlock.
- Never use a fixed salt, a reused salt across independent records, a timestamp, a process identifier, or caller-provided low-entropy data as a substitute for a CSPRNG.
- Continue accepting caller-supplied salt bytes only for parsing previously persisted records and deterministic test vectors, subject to the existing 16..=64-byte and non-all-zero validation.
- Salt uniqueness is a generation/lifecycle requirement. A 32-byte random salt makes accidental collision negligible; it does not create password entropy and does not compensate for weak passwords.

## 3. Proposed ownership and randomness boundary (CCB-03)

**Recommendation for review:** put the production salt-generation entry point behind `ark-crypto` rather than requiring every future caller to implement its own random-byte handling. The implementation should obtain bytes from the operating system CSPRNG through a small, reviewed Rust abstraction (candidate: `getrandom`). The exact crate version, feature set, target support, and dependency policy must be reviewed before adding the dependency.

The proposed API should return a validated `Argon2Salt` and a sanitized generation error. It must not accept a caller-provided RNG in the public production API. A private test seam may inject a deterministic failing/successful source so error mapping can be tested without exposing test controls to downstream callers.

The lifecycle owner (future workspace/package credential-creation component) remains responsible for:
- generating a new salt whenever it creates a new password-derived wrapper;
- persisting the salt and exact KDF parameters with that wrapper;
- loading and validating the persisted salt during unlock;
- not silently replacing a persisted salt during unlock or password verification;
- rate limiting and bounding concurrent Argon2id work at the service/broker boundary.

This recommendation is intentionally not implemented here. Crypto-owner approval is required because it changes the dependency and API boundary.

## 4. Password and derivation contract

- Password input is opaque bytes, not a Rust `String` contract. The caller/UI owns text encoding (normally UTF-8) and any normalization policy. `ark-crypto` performs no Unicode normalization.
- Empty passwords are rejected.
- Passwords longer than 1,024 bytes are rejected before Argon2 memory allocation.
- Borrowed password slices remain caller-owned; callers should use a zeroizing owner for password buffers and clear them as soon as their lifecycle permits. The KDF cannot zeroize memory it does not own.
- The output is exactly 32 bytes and is a KEK, not a file-encryption key or a general-purpose key. It must not be logged, formatted, or reused for unrelated purposes. Use the separately specified HKDF purpose/context boundary when deriving distinct subkeys.
- The existing production policy bounds remain: memory 16..=256 MiB, iterations 1..=10, lanes 1..=8; default 64 MiB / 3 / 1. These values are not a universal performance guarantee.

## 5. Error behavior

Public errors must remain static and secret-safe. They must not include password bytes, salt bytes, derived output, or OS error details that could disclose sensitive context.

The proposed salt-generation API should map OS entropy-source failures to one dedicated sanitized variant (for example, `RandomGenerationFailed`). It must fail closed: no zero-filled, deterministic, time-based, or fallback salt is returned. The caller must receive an error and abort wrapper creation.

Existing validation and derivation errors remain deterministic: empty/oversized password, invalid salt length, degenerate salt, invalid parameters, and derivation failure.

## 6. Test strategy

Before accepting the proposal, add tests for:
- generated salts always have the configured 32-byte length and pass `Argon2Salt` validation;
- injected entropy-source failure returns only the sanitized random-generation error and produces no salt;
- malformed persisted salts (below/above bounds and all-zero) are rejected;
- the same persisted salt and parameters reproduce the same KDF result (deterministic contract test; not a randomness test);
- error formatting and Debug output never contain input or derived bytes.

Do not test uniqueness by asserting that two random outputs differ; that is a probabilistic test and does not prove CSPRNG quality. Do not claim entropy quality from a smoke test. Validate the selected OS RNG dependency and supported targets separately, and keep the dependency locked.

## 7. Security invariants and limitations

Relevant invariants: ARK-SEC-013 (reviewed cryptography), ARK-SEC-016 (key separation), ARK-SEC-018 (bounded parsing), ARK-SEC-027 (bounded resource use), and ARK-SEC-028 (secret-safe diagnostics).

A salt is not secret and does not increase password entropy. CSPRNG output quality depends on the operating system entropy facility and the reviewed library implementation. This proposal does not implement a password rate limiter, global KDF concurrency admission control, persistent metadata authentication/versioning, key wrapping, or a workspace credential service.

## 8. Review decisions required

- [ ] Crypto owner accepts or amends the proposed ownership boundary.
- [ ] Independent reviewer approves the RNG abstraction, dependency version/features, and supported target behavior.
- [ ] API shape and sanitized error variant are approved.
- [ ] Tests are implemented and run on the supported CI matrix.
- [ ] Persisted salt/parameter metadata contract is coordinated with the future persistence owner.

Until these decisions and implementation evidence exist, CCB-03 remains **proposed / unresolved** and S-02 must remain open.
