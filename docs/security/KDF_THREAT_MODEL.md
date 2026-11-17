# ARK KDF Foundation Threat Model (Argon2id & HKDF-SHA-256)

**Status:** Proposed Working Draft for Foundation Stabilization v1 (`[A-02]` / Issue #10) — Pending Team Review.  
**Scope:** Strictly scoped to the current key-derivation foundation (`ark-crypto` PR #3: `src/kdf/argon2id.rs`, `src/kdf/hkdf_sha256.rs`) and its integration boundary with `ark-core` (PR #2).  
**Out of Scope:** Unimplemented AEAD chunk encryption, storage formats, WAL/journaling, recovery envelopes, and full-product security claims.

> **Evidence & Honesty Rule:**
> In accordance with `AGENTS.md` and `docs/architecture/SECURITY_INVARIANTS.md`, no threat mitigation is considered established or verified solely by virtue of being documented. Implemented mitigations must link to concrete code or executable tests. All gaps, unenforced parameters, and caller responsibilities are explicitly classified.

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
- **Verification Evidence:** Bounded adversarial tests in `[A-04]`; parameter policy implementation in `[S-03]`.

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
- **Residual Risk & Owner:** Callers logging raw inputs. **Owner:** Core / Crypto / Security (`[A-04]` sentinel tests).
- **Verification Evidence:** `src/kdf/argon2id.rs`, `src/kdf/hkdf_sha256.rs`; explicit sentinel leakage tests in `[A-04]`.

---

### T-06: Cryptographic Dependency & Supply-Chain Compromise
- **Asset:** Primitives integrity (`argon2`, `hkdf`, `sha2`, `zeroize`).
- **Attacker Capability:** Upstream vulnerability, supply-chain injection, or unmaintained transitive dependencies.
- **Preconditions:** Inclusion of unpinned or vulnerable crate versions in `Cargo.lock`.
- **Attack Path:** Known vulnerability in upstream hashing or zeroization crate exploited to bypass protections.
- **Impact:** Weakened encryption, side-channel leakage, or memory corruption.
- **Likelihood / Risk Rationale:** Low-Medium; dependencies are standard RustCrypto crates, but independent advisory audit has not yet run.
- **Existing Mitigation:** Dependency versions declared in `Cargo.toml`.
- **Mitigation Status:** `Pending Independent Audit` (formal advisory inspection, Cargo.lock audit, and feature-flag verification are assigned to `[A-06]`).
- **Residual Risk & Owner:** Transitive security advisories. **Owner:** Security (`[A-06]`).
- **Verification Evidence:** Pending `[A-06]` security audit deliverable.

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

| Priority | Action Item | Target Issue & Subsystem | Action Type |
| :--- | :--- | :--- | :---: |
| **P1 — Must-Fix** | Enforce mandatory min/max policy bounds on `Argon2idConfig` (`memory_kib`, `iterations`, `lanes`) to prevent DoS and weak parameters. | `[S-02]`, `[S-03]` (`ark-crypto`) | Implementation |
| **P1 — Must-Fix** | Integrate `ark-crypto` into root Cargo workspace with `#![forbid(unsafe_code)]` and strict workspace lints. | `[S-01]` (`ark-crypto`) | Implementation |
| **P2 — Should-Fix** | Establish formalized, prefix-bound domain separation registry for HKDF contexts. | `[S-04]` (`ark-crypto`) | Implementation |
| **P2 — Should-Fix** | Implement adversarial sentinel secret-leakage tests and bounded input tests. | `[A-04]` (`ark-core` / security) | Testing |
| **P3 — Required Gate** | Complete independent dependency advisory review and Cargo.lock audit. | `[A-06]` (Security) | Audit |
| **P3 — Architecture** | Define typed IKM wrapper at Core-to-Crypto boundary to reject raw passwords. | `[E-03]` (Systems Engineering) | Architecture |
