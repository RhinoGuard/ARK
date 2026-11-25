# KDF Contract-Gate Handoff Report

**Repository:** `RhinoGuard/ARK`  
**S-07:** [Issue #37 — Finalize Crypto/KDF candidate and close remaining evidence gates](https://github.com/RhinoGuard/ARK/issues/37)  
**Delivery PR:** [#35 — security(crypto): close remaining KDF contract gates](https://github.com/RhinoGuard/ARK/pull/35)  
**Working branch:** `feat/crypto-kdf-contract-gates`  
**Integration base:** `fix/crypto-kdf-production-api-boundary` (PR #28 head `3ab35c3acf00d0dbd4df7e14ec0fc199e904a2a9`)  
**Current delivery status:** PR #35 is open and Ready for Review; not merged.  
**Current PR #35 head:** `87a6c438df5150715d36037b2453a53912f695c6`  
**Target branch:** PR #28 branch, not `main`.

## Executive summary

PR #35 is the focused KDF follow-up stacked on the selected hardened candidate PR #28. The source-code head `6b52fed41452782cc67a22a950d02411e815cb42` was 17 commits ahead of, and 0 commits behind, PR #28 head `3ab35c3acf00d0dbd4df7e14ec0fc199e904a2a9`. The later S-07 commits changed only Markdown handoff/disposition records and PR metadata.

GitHub Actions run [#45645080815](https://github.com/RhinoGuard/ARK/actions/runs/45645080815) completed successfully on the exact current PR #35 head `87a6c438df5150715d36037b2453a53912f695c6`. All six jobs passed: commit-message validation, locked dependency audit, Windows checks, Rust 1.85 MSRV checks, macOS checks, and Ubuntu checks. Formatting, workspace build, Clippy, and tests passed on Windows, macOS, and Ubuntu; MSRV formatting/build/Clippy/tests also passed.

This closes the exact-current-head CI evidence gate. It does not constitute a benchmark run, independent security audit, Crypto-owner acceptance of the six HKDF purpose semantics, or Foundation exit approval.

No changes were pushed to `main`. PR #28, PR #26, PR #27, and PR #3 were not rewritten or merged.

## Delivered changes

1. **Argon2id salt ownership**
   - Clarified that `ark-crypto` validates caller-supplied salts and derives a KEK; it does not generate random bytes.
   - Assigned creation/re-key salt generation to the future credential-lifecycle owner using an approved OS CSPRNG.
   - Required the exact persisted salt to be reused during unlock.
   - Corrected the public `derive_kek` documentation.

2. **HKDF IKM provenance**
   - Documented that the 16-byte minimum is a length check, not proof of entropy or trusted origin.
   - Documented caller obligations and prohibited treating raw passwords or low-entropy identifiers as valid production IKM.
   - Added regression coverage demonstrating the distinction.

3. **Registered HKDF purposes**
   - Documented all six ARK v1 purpose labels and intended roles.
   - Added regression coverage locking the exact labels and checking distinct derivations.
   - Added secret-wrapper Debug redaction regression coverage.
   - **Still requires explicit Crypto-owner confirmation of the six purpose semantics.** The existing owner comment on Issue #17 confirms the Core-to-Crypto boundary and salt ownership split, but does not explicitly approve the six-purpose registry.

4. **Argon2id calibration**
   - Added a six-profile benchmark harness and a manual Windows/Linux/macOS workflow.
   - The workflow has not yet been dispatched for this candidate; no S-07 cross-platform benchmark artifacts are available.
   - The harness reports configured Argon2 memory cost, not total process peak RSS. The current workflow uses two warm-ups and seven timed samples; the resulting p95 estimate is therefore coarse and should not be treated as a stable tail-latency characterization.
   - Representative constrained hardware, concurrency pressure, and process peak-memory measurements remain outstanding. No new tuning recommendation or universal-suitability claim is made.

5. **Deferred boundaries**
   - Service-level sanitized error mapping remains deferred to Issue #32 because the service/auth/session boundary does not exist in this KDF-only scope.
   - Authenticated persisted KDF metadata and migration remain deferred to Issue #33 because the storage/credential-persistence subsystem does not exist.

6. **PR reconciliation**
   - PR #28 remains the selected hardened Crypto integration candidate.
   - PR #26 and PR #27 retain their staged ancestry; no ownership or history changes were made.
   - PR #3 and PR #28 remain diverged from merge base `bafa677d6b42e7b04b736fcc386cd8e4ea4d9943`; comparing PR #3 head `d705efd042ca31113eab3b2bdf63fb643fd27f8b` to PR #28 head `3ab35c3acf00d0dbd4df7e14ec0fc199e904a2a9` reports 16 ahead / 4 behind. PR #3 is not proposed as a separate merge path.

## CCB status

| Gate | Status |
|---|---|
| CCB-01/06 — IKM trust and provenance | Contract and regression coverage delivered; upstream provenance enforcement remains with the future credential/key-lifecycle owner. |
| CCB-02 — HKDF purposes | Six exact labels and regression tests delivered; explicit Crypto-owner confirmation and independent review remain. |
| CCB-03 — CSPRNG/salt ownership | Ownership boundary documented and confirmed by the Crypto owner in the Issue #17 E-03 comment; actual OS RNG integration is deferred to Issue #31. |
| CCB-04 — Argon2 calibration | Harness and workflow delivered; cross-platform artifacts, constrained-hardware evidence, and peak-RSS measurements remain outstanding under Issue #34. |
| CCB-05 — service-level errors | Explicitly deferred to Issue #32 because the service boundary does not exist. |
| CCB-07 — persisted metadata/migration | Explicitly deferred to Issue #33 because the persistence boundary does not exist. |

## Current GitHub state

- PR #35: open, Ready for Review, base `fix/crypto-kdf-production-api-boundary`, current head `87a6c438df5150715d36037b2453a53912f695c6`.
- PR #28: open, Draft, selected candidate; head `3ab35c3acf00d0dbd4df7e14ec0fc199e904a2a9`.
- PR #26 and PR #27: open, Draft; staged ancestry unchanged.
- Issue #29: closed as completed.
- S-07 Issue #37: remains open pending benchmark artifacts, owner confirmation, and independent review.
- No PR was merged, no Issue was closed as part of this refresh, and no branch history was rewritten.

## Validation evidence

- Exact-current-head CI: [Run #45645080815](https://github.com/RhinoGuard/ARK/actions/runs/45645080815) on `87a6c438df5150715d36037b2453a53912f695c6` — completed successfully, 6/6 jobs.
- Windows: formatting, workspace build, Clippy, and tests passed.
- macOS: formatting, workspace build, Clippy, and tests passed.
- Ubuntu: formatting, workspace build, Clippy, and tests passed.
- MSRV: Rust 1.85.0 formatting, build, Clippy, and tests passed.
- Locked dependency audit and commit-message validation passed.
- No local Rust toolchain commands were executed in this GitHub editing session.
- No new calibration workflow run or artifact is available.
- No independent Security Validation approval is recorded for PR #35.

## Follow-up and remaining acceptance gates

- [#31 — OS-CSPRNG salt generation at credential lifecycle](https://github.com/RhinoGuard/ARK/issues/31)
- [#32 — Sanitized service-level KDF error mapping](https://github.com/RhinoGuard/ARK/issues/32)
- [#33 — Authenticated KDF metadata and migration](https://github.com/RhinoGuard/ARK/issues/33)
- [#34 — Supported-platform Argon2id calibration evidence](https://github.com/RhinoGuard/ARK/issues/34)
- Obtain explicit Crypto-owner confirmation of the six HKDF purpose semantics.
- Dispatch the calibration workflow and retain raw artifacts plus exact environment metadata.
- Add process peak-memory measurements and representative constrained-hardware/concurrency evidence where available.
- Obtain independent Security Validation review under ARK-SEC-032.

## Review recommendation

PR #35 has passing CI on its exact current head and is ready for technical review. S-07 is not complete: benchmark artifacts, six-purpose owner acceptance, and independent security approval remain outstanding. Do not claim Foundation exit or merge readiness until the corresponding evidence and decisions are recorded. Keep PR #28/#26/#27 states and all existing history unchanged.
