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
| A-03 — invariant traceability | #11 | **Not revalidated.** The branch name exists, but the expected traceability file was not located at the initially guessed path. Do not infer acceptance from issue closure. Locate the exact artifact and verify every mapping against current code/tests before claiming current validity. |
| A-04 — adversarial negative tests | #12 | **Executed successfully on the current candidate.** The 8 negative tests cover password/salt/config bounds (including direct public-field config construction), empty/short IKM, unregistered purposes, purpose-label uniqueness and error-format redaction. All 8 pass in CI #32 on Ubuntu, Windows, macOS and Rust 1.85. This validates only the exercised inputs and does not prove IKM provenance or universal diagnostic safety. |
| A-05 — KDF fuzzing | #13 | **API-connected replacement implemented; actual fuzz execution still pending.** The two targets call the actual public KDF APIs with bounded input. CI #32 does not build or execute the separate `fuzz/` workspace. A dedicated cargo-fuzz smoke run and recorded corpus/crash evidence are still required. |
| A-06 — independent Crypto review | #14 | **Historical Ali report reconciled; fresh owner review still required.** `docs/reviews/A06_CURRENT_KDF_REVIEW.md` is a Systems Engineering cross-check of the hardened API, exact candidate revisions, CI evidence and residual risks. It is not Ali's independent approval. Saleh's response and Ali's review on the final Crypto head remain outstanding. |

## 3. A-05 replacement harness

A new branch, `feat/security-validation-kdf-fuzz-integration`, was created from `fix/crypto-kdf-production-api-boundary`, not from `main`. This is deliberate: the current `main` does not yet contain `ark-crypto`, so a fuzz package depending on it cannot be built from main alone.

The branch adds:
- `fuzz/Cargo.toml` with a standalone fuzz workspace and a path dependency on `ark-crypto`;
- `fuzz_argon2id`, which calls `derive_kek` with bounded input and a fixed minimum-cost valid profile;
- `fuzz_hkdf`, which calls `HkdfContext::parse`, `HkdfIkm::from_slice`, and `derive_purpose_subkey`;
- `fuzz/README.md` documenting commands, resource bounds, and evidence requirements.

This is a real API-connected harness, but no fuzz execution has been performed by this change. Do not mark A-05 validated until the target builds and runs for a recorded duration with a recorded corpus and triaged result.

## 4. Integration and ownership

- Ali remains the Security Validation owner.
- Saleh remains the Crypto implementation owner.
- The replacement fuzz branch is dependent on the pending KDF boundary branch. It must not be merged independently into `main` before the Crypto dependency is accepted.
- Do not merge the old `feat/security-testing-framework` or `feat/kdf-fuzzing-harness` wholesale.
- Two new review artifacts were added: `docs/reviews/A03_TRACEABILITY_MATRIX_RECONCILIATION.md` and `docs/reviews/A06_CURRENT_KDF_REVIEW.md`. The KAT and KDF integration test source omissions were restored. Draft PR #26 was opened against `fix/crypto-kdf-production-api-boundary` to validate the stacked candidate; it is not targeted at `main`. No review was submitted, no issue state was changed, and no branch was merged.

## 5. Test-source recovery and validation boundary

The hardened branch declared `#[cfg(test)] mod kat_tests;` but the module source was absent. The integration branch now contains `crates/ark-crypto/src/kdf/kat_tests.rs`, converted from the original RFC KAT test source so the test-only helper functions remain internal, plus `crates/ark-crypto/tests/kdf_tests.rs` rewritten to use only the public typed purpose API. These source files have not been compiled or run. The test suites compare registered purpose labels where label uniqueness is the property under test; they do not claim that cryptographic hash outputs can never collide.

## 6. Remaining work

1. Build the fuzz package with the pinned/nightly toolchain and resolve any compile or lint failures.
2. Run bounded fuzz sessions for both targets; record exact commit, toolchain, cargo-fuzz version, commands, duration, corpus, and crash triage.
3. Validate the restored KAT and integration test modules plus A-04 negative tests against the final public API.
4. Obtain Ali's review of the located A-03 matrix and its current-API reconciliation.
5. Refresh A-06 on the final Crypto PR head and the final E-03 decisions; preserve the distinction between static review and executed tooling.
6. Obtain Ali's independent owner review and Saleh's response before security acceptance.

## 7. Limitations

This report is based on GitHub source and metadata inspection. No local Cargo build, test, Clippy, benchmark, or fuzz command was run. It is not a cryptographic certification, and the new fuzz harness is not yet execution evidence.


## 8. Current execution evidence

GitHub Actions [run #32](https://github.com/RhinoGuard/ARK/actions/runs/44621678450) passed on exact candidate head `27bc2e61a5e136aeb1febf147526fc2bc82b7d7c`. Ubuntu, Windows, macOS and MSRV Rust 1.85 completed format, workspace build, Clippy and tests successfully; commit-message validation also passed. The logged test suites included 10 Core tests, 6 KDF unit/KAT tests, 8 KDF integration tests and 8 adversarial negative tests, all passing.

A-04 is therefore execution-validated for those cases on the stated candidate. A-05 remains **not execution-validated** because the ordinary Rust CI workflow does not enter the standalone fuzz workspace. The replacement fuzz targets must still be run separately. This passing CI does not replace Ali's independent owner review or Saleh's Crypto response.
