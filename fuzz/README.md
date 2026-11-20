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

The dedicated workflow has executed successfully on the latest PR #26 head. The latest run and exact toolchain/corpus evidence are recorded below. This establishes only bounded smoke execution; before security acceptance, perform a longer owner-reviewed fuzz session and record exact commit SHA, Rust/nightly version, cargo-fuzz version, command, duration, corpus, coverage, crash count, and triage outcome. Convert every discovered crash into a minimized regression test before acceptance.


## Recorded smoke evidence

Three clean bounded smoke runs are now recorded. The latest and canonical repeat is [KDF fuzz smoke #10](https://github.com/RhinoGuard/ARK/actions/runs/44624894126) on current candidate head `076c84082a6259b9c1c724327bdc32ba614581b8`; earlier successful runs #4 and #7 are retained as repeat samples.

- Environment: GitHub-hosted Ubuntu runner.
- Toolchain: `rustc 1.102.0-nightly (65ce8611f 2026-11-18)`; `cargo 1.102.0-nightly (f93c676e3 2026-11-15)`; `cargo-fuzz 0.13.2`.
- Both target formatting and compilation passed; both targets exited successfully; no crash was reported; artifact upload succeeded.

| Target | Bounded duration | Executions | Final libFuzzer counters | Peak RSS |
|---|---:|---:|---:|---:|
| `fuzz_argon2id` | 21 s | 754 | `cov: 338`, `ft: 346`, corpus 7 / 103 B | 330 MiB |
| `fuzz_hkdf` | 21 s | 4,633,173 | `cov: 101`, `ft: 115`, corpus 25 / 2,156 B | 464 MiB |

Commands used `-max_total_time=20`, `-rss_limit_mb=1024`, a 10-second per-input timeout, and target-specific maximum input lengths (1,040 bytes for Argon2id; 1,024 bytes for HKDF). The observed wall-time is 21 seconds per target due to libFuzzer's stop/report boundary.

The uploaded artifact is [`kdf-fuzz-smoke-d8fe9afbb3f9d2299cafde0cbb390f5c0b4f4188`](https://github.com/RhinoGuard/ARK/actions/runs/44624894126/artifacts/14579133051), SHA-256 `2d80c6fa8c221d41b1705dffd5bcbc032a604d7127fc8ef0e3d80ac910dff210`.

These are three 20-second smoke samples, not a sustained campaign, cross-platform fuzz results, cryptographic proof, or assurance that no latent crash exists. Retain the original run logs and artifact as the reproducibility record.
