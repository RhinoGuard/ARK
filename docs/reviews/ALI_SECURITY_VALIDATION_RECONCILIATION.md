# Ali Security Validation Reconciliation

**Prepared by:** Systems Engineering integration review  
**Date:** 2026-11-20  
**Purpose:** Reconcile the Security Validation artifacts and closed A-01–A-06 issues with the current KDF API and GitHub evidence. This is not an owner approval and does not replace Ali's independent A-06 review.

## 1. Evidence reviewed

- `feat/security-testing-framework` compared with `main`: 13 commits ahead, 3 behind, diverged; includes historical Foundation files and must not be merged wholesale.
- `feat/kdf-fuzzing-harness` compared with `main`: 14 commits ahead, 3 behind, diverged.
- `docs/kdf-threat-model`: threat model contains pre-hardening descriptions of the KDF API.
- `docs/crypto-security-review`: A-06 report dated 2026-11-17, scoped to earlier PR #3 source commits `c7322e2..3fdabeb`.
- Current PR #3 remains open at `d705efd042ca31113eab3b2bdf63fb643fd27f8b`; the existing formal review remains `CHANGES_REQUESTED`.
- Current KDF boundary branch: `fix/crypto-kdf-production-api-boundary`, head `552041395ee9097902eb6a7d1c934ed55b57a99f`.
- The original KDF fuzz targets in `feat/kdf-fuzzing-harness` simulate checks and do not call `ark-crypto`; the replacement targets in this integration branch call the actual API.
- Issues #9–#14 are closed in GitHub. Closed state is recorded as workflow status only; it does not override contradictory source evidence or establish that the present API has been validated.

## 2. A-01 through A-06 reconciliation

| Task | GitHub issue | Current evidence-based disposition |
|---|---:|---|
| A-01 — Core API test reconciliation | #9 | **Closed historically; current acceptance not revalidated here.** The old branch is diverged and contains duplicated Foundation history. Port only Ali-owned test changes and verify against current Core API if further changes are required. |
| A-02 — KDF threat model | #10 | **Implemented — needs synchronization.** Existing threat model is useful but contains stale descriptions of raw `derive_subkey`, unbounded Argon2 configuration, and open-ended context. An API reconciliation addendum has been added in this branch; owner review remains outstanding. |
| A-03 — invariant traceability | #11 | **Artifact located and reconciled; owner validation pending.** Source matrix is `docs/security/TRACEABILITY_MATRIX.md` on `docs/security-traceability-matrix`. `docs/reviews/A03_TRACEABILITY_MATRIX_RECONCILIATION.md` corrects the earlier guessed-path error and reconciles stale KDF rows against PR #26 head `076c840...`; relevant KDF tests are CI-validated in #46. Ali's review and any update to the source matrix remain outstanding. |
| A-04 — adversarial negative tests | #12 | **Executed successfully on the current candidate.** The 8 negative tests cover password/salt/config bounds (including direct public-field config construction), empty/short IKM, unregistered purposes, purpose-label uniqueness and error-format redaction. All 8 pass in the exact-head CI #46 on Ubuntu, Windows, macOS and Rust 1.85. This validates only the exercised inputs and does not prove IKM provenance or universal diagnostic safety. |
| A-05 — KDF fuzzing | #13 | **API-connected replacement executed with bounded smoke evidence; sustained campaign pending.** The two targets call actual public KDF APIs. KDF fuzz smoke #10 passed on the latest PR #26 head `076c840...`; no crash was reported and an artifact was uploaded. It is a 20-second-per-target hosted-Ubuntu smoke, not a sustained or cross-platform campaign. |
| A-06 — independent Crypto review | #14 | **Historical Ali report reconciled; fresh owner review still required.** `docs/reviews/A06_CURRENT_KDF_REVIEW.md` is a Systems Engineering cross-check of the hardened API, exact candidate revisions, CI evidence and residual risks. It is not Ali's independent approval. Saleh's response and Ali's review on the final Crypto head remain outstanding. |

## 3. A-05 replacement harness

The branch `feat/security-validation-kdf-fuzz-integration` is stacked on `fix/crypto-kdf-production-api-boundary`, not on `main`; its latest head is `076c84082a6259b9c1c724327bdc32ba614581b8`. This is deliberate: the current `main` does not yet contain `ark-crypto`, so a fuzz package depending on it cannot be built from main alone.

The branch adds:
- `fuzz/Cargo.toml` with a standalone fuzz workspace and a path dependency on `ark-crypto`;
- `fuzz_argon2id`, which calls `derive_kek` with bounded input and a fixed minimum-cost valid profile;
- `fuzz_hkdf`, which calls `HkdfContext::parse`, `HkdfIkm::from_slice`, and `derive_purpose_subkey`;
- `fuzz/README.md` documenting commands, resource bounds, and evidence requirements.

This is a real API-connected harness. Three bounded executions are now recorded; the latest is KDF fuzz smoke #10 on the exact current branch head. See Section 9. This is limited smoke evidence, not a sustained A-05 campaign.

