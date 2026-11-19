# ARK KDF Foundation Threat Model (Argon2id & HKDF-SHA-256)

**Status:** Proposed Working Draft for Foundation Stabilization v1 (`[A-02]` / Issue #10) — Synchronized with Activity Deliverables.  
**Scope:** Strictly scoped to the current key-derivation foundation (`ark-crypto` PR #3: `src/kdf/argon2id.rs`, `src/kdf/hkdf_sha256.rs`) and its integration boundary with `ark-core` (PR #2).  
**Out of Scope:** Unimplemented AEAD chunk encryption, storage formats, WAL/journaling, recovery envelopes, and full-product security claims.

> **Evidence & Honesty Rule:**
> In accordance with `AGENTS.md` and `docs/architecture/SECURITY_INVARIANTS.md`, no threat mitigation is considered established or verified solely by virtue of being documented. Implemented mitigations must link to concrete code or executable tests. The existence of test code or audit deliverables is distinguished from integrated end-to-end verification.

---

## 1. Scope, Implemented Code, & Explicit Non-Guarantees

### 1.1 Currently Implemented Primitives (PR #3)
1. **`derive_kek(password, salt, config)`** (`src/kdf/argon2id.rs`): Derives a 256-bit Key Encryption Key (KEK) using Argon2id (`Version::V0x13`).
   - Checks: `password` non-empty and `<= 1024` bytes; `salt` length in `16..=64` bytes.
   - Output: `Zeroizing<[u8; 32]>`.
2. **`derive_subkey(input_key_material, salt, context)`** (`src/kdf/hkdf_sha256.rs`): Derives a 256-bit subkey using HKDF-SHA-256.
   - Checks: `input_key_material` non-empty; `context` non-empty and `<= 256` bytes (`MAX_CONTEXT_LEN`).
   - Output: `Zeroizing<[u8; 32]>`.

### 1.2 Explicit Non-Guarantees & Environmental Limits
- **Salt Generation:** `derive_kek` does not generate salts; salt entropy and uniqueness are entirely caller/CSPRNG responsibilities.
- **Memory Scrubbing Limits:** `Zeroizing` zeros its owned memory upon `Drop`. It cannot prevent or scrub copies in OS swap/pagefile, crash dumps, CPU registers, hibernation files, or borrowed caller slices (`password: &[u8]`, `input_key_material: &[u8]`). OS memory locking (`mlock` / `VirtualLock`) is not implemented.
- **Password Strength:** No password entropy enforcement, dictionary filtering, or rate-limiting exists in this primitive layer.
- **Anti-Rollback & State:** KDFs are stateless pure functions. Protection against replay or rollback requires state anchoring outside this crate.

---

## 2. Threat Analysis Matrix

### T-01: Offline Password Guessing & Low-Entropy Passwords
- **Asset:** User password and derived 256-bit KEK.
- **Attacker Capability:** Offline adversary with access to persisted ciphertext headers, salt, and KDF parameters.
- **Preconditions:** Weak user password or low computational cost parameters.
- **Attack Path:** Attacker executes optimized GPU/ASIC dictionary attacks against Argon2id using known salt and parameters.
- **Impact:** Total compromise of KEK and unwrapping of workspace keys.
- **Likelihood / Risk Rationale:** High if low-entropy passwords are permitted or if default parameters are reduced.
- **Existing Mitigation:** Uses `argon2` (`Algorithm::Argon2id`, `Version::V0x13`). Default configuration provides `memory_kib: 65536` (64 MiB), `iterations: 3`, `lanes: 1`. Empty passwords rejected.
- **Mitigation Status:** `Partially Implemented` (algorithm choice implemented; password entropy checks and benchmark-backed production parameter policy deferred to `[S-03]` and UI/Policy).
- **Residual Risk & Owner:** Brute-force feasibility on weak passwords. **Owner:** Crypto (`[S-03]`) & Policy/UI.
- **Verification Evidence:** `src/kdf/argon2id.rs` unit tests; benchmark evidence pending `[S-03]`.

---

### T-02: Predictable Salt, Salt Reuse, & Missing Entropy
- **Asset:** KEK uniqueness across vaults, packages, and password rotations.
- **Attacker Capability:** Passive/offline adversary observing multiple encrypted vaults or password rotations.
- **Preconditions:** Caller provides identical, static, or low-entropy salt bytes (e.g., all-zeroes or counter).
- **Attack Path:** Identical `(password, salt)` pairs yield identical KEKs, allowing multi-target attacks and cross-vault equality detection.
- **Impact:** Loss of cryptographic isolation between vaults or password generations.
- **Likelihood / Risk Rationale:** Medium; `derive_kek` accepts raw `&[u8]` from external callers.
- **Existing Mitigation:** Salt length enforced to `16..=64` bytes (`Argon2Error::InvalidSaltLength`).
- **Mitigation Status:** `Caller Responsibility` (length is enforced; entropy quality and uniqueness are external caller duties).
- **Residual Risk & Owner:** Caller supplying low-entropy salt. **Owner:** Crypto (`[S-02]` salt policy) & Storage/Package callers.
- **Verification Evidence:** `tests/kdf_tests.rs` (`argon2id_rejects_invalid_salt`, `argon2id_changes_when_salt_changes`).

---

### T-03: KDF Resource Exhaustion & Parameter Abuse (DoS)
- **Asset:** Host memory, CPU availability, and service responsiveness.
- **Attacker Capability:** Adversary supplies crafted KDF configuration (e.g., via hostile `.arkvault` package header) or oversized password.
- **Preconditions:** Caller deserializes untrusted configuration directly into `Argon2idConfig` without ARK-enforced bounds.
- **Attack Path:** Attacker sets `memory_kib` to extreme gigabyte values or `iterations` to millions, triggering OOM crash or thread denial of service.
- **Impact:** Process crash, thread starvation, or resource exhaustion DoS.
- **Likelihood / Risk Rationale:** High upon importing untrusted packages if bounds are not strictly enforced by ARK.
- **Existing Mitigation:** Password length bounded to `<= 1024` bytes (`Argon2Error::PasswordTooLong`).
- **Mitigation Status:** `Partially Implemented` (password length bounded; `Argon2idConfig` fields are `pub` without ARK min/max clamp).
- **Residual Risk & Owner:** Unbounded parameter execution. Internal `argon2::Params` limits do not replace ARK policy. **Owner:** Crypto (`[S-02]`, `[S-03]`) & Package Importer.
- **Verification Evidence:** Bounded negative tests drafted on branch `test/adversarial-negative-tests` (`[A-04]`); parameter policy implementation in `[S-03]`.

---

### T-04: HKDF Context Confusion & Cross-Purpose Subkey Reuse
- **Asset:** Domain-separated subkeys (Project Keys, File Keys, Metadata Keys).
- **Attacker Capability:** Adversary induces two callers or subsystems to pass identical `context` strings.
- **Preconditions:** Callers use arbitrary, unregistered `&[u8]` context labels.
- **Attack Path:** Two distinct operational purposes (e.g., chunk encryption vs. header MAC) derive the same subkey from a shared master key.
- **Impact:** Catastrophic key reuse across distinct algorithms or security boundaries.
- **Likelihood / Risk Rationale:** Medium; string length is capped at 256 bytes, but no domain separation taxonomy is enforced in code.
- **Existing Mitigation:** Context must be non-empty and `<= 256` bytes (`HkdfError::EmptyContext`, `HkdfError::ContextTooLong`).
- **Mitigation Status:** `Partially Implemented` (length/non-empty validated; structured context registry and domain prefix contract deferred to `[S-04]`).
- **Residual Risk & Owner:** Ad-hoc context strings. **Owner:** Crypto (`[S-04]`).
- **Verification Evidence:** `tests/kdf_tests.rs` (`hkdf_separates_distinct_contexts`); context contract tests in `[S-04]`.

---

### T-05: Secret Disclosure via Errors, Logs, Debug, & Panics
- **Asset:** Passwords, KEKs, IKMs, and derived subkeys.
- **Attacker Capability:** Local process, log consumer, or crash dump analyzer reading stdout/stderr, logs, or error structures.
- **Preconditions:** Secrets formatted into error representations, panic messages, or debug strings.
- **Attack Path:** Malformed invocation triggers an error or panic that includes input slices in diagnostic text.
- **Impact:** Exposure of plaintext credentials or key material in logs/dumps.
- **Likelihood / Risk Rationale:** Low for current error enums (`Argon2Error`, `HkdfError` have static strings), but risk exists in caller error chains and debug formatters.
- **Existing Mitigation:** Error types are unit enums with static text; KDF functions do not panic on invalid inputs.
- **Mitigation Status:** `Implemented` for KDF primitives; `Caller Responsibility` for calling subsystems.
- **Residual Risk & Owner:** Callers logging raw inputs. **Owner:** Core / Crypto / Security.
- **Verification Evidence:** `src/kdf/argon2id.rs`, `src/kdf/hkdf_sha256.rs`; explicit sentinel leakage test suite drafted on branch `test/adversarial-negative-tests` (`[A-04]`).

---

### T-06: Cryptographic Dependency & Supply-Chain Compromise
- **Asset:** Primitives integrity (`argon2`, `hkdf`, `sha2`, `zeroize`).
- **Attacker Capability:** Upstream vulnerability, supply-chain injection, or unmaintained transitive dependencies.
- **Preconditions:** Inclusion of unpinned or vulnerable crate versions in `Cargo.lock`.
- **Attack Path:** Known vulnerability in upstream hashing or zeroization crate exploited to bypass protections.
- **Impact:** Weakened encryption, side-channel leakage, or memory corruption.
- **Likelihood / Risk Rationale:** Low-Medium; dependencies are standard RustCrypto crates.
- **Existing Mitigation:** Dependency versions declared in `Cargo.toml`.
- **Mitigation Status:** `Initial Review Drafted` (initial independent advisory and implementation review delivered in `docs/security/CRYPTO_DEPENDENCY_AND_IMPLEMENTATION_REVIEW.md` on branch `docs/crypto-security-review` (`[A-06]`); formal automated CI audit gate deferred).
- **Residual Risk & Owner:** Transitive security advisories across release cycles. **Owner:** Security (`[A-06]`).
- **Verification Evidence:** Initial review report delivered in `[A-06]`.

---

### T-07: Incomplete Memory Scrubbing & OS Paging Remnants
- **Asset:** Plaintext passwords, intermediate state, and derived keys in RAM.
- **Attacker Capability:** Local attacker with memory dump capability, pagefile inspection, or post-termination RAM read.
- **Preconditions:** Secrets held in unpinned RAM during execution.
- **Attack Path:** OS swaps process memory to unencrypted disk, or borrowed caller slices remain uncleared in memory after KDF return.
- **Impact:** Post-session extraction of passwords or key material from storage media or memory dumps.
- **Likelihood / Risk Rationale:** Medium-High across consumer desktop OS environments.
- **Existing Mitigation:** Derived output buffers wrap arrays in `zeroize::Zeroizing<[u8; 32]>`.
- **Mitigation Status:** `Partially Implemented` (output buffers zeroized on drop; input slices, compiler register artifacts, and OS paging are caller/OS boundaries and documented Non-Guarantees).
- **Residual Risk & Owner:** Residual plaintext in RAM/swap. **Owner:** Core / Runtime / Storage (`ARK-SEC-029`).
- **Verification Evidence:** Code inspection of `Zeroizing` in `src/kdf/*.rs`.

---

### T-08: Weak or Short IKM Misuse at HKDF Boundary
- **Asset:** Subkey entropy and cryptographic strength.
- **Attacker Capability:** Call-site misuse (caller passing low-entropy password or < 16-byte IKM directly to `derive_subkey`).
- **Preconditions:** `derive_subkey` accepts any non-empty byte slice (`&[u8]`).
- **Attack Path:** A developer mistakenly passes a raw user password into `derive_subkey` instead of `derive_kek`, bypassing Argon2id memory hardness.
- **Impact:** Subkeys derived from weak entropy, susceptible to rapid offline dictionary attack.
- **Likelihood / Risk Rationale:** Medium; raw slice parameters allow cross-domain confusion without type-system enforcement.
- **Existing Mitigation:** Rejects empty slice (`HkdfError::EmptyInputKeyMaterial`).
- **Mitigation Status:** `Caller Responsibility` / `Deferred` (type-level IKM validation and Core-to-Crypto typed contracts deferred to `[E-03]` and `[S-04]`).
- **Residual Risk & Owner:** Weak IKM supplied by caller. **Owner:** Systems (`[E-03]`) & Crypto (`[S-04]`).
- **Verification Evidence:** `tests/kdf_tests.rs`; typed contract enforcement pending `[E-03]`.

---

## 3. Prioritized Remediation & Issue Alignment

| Priority | Action Item | Target Issue & Subsystem | Status & Deliverable |
| :--- | :--- | :--- | :---: |
| **P1 — Must-Fix** | Enforce mandatory min/max policy bounds on `Argon2idConfig` (`memory_kib`, `iterations`, `lanes`) to prevent DoS and weak parameters. | `[S-02]`, `[S-03]` (`ark-crypto`) | Open (Crypto implementation) |
| **P1 — Must-Fix** | Integrate `ark-crypto` into root Cargo workspace with `#![forbid(unsafe_code)]` and strict workspace lints. | `[S-01]` (`ark-crypto`) | Open (Crypto implementation) |
| **P2 — Should-Fix** | Establish formalized, prefix-bound domain separation registry for HKDF contexts. | `[S-04]` (`ark-crypto`) | Open (Crypto implementation) |
| **P2 — Should-Fix** | Implement adversarial sentinel secret-leakage tests and bounded input tests. | `[A-04]` (`ark-core` / security) | Test suite drafted on `test/adversarial-negative-tests` |
| **P3 — Required Gate** | Complete independent dependency advisory review and Cargo.lock audit. | `[A-06]` (Security) | Initial audit report delivered on `docs/crypto-security-review` |
| **P3 — Architecture** | Define typed IKM wrapper at Core-to-Crypto boundary to reject raw passwords. | `[E-03]` (Systems Engineering) | Open (Architecture contract) |


---

## 4. API reconciliation addendum — 2026-11-20

This addendum supersedes the implementation descriptions above wherever they conflict with the current KDF API on `fix/crypto-kdf-production-api-boundary` (base: `main` at `68dad185bd5572f3abd557a61059415a932ca95d`; KDF boundary head at the time of reconciliation: `552041395ee9097902eb6a7d1c934ed55b57a99f`).

### 4.1 Current KDF API facts

- Argon2id production derivation is exposed through `derive_kek` and `derive_kek_with_salt`. The production path enforces password bounds, salt length/non-degeneracy, and `Argon2idConfig::validate()` before invoking Argon2id. The configuration fields remain publicly constructible, so validation at the derivation boundary is essential and must remain covered by regression tests.
- Argon2id policy bounds currently enforced by the implementation are 16–256 MiB memory, 1–10 iterations, and 1–8 lanes. These are hard resource bounds; they are not proof of acceptable latency on every target platform.
- Production HKDF derivation is exposed through `derive_purpose_subkey(HkdfIkm, salt, HkdfKeyPurpose)`. The lower-level raw-slice `derive_subkey` is private.
- `HkdfContext::parse` accepts only the exact six registered `ARK/v1/<purpose>` labels. Unknown and ad-hoc purposes are rejected.
- RFC 9106 and RFC 5869 conformance helpers are test-scoped and are not re-exported as production API.
- `HkdfIkm` enforces a 16-byte minimum and redacts Debug output, but it is a borrowed wrapper. It does not prove entropy, randomness, approved provenance, or that the bytes are not a human password. The producer/caller remains responsible for provenance; raw passwords must not be supplied as HKDF IKM.

### 4.2 Threat disposition changes

| Threat | Updated status | Evidence boundary / residual risk |
|---|---|---|
| T-03 — KDF resource exhaustion | **Partially mitigated by implementation** | Production Argon2id validates hard bounds even when callers construct the public config fields directly. The maximum permitted workload can still be expensive under concurrency; caller-level admission control/rate limiting and platform calibration remain outside this primitive. |
| T-04 — HKDF context confusion | **Mitigated for the current registered API surface** | Production derivation selects a closed `HkdfKeyPurpose`; parser rejects unregistered labels. Future purposes require a reviewed registry/API change. Domain separation does not provide authorization or protect compromised IKM. |
| T-08 — weak/short IKM misuse | **Partially mitigated** | The raw-slice derivation helper is private and the public path requires `HkdfIkm`; however, any caller can wrap an arbitrary slice of at least 16 bytes. Length validation is not provenance or entropy validation, so caller misuse with a long password remains possible absent a stronger source-typed contract. |
| KAT helper exposure | **Addressed in the current branch** | RFC conformance helpers are `cfg(test)` and internal; verify with production build/API inspection and tests on the final integrated commit. |

### 4.3 Verification status

The API reconciliation above is based on source inspection and exact-head CI evidence. The candidate's Rust tests and Clippy passed CI #32/#42; the separate API-connected fuzz harness completed two bounded smoke runs, most recently KDF fuzz smoke #7 on head `be376f62ee454ca359f79c278f69fe3d882e3231`. This is not a sustained fuzz campaign or a cross-platform campaign.

### 4.4 Required follow-up

1. Keep this threat model synchronized with the final Crypto API selected by E-03 and PR #3.
2. Preserve the distinction between IKM shape validation and provenance/entropy guarantees.
3. Add caller-level resource admission controls when KDF calls are exposed to untrusted or concurrent requests.
4. Extend the bounded smoke evidence into an owner-reviewed sustained fuzz campaign; retain exact toolchain, duration, corpus, coverage, RSS and crash triage.
