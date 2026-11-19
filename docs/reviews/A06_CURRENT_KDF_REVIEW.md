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
- Current Security Validation integration branch: `feat/security-validation-kdf-fuzz-integration`, head `bae0315e6f8891df43e1d4986d8c1440b81de021`, stacked on the hardened Crypto candidate.

The integration branch adds the missing internal KAT module and restores the public-API KDF integration tests. It has not been built or tested here. No claim is made that its source has passed compilation.

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

**Remaining evidence:** Execute the negative suite and test the exact integrated candidate; calibrate latency/memory on supported targets before final product-policy approval.

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

These successes apply **only** to that exact old PR head. They do not transfer to `5520413...` or `bae0315...`.

Draft PR #26 was opened against the Crypto boundary branch (not `main`) to trigger validation of the stacked candidate. GitHub Actions run `44619564386` (`ARK CI`) was observed in `in_progress` for head `280d4abb38c53db14382e02e919fb9a020a649e4` at the time this report was updated. No conclusion is claimed until the run completes. No local `cargo fmt`, `cargo build`, `cargo clippy`, `cargo test`, `cargo audit`, benchmark or fuzz command was executed. The new tests and fuzz targets remain unvalidated until the exact-head CI and dedicated fuzz execution finish.

## 6. Remaining risk register

| Priority | Remaining item | Owner / boundary | Exit evidence |
|---|---|---|---|
| P1 | Build/test/Clippy exact integrated KDF + validation candidate; verify unit KAT and integration suites compile | Crypto + Security Validation | Successful CI for exact head; no inherited old-head checks |
| P1 | Fresh independent A-06 review and Crypto owner response | Ali / Saleh | Ali reviews current diff; Saleh responds to each finding |
| P1 | Resolve IKM source/provenance contract | Crypto + Systems (E-03) | Explicit owner decision and call-site contract |
| P1 | Run bounded KDF fuzz sessions and triage artifacts | Ali / Security Validation | Exact commit, toolchain, cargo-fuzz version, commands, duration, corpus, coverage and crash triage |
| P2 | Run `cargo audit` and establish repeatable dependency advisory CI | Crypto + Security/CI | Archived report and automated gate |
| P2 | Calibrate Argon2id settings on target platforms | Crypto + Platform | Reproducible benchmark data and approved latency/resource budget |
| P2 | Add service-level admission control when exposed to untrusted concurrent requests | Runtime/Service owner | Bounded queue/concurrency tests and rate-limit policy |

## 7. Final disposition

This static cross-check shows that the specific structural, Argon2 per-call bound, and HKDF context issues from Ali's original report have been addressed in a staged candidate, while IKM provenance and global resource controls remain residual concerns. The integration branch also needed restoration of missing KAT/integration test source.

**Not ready for final security sign-off.** The candidate needs executed CI/testing, dependency audit evidence, bounded fuzz runs, and Ali's fresh independent review with Saleh's response. This document is authored as a Systems Engineering reconciliation; it must not be attributed to Ali or used as a substitute for his A-06 approval.


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

Although the normal workspace CI does not include the standalone fuzz package, a dedicated [KDF fuzz smoke run #4](https://github.com/RhinoGuard/ARK/actions/runs/44623186546) built and ran both actual public-API targets on head `4ec4de8d23201d7f2806b08cdccfb979b19fc0fc`. The environment used `rustc 1.102.0-nightly (65ce8611f 2026-11-18)` and `cargo-fuzz 0.13.2`.

- Argon2id: 590 executions / 21 seconds; libFuzzer coverage 338, feature count 346, final in-run corpus 7 entries / 105 bytes; peak RSS 329 MiB.
- HKDF: 3,963,943 executions / 21 seconds; coverage 101, feature count 115, final in-run corpus 25 entries / 1,916 bytes; peak RSS 468 MiB.
- Both targets exited successfully, no crash was reported, and the corpus/artifact upload succeeded. Artifact SHA-256: `8b3c01cf26d8e85c18107d0046dec2469833f1afb33f71bfa165e6aa54d285c3`.

These results are a single bounded smoke sample per target, not a sustained campaign, cross-platform fuzz run, or resource-pressure test. They do not change the residual IKM provenance, service admission-control, dependency-audit, or platform-calibration findings.
