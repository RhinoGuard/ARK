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

One successful bounded smoke run is available:

- Workflow: [KDF fuzz smoke #4](https://github.com/RhinoGuard/ARK/actions/runs/44623186546)
- Tested commit: `4ec4de8d23201d7f2806b08cdccfb979b19fc0fc`
- Environment: GitHub-hosted Ubuntu runner; `rustc 1.102.0-nightly (65ce8611f 2026-11-18)`; `cargo-fuzz 0.13.2`.
- Exit status: both targets completed successfully; no crash was reported; the corpus/artifact upload step passed.

| Target | Bounded duration | Executions | Final libFuzzer counters | Peak RSS |
|---|---:|---:|---:|---:|
| `fuzz_argon2id` | 21 s | 590 | `cov: 338`, `ft: 346`, corpus 7 / 105 B | 329 MiB |
| `fuzz_hkdf` | 21 s | 3,963,943 | `cov: 101`, `ft: 115`, corpus 25 / 1,916 B | 468 MiB |

Both commands used `-max_total_time=20`, `-rss_limit_mb=1024`, a 10-second per-input timeout, and target-specific maximum input lengths (1,040 bytes for Argon2id; 1,024 bytes for HKDF). The observed wall-time is 21 seconds per target due to libFuzzer's stop/report boundary.

The uploaded artifact is [`kdf-fuzz-smoke-49235ff289a11bfc49d19ebe0632eabf00dc3369`](https://github.com/RhinoGuard/ARK/actions/runs/44623186546/artifacts/14578157451), SHA-256 `8b3c01cf26d8e85c18107d0046dec2469833f1afb33f71bfa165e6aa54d285c3`.

This result is a single 20-second smoke sample, not a sustained campaign, cross-platform fuzz result, cryptographic proof, or assurance that no latent crash exists. Retain the original run logs and artifact as the reproducibility record.
