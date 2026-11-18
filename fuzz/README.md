# ARK Fuzzing Harness & Invocation Specification

**Status:** Scaffold & Target Specification (`[A-05]` / Issue #13) — Distinguishing Core Targets from KDF Scaffolds.  
**Execution Status:** **Not Run / Not Executed in Automated CI.** No execution duration, coverage percentage, or absence-of-crash claims are made for these targets in the current repository state.

---

## 1. Classification of Current Fuzz Targets

The fuzz package contains two distinct classes of targets:

### 1.1 Core API Connected Targets
- **`fuzz_core_ids`:** Directly invokes `ark-core::ids::*` (`from_bytes`, hex formatting, round-trip invariants). Connected to implemented Core public APIs.
- **`fuzz_core_versions`:** Directly invokes `ark-core::versions::*` (`ApplicationVersion`, `StorageFormatVersion`, etc.). Connected to implemented Core public APIs.

### 1.2 KDF Input Scaffolds (Input-Shape Only)
- **`fuzz_argon2id`:** An **input-shape validation scaffold**. It decodes structured byte slices to evaluate parameter boundary logic and length clamping. **Important:** It does **not** directly invoke `ark_crypto::kdf::derive_kek` in the current un-integrated workspace layout. It is a harness scaffold awaiting `[S-01]` workspace integration.
- **`fuzz_hkdf`:** An **input-shape validation scaffold**. It evaluates context slicing and IKM length bounds around `MAX_CONTEXT_LEN = 256`. **Important:** It does **not** directly execute `ark_crypto::kdf::derive_subkey` in the current layout.

---

## 2. Resource Bounding & Invocation Specification

When executed with `cargo-fuzz` (under a nightly toolchain with ASan):

```bash
# Core Domain Targets (Direct API Calls)
cargo fuzz run fuzz_core_ids -- -rss_limit_mb=1024
cargo fuzz run fuzz_core_versions -- -rss_limit_mb=1024

# KDF Scaffold Targets (Parameter Bounds Simulation)
cargo fuzz run fuzz_argon2id -- -rss_limit_mb=1024
cargo fuzz run fuzz_hkdf -- -rss_limit_mb=1024
```

---

## 3. Crash Triage & Limitation Notice

- Any crash discovered in future fuzz runs must be minimized, converted to a regression test, and tracked before closing.
- Fuzzing provides empirical evidence of robustness against malformed inputs; it is **not** mathematical proof of absence of vulnerabilities or cryptographic correctness.
