# ARK Security Invariant Traceability Matrix (Foundation v1)

**Status:** Baseline traceability for `[A-03]` (Issue #11).  
**Canonical Source:** `docs/architecture/SECURITY_INVARIANTS.md` (`ARK-SEC-001` through `ARK-SEC-032`).  
**Evaluated Subsystems:**
- `ark-core` (PR #2: `feat/rust-core-foundation`)
- `ark-crypto` (PR #3: `feature/crypto-kdf-implementation`)
- Core Security & Fuzz Tests (PR #4 / `feat/security-testing-framework`)

> **Honesty & Quality Constraints:**
> - Typed identifiers (`WorkspaceId`, `SessionId`, etc.) are domain labels, **not** authentication or authorization.
> - `RuntimeMode` (`Development`, `Execution`) is a workflow selector, **not** a security boundary or access grant.
> - Numeric version ordering (`StorageFormatVersion`, `ApplicationVersion`) is **not** anti-rollback without trusted monotonic state.
> - Unimplemented subsystems (storage, policy, broker, identity, recovery, filesystem) are explicitly marked `Not Implemented` / `Deferred`.

---

## 1. Traceability Matrix: Core & KDF Foundation

| ID | Invariant Statement | Component / API | Enforcement Point | Test Path & Assertion | Status | Owner & Next Step |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| **ARK-SEC-001** | Default Deny | `ark-policy`, `ark-broker` | Not yet in codebase | None | `Not Implemented` (Deferred to Phase 4) | Policy/Broker Owner |
| **ARK-SEC-002** | Fail Closed | `ark-core::errors::ArkError` | Coarse `ErrorCode` taxonomy; no default `Allow` | `crates/ark-core/tests/negative_core_tests.rs` (`test_negative_error_code_exhaustiveness`) | `Partially Implemented` (Core error vocabulary exists; policy/state fail-closed deferred) | Core / Policy |
| **ARK-SEC-003** | Identity Is Not Authorization | `ark-core::ids::*` | Type system & doc contracts explicitly state IDs are labels only | `crates/ark-core/tests/security_invariants_tests.rs` (`test_invariant_identifier_type_safety`) | `Enforced and Tested` (At Core vocabulary level; runtime enforcement deferred) | Core / Identity |
| **ARK-SEC-004** | No Path-as-Authority | `ark-core::ids::FileId` | `FileId` provides path-independent 16-byte identity | `crates/ark-core/tests/negative_core_tests.rs` (`test_negative_domain_identifier_cross_construction`) | `Partially Implemented` (Type exists; filesystem resolution deferred to Phase 6) | Core / Filesystem |
| **ARK-SEC-005** | Strong Process Evidence | `ark-identity` | Not yet in codebase | None | `Not Implemented` (Deferred to Phase 4) | Identity Owner |
| **ARK-SEC-006** | UI Is Not Authority | Desktop UI / Broker | Not yet in codebase | None | `Not Implemented` (Deferred to Phase 8) | UI / Broker |
| **ARK-SEC-007** | Least Privilege | `ark-policy`, `ark-session` | Not yet in codebase | None | `Not Implemented` (Deferred to Phase 4) | Policy / Session |
| **ARK-SEC-008** | Explicit Deny Wins | `ark-policy` | Not yet in codebase | None | `Not Implemented` (Deferred to Phase 4) | Policy |
| **ARK-SEC-009** | Session Binding | `ark-session` | Not yet in codebase | None | `Not Implemented` (Deferred to Phase 4) | Session |
| **ARK-SEC-010** | Revocation-Aware Caching | `ark-session`, `ark-policy` | Not yet in codebase | None | `Not Implemented` (Deferred to Phase 4) | Session / Policy |
| **ARK-SEC-011** | Honest Revocation | `ark-filesystem`, `ark-runtime` | Not yet in codebase | None | `Not Implemented` (Deferred to Phase 5/6) | Runtime / Filesystem |
| **ARK-SEC-012** | Restart Safety | `ark-session`, `ark-identity` | Not yet in codebase | None | `Not Implemented` (Deferred to Phase 4) | Session / Identity |
| **ARK-SEC-013** | No Home-Grown Cryptography | `ark-crypto` (`src/kdf/*.rs`) | Uses reviewed `argon2`, `hkdf`, `sha2` crates | `Cargo.toml` dependencies + `tests/kdf_tests.rs`; independent audit in `[A-06]` | `Enforced and Tested` | Crypto (`[S-01]`) & Security (`[A-06]`) |
| **ARK-SEC-014** | AEAD Everywhere Appropriate | `ark-crypto` (AEAD) | Not yet implemented (only KDF in PR #3) | None | `Not Implemented` (Deferred to Phase 2b/3) | Crypto |
| **ARK-SEC-015** | Nonce Uniqueness | `ark-crypto` (AEAD) | Not yet implemented | None | `Not Implemented` (Deferred to Phase 2b/3) | Crypto |
| **ARK-SEC-016** | Key Separation | `ark-crypto::kdf::derive_subkey` | HKDF-SHA-256 `context` parameter (`MAX_CONTEXT_LEN = 256`, non-empty check) | `tests/kdf_tests.rs` (`hkdf_separates_distinct_contexts`, `hkdf_rejects_empty_context`) | `Partially Implemented` (Structured context registry pending `[S-04]`) | Crypto (`[S-04]`) |
| **ARK-SEC-017** | No Raw Master-Key Persistence | `ark-crypto::kdf::*` | Outputs wrapped in `zeroize::Zeroizing<[u8; 32]>` | Code inspection (`src/kdf/argon2id.rs`, `hkdf_sha256.rs`); storage persistence deferred | `Partially Implemented` (In-memory scrubbing of output; storage deferred) | Crypto / Storage |
| **ARK-SEC-018** | Bounded Parsing | `ark-crypto::kdf::*` | `password.len() <= 1024`, `salt.len() in 16..=64`, `context.len() <= 256` | `tests/kdf_tests.rs` (`argon2id_rejects_invalid_salt`); adversarial bounds in `[A-04]` | `Partially Implemented` (`Argon2idConfig` bounds pending `[S-02]`/`[S-03]`) | Crypto (`[S-02]`, `[S-03]`) |
| **ARK-SEC-019** | Anti-Rollback Is Separate | `ark-core::versions::*` | Distinct version structs with `Ord` | `crates/ark-core/tests/security_invariants_tests.rs` (ordering only; trusted state deferred) | `Documented Only` (Version ordering alone is not anti-rollback) | Storage / Recovery |
| **ARK-SEC-020** | No Plaintext Success on Integrity Failure | `ark-crypto`, `ark-storage` | Not yet implemented | None | `Not Implemented` (Deferred to AEAD/Storage) | Crypto / Storage |
| **ARK-SEC-021** | Durable Commit Contract | `ark-storage` | Not yet in codebase | None | `Not Implemented` (Deferred to Phase 3) | Storage |
| **ARK-SEC-022** | Recoverable Transactions | `ark-storage` | Not yet in codebase | None | `Not Implemented` (Deferred to Phase 3) | Storage |
| **ARK-SEC-023** | Atomic Publication | `ark-storage`, `ark-package` | Not yet in codebase | None | `Not Implemented` (Deferred to Phase 3/7) | Storage / Package |
| **ARK-SEC-024** | No Shutdown-Only Durability | `ark-storage`, `ark-runtime` | Not yet in codebase | None | `Not Implemented` (Deferred to Phase 3/5) | Storage / Runtime |
| **ARK-SEC-025** | Recovery Is Verified | `ark-recovery` | Not yet in codebase | None | `Not Implemented` (Deferred to Phase 7) | Recovery |
| **ARK-SEC-026** | Explicit Trust Boundaries | `docs/security/THREAT_MODEL.md`, `KDF_THREAT_MODEL.md` | Documented boundaries and non-guarantees | Documentation review (`[A-02]`) | `Documented Only` | Security (`[A-02]`) & Systems (`[E-04]`) |
| **ARK-SEC-027** | Bounded Resource Use | `ark-crypto::kdf` | Password/salt/context length limits; `Argon2idConfig` unbounded | Adversarial bounded tests planned in `[A-04]` / `[A-05]` | `Partially Implemented` (`Argon2idConfig` upper bounds missing) | Crypto (`[S-03]`) |
| **ARK-SEC-028** | Secret-Safe Diagnostics | `ark-core::errors::ArkError`, `Argon2Error`, `HkdfError` | Unit-variant error enums with static messages; no secret fields | `security_invariants_tests.rs` (`test_invariant_sanitized_error_no_leakage`); sentinel tests in `[A-04]` | `Enforced and Tested` (For current error enums; sentinel tests pending `[A-04]`) | Core / Crypto / Security |
| **ARK-SEC-029** | No False Secure-Erase Claim | `docs/security/SECURITY_INVARIANTS.md`, `KDF_THREAT_MODEL.md` | Explicit non-guarantee documented | Documentation review | `Documented Only` | Security / Storage |
| **ARK-SEC-030** | Compatibility Is Evidence-Based | `ark-core::versions::*` | Independent version types prevent cross-domain substitution | `security_invariants_tests.rs` (`test_invariant_version_domain_separation`) | `Partially Implemented` (Domain separation enforced; runtime matrix deferred) | Core / Runtime |
| **ARK-SEC-031** | No Silent Downgrade | `ark-core::versions`, `ark-storage` | Distinct version types; parser enforcement deferred | `security_invariants_tests.rs` | `Partially Implemented` (Types exist; format parser rejection deferred) | Core / Storage |
| **ARK-SEC-032** | Security Review Gate | `.github/workflows/ci.yml`, `CODEOWNERS` | `#![forbid(unsafe_code)]`, strict Clippy lints, PR reviews | `cargo clippy --tests` & independent review in `[A-06]` | `Enforced and Tested` (CI lints active; `[A-06]` pending) | Security & Systems |

---

## 2. Material Gaps & Follow-Up Actions

1. **`Argon2idConfig` Resource & Security Bounds (`ARK-SEC-018`, `ARK-SEC-027`):** `memory_kib`, `iterations`, and `lanes` are public fields without min/max enforcement. Tracked in `[S-02]` / `[S-03]`.
2. **HKDF Context Registry (`ARK-SEC-016`):** `derive_subkey` accepts arbitrary `&[u8]` contexts. Structured domain-separation labels tracked in `[S-04]`.
3. **Sentinel Secret-Leakage Tests (`ARK-SEC-028`):** Current Core tests verify static error messages; explicit sentinel secret injection tests across KDF/Core boundaries will be added in `[A-04]`.
4. **Workspace Lint Integration for `ark-crypto` (`ARK-SEC-032`):** `ark-crypto` (PR #3) is not yet integrated into the root Cargo workspace (`[S-01]`) where `#![forbid(unsafe_code)]` and `unwrap_used = "deny"` are enforced.
