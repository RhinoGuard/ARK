# ARK KDF Foundation Threat Model (Argon2id & HKDF-SHA-256)

**Status:** Initial baseline for Foundation Stabilization v1 (`[A-02]` / Issue #10).  
**Scope:** Strictly scoped to the current key-derivation foundation (`ark-crypto` PR #3: `src/kdf/argon2id.rs`, `src/kdf/hkdf_sha256.rs`) and its boundary with `ark-core` (PR #2).  
**Out of Scope:** Unimplemented AEAD chunk encryption, storage formats, WAL/journaling, recovery envelopes, and full-product security claims.

---

## 1. Scope & Assumptions

The current KDF foundation provides two primitives:
1. **`derive_kek(password, salt, config)`** (`src/kdf/argon2id.rs`): Derives a 256-bit Key Encryption Key (KEK) from a user password and caller-supplied salt using Argon2id (`Version::V0x13`).
2. **`derive_subkey(input_key_material, salt, context)`** (`src/kdf/hkdf_sha256.rs`): Derives 256-bit domain-separated subkeys using HKDF-SHA-256.

**Explicit Non-Guarantees:**
- This layer does not generate salts (delegated to caller/OS CSPRNG).
- This layer does not enforce password strength policies or rate-limiting.
- `Zeroize` / `Zeroizing` reduces exposure windows in Rust heap/stack allocations but cannot guarantee physical erasure from CPU registers, compiler-elided copies, OS swap/pagefile, hibernation, or crash dumps.

---

## 2. Threat Analysis Matrix

### T-01: Offline Password Guessing & Weak-Password Risk
- **Asset:** User password and derived 256-bit KEK.
- **Attacker Capability:** Offline attacker with read access to persisted encrypted workspace/package headers and salt.
- **Preconditions:** Weak or dictionary-guessable user password, or KDF parameters set too low for target hardware.
- **Attack Path:** Attacker extracts salt and KDF parameters from storage/package metadata and runs GPU/ASIC offline brute-force against Argon2id.
- **Impact:** Compromise of KEK, unwrapping of Workspace/Package Master Key, and full decryption of protected data.
- **Likelihood / Risk Rationale:** High if weak passwords are permitted or if `Argon2idConfig` defaults (`64 MiB`, `3 iterations`, `1 lane`) are lowered without benchmarking.
- **Existing Mitigation:** Uses `argon2` (`Algorithm::Argon2id`, `Version::V0x13`) with default `memory_kib: 65536`, `iterations: 3`, `lanes: 1`, and rejects empty passwords (`Argon2Error::EmptyPassword`).
- **Mitigation Status:** `Partially Implemented` (password strength policy and hardware-benchmarked parameter policy are not yet enforced).
- **Residual Risk & Owner:** Residual risk remains for low-entropy passwords. **Owner:** Crypto (`[S-03]` parameter policy) & Policy/UI (password strength UX).
- **Verification Test / Evidence:** `src/kdf/argon2id.rs` (`derive_kek` empty-password check); `[S-03]` benchmark evidence pending.

---

### T-02: Salt Generation Failure, Salt Reuse, & Incorrect Salt Handling
- **Asset:** 256-bit KEK uniqueness across workspaces, packages, and password rotations.
- **Attacker Capability:** Passive/offline attacker observing multiple encrypted workspaces or password-rotated headers.
- **Preconditions:** Caller supplies a static, predictable, or reused salt (e.g., all-zeroes or constant bytes), or CSPRNG failure is silently ignored.
- **Attack Path:** Identical `(password, salt)` pairs produce identical KEKs, enabling precomputed/multi-target attacks or leaking password equality across vaults.
- **Impact:** Loss of per-vault/per-package key independence.
- **Likelihood / Risk Rationale:** Medium; `derive_kek` accepts `&[u8]` directly from callers without generating or tracking salt entropy.
- **Existing Mitigation:** `derive_kek` enforces salt length `16..=64` bytes (`Argon2Error::InvalidSaltLength` in `src/kdf/argon2id.rs`).
- **Mitigation Status:** `Caller Responsibility` (entropy quality and uniqueness are not enforced by `derive_kek`).
- **Residual Risk & Owner:** Caller may pass a 16-byte constant or weak PRNG output. **Owner:** Crypto (`[S-02]` salt policy) & Core/Storage callers.
- **Verification Test / Evidence:** `tests/kdf_tests.rs` (`argon2id_rejects_invalid_salt`, `argon2id_changes_when_salt_changes`).

---

### T-03: KDF Resource Exhaustion & Attacker-Controlled Parameter Abuse
- **Asset:** Host CPU, RAM, and application availability (DoS).
- **Attacker Capability:** Attacker supplies crafted/inflated `Argon2idConfig` values (e.g., via untrusted `.arkvault` package headers) or oversized password buffers.
- **Preconditions:** Untrusted metadata is parsed and passed to `derive_kek` without upper-bound validation.
- **Attack Path:** Attacker sets `memory_kib` to multi-gigabyte values or `iterations` to millions, causing OOM crash or thread starvation.
- **Impact:** Application crash or Denial of Service.
- **Likelihood / Risk Rationale:** High once untrusted package import is wired up if bounds are not enforced.
- **Existing Mitigation:** Password length is capped at `1024` bytes (`Argon2Error::PasswordTooLong` in `src/kdf/argon2id.rs`). `argon2::Params::new` validates basic library limits.
- **Mitigation Status:** `Partially Implemented` (password length is bounded, but `Argon2idConfig` fields are `pub` with no ARK-enforced upper/lower policy bounds).
- **Residual Risk & Owner:** Unbounded `memory_kib` / `iterations` / `lanes` accepted by `Argon2idConfig`. **Owner:** Crypto (`[S-02]`/`[S-03]`) & Package Importer.
- **Verification Test / Evidence:** `src/kdf/argon2id.rs` (`password.len() > 1024`); adversarial bounds tests planned in `[A-04]`.

---

### T-04: HKDF Context Confusion & Cross-Purpose Key Reuse
- **Asset:** Domain-separated subkeys (Project Key, File Key, Metadata Key, etc.).
- **Attacker Capability:** Attacker induces two subsystems to use the same HKDF `context` string or swaps ciphertexts between contexts.
- **Preconditions:** Callers pass ad-hoc `&[u8]` context strings without a centralized registry or versioned domain-separation format.
- **Attack Path:** Two distinct purposes (e.g., content encryption vs. metadata authentication) accidentally share the same `context` bytes, deriving identical subkeys from the same IKM.
- **Impact:** Cross-protocol/cross-context key reuse, potentially breaking AEAD nonce uniqueness or authentication guarantees.
- **Likelihood / Risk Rationale:** Medium; `derive_subkey` accepts any non-empty `&[u8]` up to `MAX_CONTEXT_LEN` (256 bytes).
- **Existing Mitigation:** Rejects empty context (`HkdfError::EmptyContext`) and context `> 256` bytes (`HkdfError::ContextTooLong` in `src/kdf/hkdf_sha256.rs`).
- **Mitigation Status:** `Partially Implemented` (length/non-empty checked, but structured context registry is not yet enforced).
- **Residual Risk & Owner:** Ad-hoc context strings by future callers. **Owner:** Crypto (`[S-04]` HKDF domain separation contract).
- **Verification Test / Evidence:** `tests/kdf_tests.rs` (`hkdf_separates_distinct_contexts`, `hkdf_rejects_empty_context`).

---

### T-05: Secret Exposure Through Errors, Logs, Debug, Panics, & Test Output
- **Asset:** Passwords, KEKs, IKMs, and derived subkeys.
- **Attacker Capability:** Local unprivileged user, log collector, or crash-report analyzer reading stdout/stderr, logs, or error strings.
- **Preconditions:** Error types, `Debug` implementations, or panic messages format sensitive buffers.
- **Attack Path:** A panic or error path prints input key material or password bytes into logs or crash dumps.
- **Impact:** Direct disclosure of secret key material.
- **Likelihood / Risk Rationale:** Low for current KDF error enums (`Argon2Error`, `HkdfError` are unit-variant enums with static messages), but possible in caller `Debug` structs or test assertions.
- **Existing Mitigation:** `Argon2Error` and `HkdfError` contain no secret payloads; `derive_kek` and `derive_subkey` return `Zeroizing<[u8; 32]>` and avoid panicking on invalid inputs.
- **Mitigation Status:** `Implemented` (for KDF error types); `Caller Responsibility` (for callers handling input slices and `Zeroizing` outputs).
- **Residual Risk & Owner:** Callers logging raw password/IKM slices before calling KDF. **Owner:** Crypto & Security (`[A-04]` sentinel secret-leakage tests).
- **Verification Test / Evidence:** `src/kdf/argon2id.rs` & `src/kdf/hkdf_sha256.rs` error enums; sentinel tests planned in `[A-04]`.

---

### T-06: Dependency Compromise & Supply-Chain Risks
- **Asset:** Integrity of cryptographic primitives (`argon2`, `hkdf`, `sha2`, `zeroize`).
- **Attacker Capability:** Upstream crate maintainer compromise, typosquatting, or unpatched vulnerability in transitive dependencies.
- **Preconditions:** Unpinned/unaudited crate versions or unreviewed feature flags.
- **Attack Path:** Malicious or vulnerable code enters through `Cargo.toml` / `Cargo.lock` updates.
- **Impact:** Key exfiltration, weakened cryptography, or memory corruption.
- **Likelihood / Risk Rationale:** Low-Medium; RustCrypto crates are widely used, but `Cargo.lock` and transitive trees require periodic advisory checks.
- **Existing Mitigation:** Uses standard RustCrypto crates (`argon2 = "0.5"`, `hkdf = "0.12"`, `sha2 = "0.10"`, `zeroize = "1.7"`).
- **Mitigation Status:** `Partially Implemented` (independent dependency & advisory audit scheduled in `[A-06]`).
- **Residual Risk & Owner:** Transitive dependencies not yet audited against RustSec. **Owner:** Security (`[A-06]`).
- **Verification Test / Evidence:** Pending `[A-06]` dependency inventory and advisory report.

---

### T-07: Memory Lifetime / Zeroization Limitations
- **Asset:** Plaintext passwords, KEKs, and subkeys in process RAM.
- **Attacker Capability:** Local privileged attacker, memory-dump access, or swap/hibernation file reader.
- **Preconditions:** Secret material resides in RAM during/after KDF execution.
- **Attack Path:** OS swaps memory pages to disk, or `password: &[u8]` / `input_key_material: &[u8]` owned by the caller is not zeroized after `derive_kek` / `derive_subkey` returns.
- **Impact:** Post-use recovery of passwords or derived keys from RAM/swap.
- **Likelihood / Risk Rationale:** Medium; `Zeroizing<[u8; 32]>` scrubs the output buffer on drop, but input slices (`&[u8]`) are borrowed and cannot be scrubbed by the KDF functions.
- **Existing Mitigation:** Output keys are wrapped in `zeroize::Zeroizing<[u8; 32]>`.
- **Mitigation Status:** `Partially Implemented` (outputs are zeroized; inputs and OS memory locking `mlock`/`VirtualLock` are caller/runtime responsibilities).
- **Residual Risk & Owner:** Caller-owned input buffers and OS swap/crash dumps. **Owner:** Crypto & Runtime (`[A-02]` documented limitation).
- **Verification Test / Evidence:** `Zeroizing` usage in `src/kdf/argon2id.rs` and `src/kdf/hkdf_sha256.rs`.

---

### T-08: Misuse at API Boundaries & Unsafe Caller Assumptions
- **Asset:** Key hierarchy integrity (`Password -> KEK -> Master Key -> Subkeys`).
- **Attacker Capability:** Indirect; developer/caller misuses KDF APIs.
- **Preconditions:** `derive_subkey` accepts any non-empty `input_key_material` (even 1 byte or a raw low-entropy password), and `derive_kek` accepts any `Argon2idConfig`.
- **Attack Path:** Caller passes a raw user password directly into `derive_subkey` (skipping Argon2id), or passes a 1-byte IKM, weakening subkey entropy.
- **Impact:** Fast brute-force of subkeys without Argon2id memory-hardness.
- **Likelihood / Risk Rationale:** Medium; raw `&[u8]` parameters lack strong domain newtypes.
- **Existing Mitigation:** Rejects empty `input_key_material` (`HkdfError::EmptyInputKeyMaterial`).
- **Mitigation Status:** `Caller Responsibility` / `Deferred` (Core-to-Crypto typed boundary is tracked in `[E-03]`).
- **Residual Risk & Owner:** Weak IKM accepted by `derive_subkey`. **Owner:** Systems Engineering (`[E-03]`) & Crypto (`[S-04]`).
- **Verification Test / Evidence:** `src/kdf/hkdf_sha256.rs`; integration contract pending `[E-03]`.

---

## 3. Prioritized Remediation List

1. **[High / Must-Fix before Production]:** Enforce explicit minimum and maximum policy bounds on `Argon2idConfig` (`memory_kib`, `iterations`, `lanes`) inside `ark-crypto` (`[S-02]`, `[S-03]`) to prevent both weak parameters and untrusted-input DoS.
2. **[High / Must-Fix before Production]:** Integrate `ark-crypto` into the root Cargo workspace (`[S-01]`) with `#![forbid(unsafe_code)]` and workspace Clippy lints enabled.
3. **[Medium / Should-Fix]:** Enforce minimum IKM length (e.g., `>= 16` or `32` bytes) in `derive_subkey` or bind it to typed key wrappers at the Core-to-Crypto boundary (`[E-03]`, `[S-04]`).
4. **[Medium / Should-Fix]:** Standardize HKDF context labels via an explicit domain-separation registry (`[S-04]`).
5. **[Low / Informational]:** Complete RustSec dependency advisory audit (`[A-06]`) and bounded KDF fuzzing (`[A-05]`).
