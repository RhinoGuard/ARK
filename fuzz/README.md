# ARK Fuzzing Harness & Reproducible Invocation Guide

**Status:** Hardened Fuzzing Guide (`[A-05]` / Issue #13).  
**Targets:** `fuzz_core_ids`, `fuzz_core_versions`, `fuzz_argon2id`, `fuzz_hkdf`.  
**Underlying Engine:** `libFuzzer` via `cargo-fuzz`.

---

## 1. Safety & Resource Bounds Policy

To ensure fuzzing remains safe for local machines and CI without triggering Out-Of-Memory (OOM) or system lockup:
- **`fuzz_argon2id`:** Memory cost is clamped to `<= 1024 KiB` and iterations are clamped to `<= 2`. Fuzzing uses scaled resource envelopes, never multi-gigabyte production workloads.
- **`fuzz_hkdf`:** Context buffer lengths are bounded to `<= 512` bytes to stress the `MAX_CONTEXT_LEN = 256` cutoff.
- **No Real Secrets:** Seed corpora and fuzz runs must never contain real user passwords, production keys, or recovery secrets.

---

## 2. Invocation Instructions

### 2.1 Prerequisites
```bash
cargo install cargo-fuzz
rustup default nightly   # libFuzzer requires the nightly compiler with ASan
```

### 2.2 Running Fuzz Targets
Run each target with bounded memory and time:

```bash
# Fuzz Core Domain Identifiers
cargo fuzz run fuzz_core_ids -- -max_total_time=60 -rss_limit_mb=1024

# Fuzz Semantic and Format Versions
cargo fuzz run fuzz_core_versions -- -max_total_time=60 -rss_limit_mb=1024

# Fuzz Argon2id Input Bounds
cargo fuzz run fuzz_argon2id -- -max_total_time=120 -rss_limit_mb=1024

# Fuzz HKDF-SHA-256 Input Bounds
cargo fuzz run fuzz_hkdf -- -max_total_time=60 -rss_limit_mb=1024
```

---

## 3. Seed Corpora & Crash Artifact Triage

- **Corpus Location:** `fuzz/corpus/<target_name>/` (managed by cargo-fuzz, ignored in git).
- **Artifact Handling:** Any crash artifact (`fuzz/artifacts/<target_name>/crash-*`) must be:
  1. Minimized via `cargo fuzz cmin <target_name>`.
  2. Reproduced via `cargo fuzz run <target_name> <path_to_artifact>`.
  3. Converted into a deterministic unit regression test in the relevant crate's `tests/` directory.
  4. Tracked in an actionable security issue before closing.

---

## 4. Limitation Statement

Fuzzing provides empirical evidence of robustness against malformed and boundary inputs. It does **not** constitute mathematical proof of absence of vulnerabilities, side-channel resilience, or cryptographic correctness.
