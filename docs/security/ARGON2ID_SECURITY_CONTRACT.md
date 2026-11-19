# ARK Argon2id Security Contract, Salt Policy, and Parameter Calibration

- **Status:** Normative KDF Subsystem Contract (`[S-02]` / Issue #5 and `[S-03]` / Issue #6)
- **Owning Subsystem:** `ark-crypto` (`crates/ark-crypto/src/kdf/argon2id.rs`)
- **Author:** Saleh (`@ZeroTrace79`)
- **Related ADRs & Invariants:** ADR-0003, ADR-0005, `ARK-SEC-013`, `ARK-SEC-028`

---

## 1. Purpose and Scope

This document specifies the normative security contract, salt policy, input/output bounds, and calibrated parameter bounds for the Argon2id (`Version::V0x13`) key-encryption key (KEK) derivation interface in `ark-crypto`.

This contract covers **KDF primitive behavior only**. It does not define password database storage, login rate-limiting services, key-wrapping formats, AEAD encryption, or workspace recovery ceremonies.

---

## 2. Argon2id Security Contract and Salt Policy (`[S-02]`)

### 2.1 Password Input Semantics and Bounds

- **Byte Semantics:** `password: &[u8]` is treated as an opaque byte sequence. `ark-crypto` intentionally does **not** perform Unicode normalization (e.g., NFC/NFKC) or whitespace trimming so that the cryptographic primitive remains encoding-agnostic and deterministic across platforms. Callers at the UI or session boundary must apply canonical UTF-8 encoding rules before invoking `derive_kek` if human text passphrases are used.
- **Empty Input Behavior:** Zero-length password slices (`password.is_empty()`) are rejected fail-closed with `Err(Argon2Error::EmptyPassword)` before allocating any Argon2id memory blocks.
- **Maximum Input Bound:** Passwords exceeding `MAX_PASSWORD_LEN` (`1024` bytes) are rejected fail-closed with `Err(Argon2Error::PasswordTooLong)`. This prevents unbounded pre-hashing overhead while accommodating long high-entropy passphrases and machine-generated secrets.

### 2.2 Salt Ownership, Generation, and Validation Policy

- **Generation Responsibility:** `ark-crypto` is a deterministic KDF primitive crate and does not embed an operating-system RNG dependency in its core KDF slice. The calling subsystem (e.g., Workspace creation or key-rotation coordinator) MUST generate a fresh, cryptographically random salt for every new workspace or password change using an approved OS-backed CSPRNG.
- **Accepted Salt Length Bounds:**
  - Minimum length: `MIN_SALT_LEN = 16` bytes (128 bits, aligning with RFC 9106 Section 3.1).
  - Recommended default length: `DEFAULT_SALT_LEN = 32` bytes (256 bits).
  - Maximum length: `MAX_SALT_LEN = 64` bytes (512 bits).
  - Inputs outside `16..=64` bytes return `Err(Argon2Error::InvalidSaltLength)`.
- **Non-Degeneracy Enforcement (`Argon2Salt`):**
  - Both `Argon2Salt::from_slice` and `derive_kek` reject all-zero salts (`bytes.iter().all(|&b| b == 0)`) with `Err(Argon2Error::DegenerateSalt)`. This guards against uninitialized buffer bugs (`[0u8; 16]` or `[0u8; 32]`) at caller boundaries.
- **Uniqueness and Non-Secrecy Rules:**
  - Every workspace creation or passphrase re-keying event MUST use a freshly generated salt.
  - Fixed, static, reused, or "secret" salts MUST NEVER be used as a substitute for password entropy. Salts are stored alongside workspace KDF metadata to prevent precomputation and cross-workspace rainbow-table attacks.

### 2.3 Output Semantics and Secret Handling

- **Output Length and Type:** `derive_kek` and `derive_kek_with_salt` always return `Result<Zeroizing<[u8; KEY_LEN]>, Argon2Error>` where `KEY_LEN = 32` (256 bits).
- **Zeroization Lifecycle:** The output array is wrapped in `zeroize::Zeroizing` so that the derived KEK buffer is automatically scrubbed on drop. Callers are responsible for zeroizing their own input `password` buffers after calling `derive_kek`.
- **Single-Purpose Rule:** The 256-bit KEK returned by `derive_kek` MUST NOT be reused directly across multiple unrelated cryptographic algorithms or domains. If multiple subkeys are required from a single root key, callers must derive purpose-separated subkeys via `derive_subkey` (`HKDF-SHA-256`).
- **Sanitized Error Taxonomy (`ARK-SEC-028`):** `Argon2Error` is a fieldless C-style enum (`EmptyPassword`, `PasswordTooLong`, `InvalidSaltLength`, `DegenerateSalt`, `InvalidParameters`, `DerivationFailed`) whose `Display` and `Debug` implementations emit fixed static text and never include password bytes, salt bytes, or derived key material. Furthermore, `Argon2Salt`'s `Debug` implementation intentionally redacts raw salt bytes, printing only the validated byte length.

---

## 3. Argon2id Parameter Policy and Benchmark Evidence (`[S-03]`)

### 3.1 Enforced Parameter Bounds (`Argon2idConfig`)

To remediate `SEC-KDF-01` (local resource-exhaustion DoS via untrusted or corrupted metadata headers), both `Argon2idConfig::new` / `Argon2idConfig::validate` and `derive_kek` enforce strict lower and upper bounds before calling `argon2::Params::new`:

| Parameter | Minimum Floor | Recommended Default | Hard Upper Ceiling | Rationale |
| :--- | :---: | :---: | :---: | :--- |
| **`memory_kib`** | `16_384` (16 MiB) | `65_536` (64 MiB) | `262_144` (256 MiB) | Prevents weak low-memory configurations (< 16 MiB) while blocking multi-gigabyte OOM / DoS allocations (> 256 MiB). |
| **`iterations`** (`t`) | `1` | `3` | `10` | Ensures at least one full memory pass while capping CPU stall time under malformed header inputs. |
| **`lanes`** (`p`) | `1` | `1` | `8` | Bounds thread/lane fan-out and satisfies RFC 9106 `m >= 8 * p`. |
| **`output_len`** | `32` bytes | `32` bytes | `32` bytes | Fixed 256-bit KEK length (`KEY_LEN`). |

> **Bypass Prevention:** Even if a caller constructs `Argon2idConfig` via struct literals (`Argon2idConfig { memory_kib, iterations, lanes }`), `derive_kek` unconditionally invokes `config.validate()` prior to memory allocation. Only the dedicated RFC 9106 test helper (`derive_argon2id_rfc9106_kat`) allows low-memory RFC conformance vectors (`m = 32 KiB`), while still enforcing the hard upper DoS ceilings (`MAX_MEMORY_KIB`, `MAX_ITERATIONS`, `MAX_LANES`).

### 3.2 Reproducible Benchmark Environment and Methodology

- **Machine & CPU:** Intel(R) Core(TM) i3-10100F CPU @ 3.60 GHz (4 physical cores, 8 logical processors)
- **System RAM:** 16 GiB (`17,022,464,000` bytes physical memory)
- **Operating System:** Microsoft Windows 10 (`NT 10.0.19045.0`, x86_64)
- **Rust Toolchain:** `rustc 1.100.0 (0ef1bc386 2026-11-09)` / `cargo 1.100.0` (`stable-x86_64-pc-windows-gnu`, `--release` optimized profile)
- **Sample Methodology:** 1 warm-up derivation followed by `N = 5` timed derivations per profile using `std::time::Instant` around `derive_kek` (32-byte salt, 38-byte test passphrase).
- **Reproducible Command:**
  ```powershell
  cargo test --package ark-crypto --test kdf_tests --release
  ```

### 3.3 Recorded Benchmark Results

| Profile Label | `memory_kib` | `iterations` (`t`) | `lanes` (`p`) | Peak KDF Block Allocation | Mean Latency | Median Latency | Min Latency | Max Latency |
| :--- | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: |
| **Minimum Floor** | `16_384` (16 MiB) | `1` | `1` | 16 MiB | `14.63 ms` | `14.82 ms` | `13.15 ms` | `16.36 ms` |
| **Low-Memory Interactive** | `32_768` (32 MiB) | `2` | `1` | 32 MiB | `56.60 ms` | `55.03 ms` | `51.62 ms` | `63.51 ms` |
| **Baseline Default (`Default`)** | `65_536` (64 MiB) | `3` | `1` | 64 MiB | `155.59 ms` | `149.94 ms` | `147.30 ms` | `175.67 ms` |
| **Parallel Baseline** | `65_536` (64 MiB) | `3` | `2` | 64 MiB | `154.45 ms` | `152.46 ms` | `147.95 ms` | `164.49 ms` |
| **High-Security Desktop** | `131_072` (128 MiB) | `3` | `2` | 128 MiB | `314.11 ms` | `308.59 ms` | `301.14 ms` | `339.49 ms` |
| **Maximum Policy Ceiling** | `262_144` (256 MiB) | `4` | `4` | 256 MiB | `841.65 ms` | `837.65 ms` | `823.30 ms` | `867.57 ms` |

> **Scope Limitation:** These measurements establish reproducible baseline evidence on a single 4-core/8-thread x86_64 Windows workstation. They are **not** a claim of universal performance suitability across mobile, embedded, or low-power ARM targets.

### 3.4 Concurrency, Denial-of-Service, and Caller Responsibilities

- **Library vs. Caller DoS Boundary:**
  - `ark-crypto` enforces per-call resource caps (`<= 256 MiB` RAM, `<= 10` iterations) so a single malformed header cannot exhaust host memory.
  - Because `derive_kek` allocates up to `memory_kib` KiB per active invocation, concurrent calls multiply memory pressure (`K` concurrent calls at the 256 MiB ceiling require `K * 256 MiB`).
  - Higher-level ARK subsystems (Session/Workspace unlock broker) MUST serialize interactive unlock requests or enforce a strict concurrency semaphore (e.g., at most 1–2 concurrent Argon2id derivations in a local desktop workspace) and apply exponential backoff on repeated authentication failures.
