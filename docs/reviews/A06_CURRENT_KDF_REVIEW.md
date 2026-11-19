# A-06 Current KDF Security Review — Systems Engineering Cross-Check

**Prepared by:** Systems Engineering integration review  
**Date:** 2026-11-20  
**Status:** Static reconciliation only — not Ali's independent owner review, approval, or cryptographic certification  
**Source issue:** [#14 — A-06](https://github.com/RhinoGuard/ARK/issues/14)

## 1. Review boundary and exact source revisions

Ali's original report, `docs/security/CRYPTO_DEPENDENCY_AND_IMPLEMENTATION_REVIEW.md`, is on branch `docs/crypto-security-review` (branch head checked: `c47d33495ba663367ca4ff5275f64c5297d6230b`; report blob: `19b4f8409611084803e25a6780a3b1f180ce8687`). It identifies itself as a **preliminary static review**, dated 2026-11-17, targeting PR #3 source commits `c7322e2..3fdabeb`.

That report is valuable historical evidence, but it is not a review of the present candidate API. This cross-check compares:

- Original PR #3: `feature/crypto-kdf-implementation`, head `d705efd042ca31113eab3b2bdf63fb643fd27f8b` (still open when checked).
- Hardened Crypto candidate: `fix/crypto-kdf-production-api-boundary`, head `552041395ee9097902eb6a7d1c934ed55b57a99f`.
- Current Security Validation integration branch: `feat/security-validation-kdf-fuzz-integration`, latest head `076c84082a6259b9c1c724327bdc32ba614581b8`, stacked on the hardened Crypto candidate.

The integration branch restores the missing internal KAT module and public-API KDF integration tests. [ARK CI #46](https://github.com/RhinoGuard/ARK/actions/runs/44624894201) passed on the latest exact head, including formatting, workspace build, Clippy and tests across Ubuntu, Windows, macOS and Rust 1.85 MSRV. [KDF fuzz smoke #10](https://github.com/RhinoGuard/ARK/actions/runs/44624894126) also passed on that exact head. No local Cargo commands were executed.

## 2. Executive disposition

The original two release-blocking findings and the two KDF API must-fix findings have materially changed in the hardened candidate:

- The Crypto crate is a member of the root Cargo workspace and inherits workspace lints.
- Argon2 production derivation revalidates public config fields before expensive work and enforces explicit per-call cost bounds.
- Production HKDF derivation takes `HkdfIkm` and a closed `HkdfKeyPurpose`; raw context-based `derive_subkey` is private.
- Context parsing accepts exactly the six registered `ARK/v1/<purpose>` strings.
- RFC KAT helpers are test-scoped rather than production re-exports.

This is a meaningful reduction of the specific risks Ali originally identified. It is not enough to call the KDF workstream complete: test execution on the exact integrated head, owner review, dependency audit output, platform calibration, and caller-level resource policy remain outstanding.

## 3. Finding-by-finding reconciliation

### SEC-KDF-01 — Argon2id cost configuration and resource exhaustion

**Original severity:** Release Blocker.  
**Original concern:** Public `Argon2idConfig` fields could pass extreme memory/iteration/parallelism values to the Argon2 library.

**Current evidence:** `Argon2idConfig::validate()` enforces:
- `memory_kib`: 16,384 through 262,144 (16–256 MiB);
- `iterations`: 1 through 10;
- `lanes`: 1 through 8.

The production `derive_kek` path revalidates the config at the boundary even when a caller creates the public struct directly, before invoking Argon2. The A-04 suite includes direct-construction rejection cases.

**Current disposition:** The original unbounded per-call configuration defect is **mitigated in the candidate**. This does not impose a global budget across concurrent derivations. At maximum settings, concurrent requests can still consume substantial memory and CPU. Admission control, queue limits, rate limiting, cancellation behavior and target-platform calibration belong to the caller/service/runtime boundary.

**Remaining evidence:** The A-04 negative suite passed in CI #46. Calibrate latency/memory on supported targets before final product-policy approval; CI functional success is not performance calibration.

### SEC-KDF-02 — Crypto crate outside the root workspace

**Original severity:** Release Blocker.  
**Original concern:** Standalone root package bypassed the Rust workspace established by PR #2.

**Current evidence:** The hardened branch's root manifest declares `members = ["crates/ark-core", "crates/ark-crypto"]`, retains resolver 3 and workspace package/MSRV metadata, and defines workspace Rust/Clippy lints. `crates/ark-crypto/Cargo.toml` inherits workspace edition, rust-version, license, repository and lints.

**Current disposition:** **Structurally addressed in the staged candidate.** It is not yet on `main`; PR #3's current head is still the original older revision. The fix must be integrated into the Crypto owner's PR/branch and checked by CI on its final head before this finding can be closed.

### SEC-KDF-03 — Short or low-quality HKDF IKM

**Original severity:** Must-Fix.  
**Original concern:** Any non-empty byte slice could enter HKDF, including a short token or a raw password.

**Current evidence:** `HkdfIkm::from_slice` rejects values shorter than 16 bytes and redacts Debug output. The public production path accepts `HkdfIkm` plus a `HkdfKeyPurpose`; raw `derive_subkey` is private. E-03's proposed Core-to-Crypto contract explicitly distinguishes length from provenance and forbids raw passwords as HKDF IKM.

**Current disposition:** **Partially mitigated.** The wrapper is borrowed and can be created from any arbitrary slice of at least 16 bytes. A long human password still passes the shape check. No entropy, randomness, source identity or approved producer provenance is proved by this wrapper. It also does not own or zeroize the caller's source bytes.

**Required owner decision:** Crypto and Systems must decide whether the primitive-level shape wrapper is sufficient for the current KDF milestone, or whether production IKM must be represented by an owned/source-typed key-material type that only approved producers can construct. Until then, all call sites must explicitly document source/provenance and must never pass raw passwords.

### SEC-KDF-04 — HKDF context confusion / missing purpose registry

**Original severity:** Must-Fix.  
**Original concern:** Arbitrary bounded context strings could be selected by downstream callers.

**Current evidence:** `HkdfKeyPurpose` exposes six canonical labels: WorkspaceKey, ProjectKey, FileKey, MetadataKey, AuditKey and RecoveryKey. `HkdfContext::parse` accepts only the exact registered strings. The public derivation entry point uses the typed enum; raw `derive_subkey` is private.

**Current disposition:** **Mitigated for the current API surface**, subject to compilation and tests on the integrated candidate. Adding a new purpose is an API/security change requiring review. Purpose separation is not process authorization, access control or proof that the IKM is uncompromised.

### SEC-KDF-05 — Memory lifetime and zeroization limitations

**Original classification:** Informational / architecture note.

**Current evidence:** Derived output buffers are `Zeroizing<[u8; 32]>`. Password and IKM inputs are borrowed slices; the callee cannot scrub their source buffers. `Zeroizing` does not guarantee removal of compiler copies, registers, swap/pagefile, crash dumps, hibernation images or OS-managed copies.

**Current disposition:** **Partially mitigated by owned-output scrubbing; residual limitations remain.** The threat model documents these non-guarantees. Memory locking, process isolation and platform-specific secret handling are not implemented by this KDF crate.

### KAT helper visibility and test integrity

The hardened API candidate scopes RFC 9106 / RFC 5869 helpers behind `cfg(test)` and keeps them out of the production re-export surface. During integration review, a concrete source omission was found: `mod.rs` declared `#[cfg(test)] mod kat_tests;` but `src/kdf/kat_tests.rs` was absent from the branch tree. That would prevent the KDF unit-test module from compiling. The missing module was restored from the original RFC-vector test source, adapted to use internal helpers; a separate `tests/kdf_tests.rs` integration suite was also restored using only public typed KDF APIs. Those new files have **not** yet been compiled or executed.

## 4. Dependency and supply-chain status

The checked candidate `Cargo.lock` pins these direct cryptographic dependencies:

| Crate | Locked version |
|---|---:|
| `argon2` | `0.5.3` |
| `hkdf` | `0.12.4` |
| `sha2` | `0.10.9` |
| `zeroize` | `1.9.2` |

The lockfile also contains transitive packages. A dependency declaration or lockfile pin is not a security audit. The RustSec Advisory Database is the authoritative public advisory source used by `cargo-audit`; its project documentation states that `cargo-audit` consumes RustSec advisories to audit `Cargo.lock` files.

A live web lookup of the aggregate RustSec advisory index was performed on 2026-11-20, but that does **not** establish a clean version-specific result for this dependency graph. No `cargo audit`, `cargo deny`, SBOM generation, license scan, feature-tree review or machine-readable advisory report was executed on this candidate. Therefore this report makes **no “no known vulnerabilities” or supply-chain-clean claim**.

**Required follow-up:** Run `cargo audit` against the exact candidate lockfile, archive the tool version and complete output, triage all direct and transitive advisories, then add a repeatable CI gate. Review enabled features and the transitive graph, not only the four direct crates.

## 5. CI and test evidence boundaries

For PR #3's old head `d705efd...`, GitHub Actions run `44580168927` completed successfully. Its five jobs were:
- Validate commit messages;
- Minimum supported Rust version (1.85);
- Rust checks on macOS;
- Rust checks on Windows;
- Rust checks on Ubuntu.

These successes apply **only** to that exact old PR #3 head. They do not transfer to `5520413...` or the stacked PR #26 branch.

PR #26 is Draft and targets the Crypto boundary branch, not `main`. On its latest exact head `076c84082a6259b9c1c724327bdc32ba614581b8`, [ARK CI #46](https://github.com/RhinoGuard/ARK/actions/runs/44624894201) passed all five jobs (Ubuntu, Windows, macOS, MSRV Rust 1.85 and commit-message validation); formatting, workspace build, Clippy and tests passed. The dedicated [KDF fuzz smoke #10](https://github.com/RhinoGuard/ARK/actions/runs/44624894126) also passed both bounded API-connected targets and artifact upload on that exact head. No local `cargo fmt`, `cargo build`, `cargo clippy`, `cargo test`, `cargo audit`, benchmark or fuzz command was executed. CI/fuzz evidence is GitHub Actions evidence only.

## 6. Remaining risk register

| Priority | Remaining item | Owner / boundary | Exit evidence |
|---|---|---|---|
| P1 | Reconcile the staged candidate into Crypto owner's final PR and obtain final review on that ref | Crypto + Security Validation | Candidate checks pass in CI #46; PR #3 remains old and has not inherited those checks |
| P1 | Fresh independent A-06 review and Crypto owner response | Ali / Saleh | Ali reviews current diff; Saleh responds to each finding |
| P1 | Resolve IKM source/provenance contract | Crypto + Systems (E-03) | Explicit owner decision and call-site contract |
| P1 | Extend bounded KDF fuzz smoke into sustained owner-reviewed campaign and triage artifacts | Ali / Security Validation | Smoke #10 recorded; longer duration, broader corpus/coverage and owner crash triage remain outstanding |
| P2 | Run `cargo audit` and establish repeatable dependency advisory CI | Crypto + Security/CI | Archived report and automated gate |
| P2 | Calibrate Argon2id settings on target platforms | Crypto + Platform | Reproducible benchmark data and approved latency/resource budget |
| P2 | Add service-level admission control when exposed to untrusted concurrent requests | Runtime/Service owner | Bounded queue/concurrency tests and rate-limit policy |

## 7. Final disposition

This static cross-check shows that the specific structural, Argon2 per-call bound, and HKDF context issues from Ali's original report have been addressed in a staged candidate; CI #46 and bounded fuzz smoke #10 pass on the latest candidate head. IKM provenance, global resource controls, dependency audit, parameter calibration and owner reviews remain residual concerns. The missing KAT/integration test source has been restored and executed in CI.

**Not ready for final security sign-off.** The candidate still needs an exact PR #3 synchronization/owner review, dependency audit evidence, sustained fuzzing, parameter calibration, and Ali's fresh independent review with Saleh's response. This document is authored as a Systems Engineering reconciliation; it must not be attributed to Ali or used as a substitute for his A-06 approval.


## 8. Final CI evidence for the integrated candidate

The earlier CI observations in Section 5 are historical snapshots. The subsequent correction was validated by GitHub Actions run **#32**:

- Run: [ARK CI #32](https://github.com/RhinoGuard/ARK/actions/runs/44621678450)
- Exact tested head: `27bc2e61a5e136aeb1febf147526fc2bc82b7d7c`
- PR: [#26 — stacked draft validation](https://github.com/RhinoGuard/ARK/pull/26)
- Base branch: `fix/crypto-kdf-production-api-boundary` at `552041395ee9097902eb6a7d1c934ed55b57a99f`

All five workflow jobs completed successfully:

1. Rust checks — Ubuntu: format, workspace build, Clippy and tests passed.
2. Rust checks — Windows: format, workspace build, Clippy and tests passed.
3. Rust checks — macOS: format, workspace build, Clippy and tests passed.
4. Minimum supported Rust version — Rust 1.85: format, build, Clippy and tests passed.
5. Conventional Commit message validation passed.

The passing Rust test output on Ubuntu, macOS and Rust 1.85 included 10 Core tests, 6 KDF unit/KAT tests, 8 KDF integration tests and 8 adversarial negative tests (all passed; no failures). Windows also completed the same test step successfully.

This closes the test-execution gap for the current Rust KDF/validation source at the stated SHA. It does **not** validate the standalone fuzz package, establish IKM entropy/provenance, supply CSPRNG ownership, create a cross-platform performance calibration, provide global concurrent Argon2 admission control, or constitute Ali/Saleh owner approval. No `cargo audit`, sustained fuzz run, benchmark, or platform resource-pressure campaign is included in this CI workflow.


## 9. Bounded fuzz smoke evidence

Although the normal workspace CI does not include the standalone fuzz package, a dedicated [KDF fuzz smoke run #7](https://github.com/RhinoGuard/ARK/actions/runs/44624226096) built and ran both actual public-API targets on head `be376f62ee454ca359f79c278f69fe3d882e3231`; the initial run #4 also passed. The environment used `rustc 1.102.0-nightly (65ce8611f 2026-11-18)`, `cargo 1.102.0-nightly (f93c676e3 2026-11-15)` and `cargo-fuzz 0.13.2`.

- Argon2id: 754 executions / 21 seconds; libFuzzer coverage 338, feature count 346, final in-run corpus 7 entries / 103 bytes; peak RSS 330 MiB.
- HKDF: 4,633,173 executions / 21 seconds; coverage 101, feature count 115, final in-run corpus 25 entries / 2,156 bytes; peak RSS 464 MiB.
- Both targets exited successfully, no crash was reported, and the corpus/artifact upload succeeded. Artifact: [KDF fuzz smoke #7](https://github.com/RhinoGuard/ARK/actions/runs/44624226096/artifacts/14577108336); SHA-256 `de2ba58bd23b5b8d5cf7707d96b4a87b7ddf10a064149a478de0d8b3ed62f78a`.

These results are a single bounded smoke sample per target, not a sustained campaign, cross-platform fuzz run, or resource-pressure test. They do not change the residual IKM provenance, service admission-control, dependency-audit, or platform-calibration findings.


## 10. Latest exact-head validation — 2026-11-20

The previous Sections 8–9 retain earlier passing runs as history. The latest verification is:

- PR #26 head: `076c84082a6259b9c1c724327bdc32ba614581b8`.
- [ARK CI #46](https://github.com/RhinoGuard/ARK/actions/runs/44624894201): all five jobs passed, including workspace build, Clippy, tests on Ubuntu/Windows/macOS and MSRV Rust 1.85, plus commit validation.
- [KDF fuzz smoke #10](https://github.com/RhinoGuard/ARK/actions/runs/44624894126): both actual API-connected targets were formatted, compiled and run successfully; artifact upload succeeded; no crash was reported.
- Toolchain: rustc/cargo `1.102.0-nightly`; cargo-fuzz `0.13.2`.
- Argon2id: 428 runs / 21 seconds, coverage 338, features 348, peak RSS 330 MiB.
- HKDF: 2,718,772 runs / 21 seconds, coverage 101, features 115, peak RSS 466 MiB.
- Artifact ID `14579133051`; SHA-256 `2d80c6fa8c221d41b1705dffd5bcbc032a604d7127fc8ef0e3d80ac910dff210`.

This evidence validates only the exercised candidate paths and a short hosted-Ubuntu smoke. It does not establish dependency cleanliness, long-duration robustness, platform calibration, IKM provenance, service-level rate controls, or owner approval.
