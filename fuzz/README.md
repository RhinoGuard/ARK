# ARK KDF Fuzzing Harness

**Status:** API-connected targets and a bounded smoke run are validated on the staged candidate; a sustained owner-reviewed fuzz campaign remains pending.

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


## Recorded smoke evidence

Two clean bounded smoke runs are now recorded. The latest and canonical repeat is [KDF fuzz smoke #7](https://github.com/RhinoGuard/ARK/actions/runs/44624226096) on candidate head `be376f62ee454ca359f79c278f69fe3d882e3231`; the first successful run was [#4](https://github.com/RhinoGuard/ARK/actions/runs/44623186546).

- Environment: GitHub-hosted Ubuntu runner.
- Toolchain: `rustc 1.102.0-nightly (65ce8611f 2026-11-18)`; `cargo 1.102.0-nightly (f93c676e3 2026-11-15)`; `cargo-fuzz 0.13.2`.
- Both target formatting and compilation passed; both targets exited successfully; no crash was reported; artifact upload succeeded.

| Target | Bounded duration | Executions | Final libFuzzer counters | Peak RSS |
|---|---:|---:|---:|---:|
| `fuzz_argon2id` | 21 s | 754 | `cov: 338`, `ft: 346`, corpus 7 / 103 B | 330 MiB |
| `fuzz_hkdf` | 21 s | 4,633,173 | `cov: 101`, `ft: 115`, corpus 25 / 2,156 B | 464 MiB |

Commands used `-max_total_time=20`, `-rss_limit_mb=1024`, a 10-second per-input timeout, and target-specific maximum input lengths (1,040 bytes for Argon2id; 1,024 bytes for HKDF). The observed wall-time is 21 seconds per target due to libFuzzer's stop/report boundary.

The uploaded artifact is [`kdf-fuzz-smoke-32b7a8302db9098a28fa637685da1aef874b2597`](https://github.com/RhinoGuard/ARK/actions/runs/44624226096/artifacts/14577108336), SHA-256 `de2ba58bd23b5b8d5cf7707d96b4a87b7ddf10a064149a478de0d8b3ed62f78a`.

These are two 20-second smoke samples, not a sustained campaign, cross-platform fuzz results, cryptographic proof, or assurance that no latent crash exists. Retain the original run logs and artifact as the reproducibility record.
