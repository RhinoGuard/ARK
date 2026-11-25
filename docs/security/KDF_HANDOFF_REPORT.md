# KDF Contract-Gate Handoff Report

**Repository:** `RhinoGuard/ARK`  
**S-07:** [Issue #37 — Finalize Crypto/KDF candidate and close remaining evidence gates](https://github.com/RhinoGuard/ARK/issues/37)  
**Delivery PR:** [#35 — security(crypto): close remaining KDF contract gates](https://github.com/RhinoGuard/ARK/pull/35)  
**Working branch:** `feat/crypto-kdf-contract-gates`  
**Integration base:** `fix/crypto-kdf-production-api-boundary` (PR #28 head `3ab35c3acf00d0dbd4df7e14ec0fc199e904a2a9`)  
**Current delivery status:** PR #35 is open and Ready for Review; not merged.  
**Target branch:** PR #28 branch, not `main`.

## Executive summary

The S-07 evidence refresh confirms that PR #35 is the focused KDF follow-up stacked on the selected hardened candidate PR #28. The source-code head validated by CI, `6b52fed41452782cc67a22a950d02411e815cb42`, was 17 commits ahead of, and 0 commits behind, PR #28 head `3ab35c3acf00d0dbd4df7e14ec0fc199e904a2a9`.

GitHub Actions run [#45400557291](https://github.com/RhinoGuard/ARK/actions/runs/45400557291) completed successfully on the exact PR #35 head. All six jobs passed: commit-message validation, locked dependency audit, Windows checks, Rust 1.85 MSRV checks, macOS checks, and Ubuntu checks. The workflow steps for formatting, workspace build, Clippy, and tests passed on all three operating systems.

This CI result closes the exact-head CI evidence gap for source head `6b52fed`. Subsequent S-07 commits updated only Markdown handoff/disposition and PR metadata; no CI result is currently visible for the newer documentation-only head, so exact-current-head CI remains pending. It does not constitute a benchmark run, independent security audit, Crypto-owner acceptance of the six HKDF purpose semantics, or Foundation exit approval.

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
   - PR #26 remains 67 commits ahead of PR #28 with 0 behind; PR #27 remains 7 commits ahead of #26 with 0 behind. The #28-to-#27 comparison is 74 ahead / 0 behind.
   - PR #3 and PR #28 remain diverged from merge base `bafa677d6b42e7b04b736fcc386cd8e4ea4d9943`; comparing PR #3 head `d705efd042ca31113eab3b2bdf63fb643fd27f8b` to PR #28 head `3ab35c3acf00d0dbd4df7e14ec0fc199e904a2a9` reports 16 ahead / 4 behind. PR #3 is not proposed as a separate merge path.

## CCB status

| Gate | Status |
|---|---|
| CCB-01/06 — IKM trust and provenance | Contract and regression coverage delivered; upstream provenance enforcement remains with the future credential/key-lifecycle owner. |
| CCB-02 — HKDF purposes | Six exact labels and regression tests delivered; explicit Crypto-owner confirmation and independent review remain. |
| CCB-03 — CSPRNG/salt ownership | Ownership boundary documented and confirmed by the Crypto owner in the Issue #17 E-03 comment; actual OS RNG integration is deferred to Issue #31. |
| CCB-04 — Argon2 calibration | Harness and workflow delivered; cross-platform artifacts, constrained-hardware evidence, and peak-RSS measurements remain outstanding under Issue #34. |
| CCB-05 — service-level errors | Explicitly deferred to Issue #32 because the service boundary does not exist. |
| CCB-07 — persisted metadata/migration | Explicitly deferred to Issue #33 because the storage/credential-persistence boundary does not exist. |

## Current GitHub state

- PR #35: open, Ready for Review, base `fix/crypto-kdf-production-api-boundary`; consult live PR metadata for the current documentation-only head. The successful CI run below is attached to source head `6b52fed41452782cc67a22a950d02411e815cb42`.
- PR #28: open, Draft, selected candidate; head `3ab35c3acf00d0dbd4df7e14ec0fc199e904a2a9`.
- PR #26 and PR #27: open, Draft; their staged ancestry remains unchanged.
- Issue #29: closed as completed.
- S-07 Issue #37: remains open pending outstanding acceptance evidence and decisions.
- No PR was merged, no Issue was closed as part of S-07, and no branch history was rewritten.

## Validation evidence

- Source-head CI: [Run #45400557291](https://github.com/RhinoGuard/ARK/actions/runs/45400557291) on `6b52fed41452782cc67a22a950d02411e815cb42` — completed, success, 6/6 jobs.
- The later documentation-only branch head has no visible CI run/status yet; exact-current-head CI remains pending.
- Windows: formatting, build, Clippy, and tests passed.
- macOS: formatting, build, Clippy, and tests passed.
- Ubuntu: formatting, build, Clippy, and tests passed.
- MSRV: Rust 1.85.0 formatting, build, Clippy, and tests passed.
- Dependency audit and commit-message validation passed.
- No local Rust toolchain commands were executed in this editing session.
- No new calibration workflow run or artifact is available.
- No independent Security Validation approval is recorded for PR #35.

## Follow-up and remaining acceptance gates

- [#31 — OS-CSPRNG salt generation at credential lifecycle](https://github.com/RhinoGuard/ARK/issues/31)
- [#32 — Sanitized service-level KDF error mapping](https://github.com/RhinoGuard/ARK/issues/32)
- [#33 — Authenticated KDF metadata and migration](https://github.com/RhinoGuard/ARK/issues/33)
- [#34 — Supported-platform Argon2id calibration evidence](https://github.com/RhinoGuard/ARK/issues/34)
- Obtain explicit Crypto-owner confirmation of the six HKDF purpose semantics.
- Run the calibration workflow and retain raw artifacts plus exact environment metadata.
- Add process peak-memory measurements and representative constrained-hardware/concurrency evidence where available.
- Obtain independent Security Validation review under ARK-SEC-032.

## Review recommendation

PR #35 is ready for technical review based on its exact-head CI result, but S-07 is not complete. Do not claim benchmark completion, six-purpose owner acceptance, independent security approval, or Foundation exit until the corresponding evidence and decisions are recorded. Keep PR #28/#26/#27 states and all existing history unchanged.
