# Security Assessment and Testing Framework (Core & KDF Working Draft)

**Subsystem:** ARK Security & Verification  
**Author:** Ali (`@alieliteworks-debug`)  
**Target Subsystems:** `ark-core` (PR #2), `ark-crypto` (PR #3)  
**Status:** Proposed Working Draft (Pending Review — Not an Approved ADR Baseline)

> **Evidence & Accuracy Notice:**
> Per `AGENTS.md` and `docs/architecture/SECURITY_INVARIANTS.md`, no security control is considered implemented or verified merely by being documented here. This document separates **currently implemented** primitives from **proposed/deferred** architecture.

---

## 1. Current Scope vs. Deferred Architecture

### 1.1 Currently Implemented (In Scope for Foundation v1)
- **`ark-core` (PR #2):** Typed domain identifiers (`WorkspaceId`, `ProjectId`, `FileId`, `SessionId`, `DeviceId`, `PackageId`, `TransactionId`), coarse sanitized `ErrorCode` / `ArkError`, `RuntimeMode` (`Development`, `Execution`), and domain-separated version types.
- **`ark-crypto` (PR #3):** Password-based KEK derivation (`derive_kek` via Argon2id) and context-separated subkey derivation (`derive_subkey` via HKDF-SHA-256) returning `Zeroizing<[u8; 32]>`.

### 1.2 Proposed / Deferred Architecture (Not Yet Implemented)
The following controls are architectural directions documented in `docs/architecture/ARCHITECTURE.md` and `docs/security/THREAT_MODEL.md`, **not** implemented or verified controls:
- Process identity verification (binary hash, executable path, live PID evidence) and interactive GUI broker consent.
- Authenticated encryption (`XChaCha20-Poly1305` AEAD) and nonce management.
- OS memory pinning (`mlock` / `VirtualLock`) and runtime plaintext working-set isolation.
- Broker IPC rate-limiting and tamper-evident audit logging.

---

## 2. Core Security Invariants Tested (`ark-core`)

1. **Sanitized Error Boundary (`ARK-SEC-028` — Partial):** `ArkError` wraps a coarse `ErrorCode` enum with static display strings and no arbitrary payload fields. *(Note: verifying static error strings does not by itself prove secret-free diagnostics across future subsystems; sentinel injection tests are tracked in `[A-04]`.)*
2. **Domain Type Segregation (`ARK-SEC-003` — Vocabulary Level):** Distinct newtype wrappers prevent accidental mixing of identifiers or version domains at compile time. Typed IDs are domain labels only, not authorization.
3. **Numeric Version Ordering (`ARK-SEC-019` Note):** `ApplicationVersion` and `StorageFormatVersion` implement monotonic `Ord` comparison. **Important:** Version ordering alone is **not** anti-rollback protection; anti-rollback requires trusted monotonic state anchoring (deferred to Storage/Recovery).
4. **Runtime Mode Distinction:** `RuntimeMode::Development` and `RuntimeMode::Execution` are distinct enum variants; `RuntimeMode` is not a security boundary or access grant.

---

## 3. Verification Status & Fuzzing Evidence

- **Compile & Lint Verification (`ark-core`):**
  - `cargo check --tests`: Passed (verifies test compilation against PR #2 public API).
  - `cargo clippy --tests`: Passed under workspace lints (`unsafe_code = "forbid"`, `panic = "deny"`, `unwrap_used = "deny"`).
- **Fuzzing Scaffold (`fuzz/`):**
  - `fuzz/Cargo.toml`, `fuzz_core_ids`, and `fuzz_core_versions` are committed as initial `libfuzzer-sys` targets.
  - **Limitation:** `cargo check` / `cargo clippy` do **not** execute fuzzers. Actual fuzzing duration, seed corpus, coverage, and crash-triage evidence are deferred to `[A-05]`.
