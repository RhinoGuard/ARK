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
- The original KDF fuzz targets in `feat/kdf-fuzzing-harness` explicitly simulate checks and do not call `ark-crypto`.
- Issues #9–#14 are closed in GitHub. Closed state is recorded as workflow status only; it does not override contradictory source evidence or establish that the present API has been validated.

## 2. A-01 through A-06 reconciliation

| Task | GitHub issue | Current evidence-based disposition |
|---|---:|---|
| A-01 — Core API test reconciliation | #9 | **Closed historically; current acceptance not revalidated here.** The old branch is diverged and contains duplicated Foundation history. Port only Ali-owned test changes and verify against current Core API if further changes are required. |
| A-02 — KDF threat model | #10 | **Implemented — needs synchronization.** Existing threat model is useful but contains stale descriptions of raw `derive_subkey`, unbounded Argon2 configuration, and open-ended context. An API reconciliation addendum has been added in this branch; owner review remains outstanding. |
| A-03 — invariant traceability | #11 | **Not revalidated.** The branch name exists, but the expected traceability file was not located at the initially guessed path. Do not infer acceptance from issue closure. Locate the exact artifact and verify every mapping against current code/tests before claiming current validity. |
| A-04 — adversarial negative tests | #12 | **Not revalidated against current KDF API.** Historical tests were on a diverged branch and used private/raw KDF helpers. They must be ported to the public typed API and rerun before being treated as current evidence. |
| A-05 — KDF fuzzing | #13 | **Original implementation not valid API fuzzing.** Existing KDF targets only simulate validation and depend on `ark-core`, not `ark-crypto`. A replacement harness now calls the real public Argon2id and HKDF APIs in this dependent branch. **Implemented — not executed / not validated.** |
| A-06 — independent Crypto review | #14 | **Preliminary static review exists but is stale.** The report predates the current PR #3 head and KDF API hardening. Its historical findings must be reconciled; it must not be represented as a fresh independent review of the current head. |

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
- No PR was opened, no review was submitted, no issue was changed, and no branch was merged as part of this reconciliation.

## 5. Remaining work

1. Build the fuzz package with the pinned/nightly toolchain and resolve any compile or lint failures.
2. Run bounded fuzz sessions for both targets; record exact commit, toolchain, cargo-fuzz version, commands, duration, corpus, and crash triage.
3. Port and validate A-04 negative tests against the final public API.
4. Locate and reconcile the actual A-03 traceability artifact.
5. Refresh the A-06 review against the final PR #3 head and the accepted E-03 contract; preserve the distinction between static review and executed tooling.
6. Obtain Ali's independent owner review and Crypto owner's response before acceptance.

## 6. Limitations

This report is based on GitHub source and metadata inspection. No local Cargo build, test, Clippy, benchmark, or fuzz command was run. It is not a cryptographic certification, and the new fuzz harness is not yet execution evidence.
