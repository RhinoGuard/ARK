# ARK KDF Fuzzing Harness

**Status:** API-connected fuzz targets implemented; execution evidence is pending.

This package invokes the actual public `ark-crypto` KDF APIs. It does not simulate the validation logic locally.

## Targets

- `fuzz_argon2id` calls `ark_crypto::kdf::derive_kek` with a fixed, policy-valid minimum-cost configuration. The input is bounded to a 1–1024 byte password followed by a 16-byte salt. The harness therefore exercises the real password/salt validation and Argon2id implementation without allowing fuzz input to select unbounded cost parameters.
- `fuzz_hkdf` calls `HkdfContext::parse`, `HkdfIkm::from_slice`, and `derive_purpose_subkey`. It exercises malformed/unknown context values, IKM length handling, and each registered purpose.

The fuzz targets are robustness tests for the current public API. They do not prove entropy or provenance of IKM, cryptographic correctness, absence of vulnerabilities, or correctness of future key hierarchy behavior. The committed smoke corpus includes one valid-shaped Argon2id password/salt input and one 32-byte HKDF IKM seed so the fuzzer can reach the valid derivation paths early.

## Prerequisites

Install a nightly Rust toolchain and `cargo-fuzz`. Run from the repository root:

```bash
cargo +nightly fuzz run fuzz_argon2id -- -max_total_time=20 -max_len=1040 -rss_limit_mb=1024 -timeout=10
cargo +nightly fuzz run fuzz_hkdf -- -max_total_time=20 -max_len=1024 -rss_limit_mb=1024 -timeout=10
```

The Argon2id target uses the current minimum production memory cost (16 MiB) and one fixed low-cost profile. This is intentionally resource-bounded, but each successful input still performs real Argon2id work; choose a bounded duration appropriate to the machine.

## CI smoke workflow

`.github/workflows/kdf-fuzz-smoke.yml` installs nightly Rust and `cargo-fuzz`, checks the standalone package, compiles both targets, then runs each with a 20-second bounded smoke budget, a 1 GiB RSS limit, and a 10-second per-input timeout. It uploads any generated corpus/artifacts. This is a smoke test, not a sustained fuzz campaign.

## Evidence discipline

The workflow has been added but its first run has not yet been observed. Do not report A-05 as validated until that workflow actually passes and its run ID/head are recorded. Before security acceptance, also perform a longer owner-reviewed fuzz session and record exact commit SHA, Rust/nightly version, cargo-fuzz version, command, duration, corpus, coverage, crash count, and triage outcome. Convert every discovered crash into a minimized regression test before acceptance.