## 4. Integration and ownership

- Ali remains the Security Validation owner.
- Saleh remains the Crypto implementation owner.
- The replacement fuzz branch is dependent on the pending KDF boundary branch. It must not be merged independently into `main` before the Crypto dependency is accepted.
- Do not merge the old `feat/security-testing-framework` or `feat/kdf-fuzzing-harness` wholesale.
- Two new review artifacts were added: `docs/reviews/A03_TRACEABILITY_MATRIX_RECONCILIATION.md` and `docs/reviews/A06_CURRENT_KDF_REVIEW.md`. The KAT and KDF integration test source omissions were restored. Draft PR #26 was opened against `fix/crypto-kdf-production-api-boundary` to validate the stacked candidate; it is not targeted at `main`. No review was submitted, no issue state was changed, and no branch was merged.

## 5. Test-source recovery and validation boundary

The hardened branch declared `#[cfg(test)] mod kat_tests;` but the module source was absent. The integration branch now contains `crates/ark-crypto/src/kdf/kat_tests.rs`, converted from the original RFC KAT test source so the test-only helper functions remain internal, plus `crates/ark-crypto/tests/kdf_tests.rs` rewritten to use only the public typed purpose API. These source files compiled and executed successfully in CI #46 on the latest PR #26 head. The test suites compare registered purpose labels where label uniqueness is the property under test; they do not claim that cryptographic hash outputs can never collide.

## 6. Remaining work

1. Run a longer owner-reviewed fuzz campaign on both targets, preserve corpus/crash triage, and consider cross-platform or resource-pressure scenarios beyond the 20-second smoke.
2. Resolve the IKM provenance issue (CCB-01/06) with an explicit Crypto/Systems decision; the current wrapper does not prevent arbitrary long password bytes from being wrapped.
3. Obtain Ali's review of the located A-03 matrix and its current-API reconciliation.
4. Refresh A-06 on the final Crypto PR head and record Saleh's response to each finding.
5. Run `cargo audit` against the exact final lockfile and review transitive dependencies.
6. Obtain Ali's independent owner review and Crypto owner sign-off before security acceptance.

## 7. Limitations

This report is based on GitHub source, metadata, and exact-head GitHub Actions evidence. No local Cargo command was run. The normal Rust workspace and the bounded 20-second fuzz smoke were executed by GitHub Actions; no sustained fuzz campaign, dependency audit, benchmark, or cryptographic certification is claimed.


## 8. Current execution evidence

GitHub Actions [run #32](https://github.com/RhinoGuard/ARK/actions/runs/44621678450) passed on exact candidate head `27bc2e61a5e136aeb1febf147526fc2bc82b7d7c`. Ubuntu, Windows, macOS and MSRV Rust 1.85 completed format, workspace build, Clippy and tests successfully; commit-message validation also passed. The logged test suites included 10 Core tests, 6 KDF unit/KAT tests, 8 KDF integration tests and 8 adversarial negative tests, all passing.

A-04 is execution-validated for the listed cases on PR #26 head `076c840...` in CI #46. A-05 has three successful bounded smoke runs, with the latest run #10 on the same head; a sustained owner-reviewed campaign remains open. This passing CI does not replace Ali's independent owner review or Saleh's Crypto response.


## 9. A-05 bounded fuzz smoke evidence

The replacement harness was built and executed on the exact current PR #26 head in GitHub Actions [KDF fuzz smoke #10](https://github.com/RhinoGuard/ARK/actions/runs/44624894126):

- Head: `076c84082a6259b9c1c724327bdc32ba614581b8`
- Toolchain: `rustc 1.102.0-nightly (65ce8611f 2026-11-18)`; `cargo 1.102.0-nightly (f93c676e3 2026-11-15)`; `cargo-fuzz 0.13.2`.
- Both target formatting and compilation passed before execution.
- Both targets completed with successful exits; no crash was reported; artifact upload succeeded.

| Target | Runs / duration | libFuzzer counters | Peak RSS |
|---|---:|---:|---:|
| Argon2id | 428 / 21 seconds | coverage 338, features 348, corpus 9 / 137 bytes | 330 MiB |
| HKDF | 2,718,772 / 21 seconds | coverage 101, features 115, corpus 25 / 1,919 bytes | 466 MiB |

Commands used 20-second maximum run budgets, 1 GiB RSS limits, 10-second per-input timeouts, and max input lengths of 1,040 bytes (Argon2id) and 1,024 bytes (HKDF). The uploaded artifact is [`kdf-fuzz-smoke-d8fe9afbb3f9d2299cafde0cbb390f5c0b4f4188`](https://github.com/RhinoGuard/ARK/actions/runs/44624894126/artifacts/14579133051), SHA-256 `2d80c6fa8c221d41b1705dffd5bcbc032a604d7127fc8ef0e3d80ac910dff210`.

This is a bounded smoke execution, not a sustained campaign, cross-platform fuzzing, global resource-pressure test, or assurance that no latent crash exists.
