# A-03 Traceability Matrix Reconciliation — Current Foundation Baseline

**Prepared by:** Systems Engineering integration review  
**Date:** 2026-11-20  
**Status:** Evidence reconciliation; not Ali's owner approval and not a replacement for the A-03 source matrix  
**Source issue:** [#11 — A-03](https://github.com/RhinoGuard/ARK/issues/11)

## 1. Source artifact and baseline correction

The A-03 source artifact exists at:

- Branch: `docs/security-traceability-matrix`
- File: `docs/security/TRACEABILITY_MATRIX.md`
- Current branch head checked: `67448995b11528a4ec4ad88e10ed84130d466094`
- File blob checked: `d59f0a07d09e9c24945115a85e760d4772ee20e1`
- Ali's completion comment: [Issue #11 comment](https://github.com/RhinoGuard/ARK/issues/11#issuecomment-6953692175)

An earlier integration note said the file had not been located. That statement was incorrect: the path was guessed incorrectly. This report corrects that record. The matrix was delivered on its own branch; it is not part of `main` and no PR/merge is inferred from the issue comment or closed issue state.

The source matrix is a Foundation v1 working draft. Its rows must be refreshed against the current Crypto API, not silently treated as current because its original completion report exists.

## 2. Reconciliation targets and evidence

Two source baselines were checked:

- Core baseline: `main` at `68dad185bd5572f3abd557a61059415a932ca95d` (merged PR #2).
- Crypto API candidate: `fix/crypto-kdf-production-api-boundary` at `552041395ee9097902eb6a7d1c934ed55b57a99f`.
- Security Validation integration candidate: `feat/security-validation-kdf-fuzz-integration` at `bae0315e6f8891df43e1d4986d8c1440b81de021` (stacked on the Crypto candidate; not independently mergeable to `main`).
- KDF-related test files added to the integration candidate: `crates/ark-crypto/src/kdf/kat_tests.rs`, `crates/ark-crypto/tests/kdf_tests.rs`, and `crates/ark-crypto/tests/adversarial_negative_tests.rs`.

This is static source reconciliation. No claim is made that these tests have compiled or executed on the candidate head.

## 3. Rows requiring correction in the original matrix

| Invariant | Original matrix description | Current source evidence | Correct current disposition |
|---|---|---|---|
| ARK-SEC-013 — No Home-Grown Cryptography | KDF uses `argon2`, `hkdf`, `sha2`; no repository-wide prohibition | The candidate KDF crate uses those dependencies. This establishes the current KDF implementation path only, not a permanent technical ban on custom cryptography. | **Implemented for current KDF code; governance/CI guard remains incomplete.** Dependency audit execution remains unverified. |
| ARK-SEC-016 — Key Separation | Arbitrary bounded `derive_subkey` context; registry pending | Public derivation is `derive_purpose_subkey(HkdfIkm, salt, HkdfKeyPurpose)`; the six purpose labels are closed. `HkdfContext::parse` rejects unregistered labels. Raw `derive_subkey` is private. | **Enforced at the current KDF API boundary; test execution pending.** Domain separation is not authorization and does not protect compromised IKM. |
| ARK-SEC-017 — No Raw Master-Key Persistence | Zeroizing output wrapper; storage deferred | KDF outputs are `Zeroizing<[u8; 32]>`. This says nothing about persistence because the storage implementation is absent. Borrowed caller inputs are not owned/zeroized by this API. | **Partially implemented.** Output-buffer drop scrubbing exists; persistence, copies, OS paging and caller input lifecycle remain outside this evidence. |
| ARK-SEC-018 — Bounded Parsing | Password/salt/context limits; Argon2 cost bounds pending | Production `derive_kek` validates password length, salt length/non-degeneracy and `Argon2idConfig::validate()` before invoking Argon2. Hard bounds are 16–256 MiB, 1–10 iterations, 1–8 lanes. | **Enforced at the production KDF entry point; test execution pending.** |
| ARK-SEC-027 — Bounded Resource Use | `Argon2idConfig` described as unbounded | Direct public-field construction remains possible, but `derive_kek` revalidates the config before expensive work. | **Per-call KDF workload bounded.** Concurrent calls can still exhaust host resources; service-level admission control/rate limits and platform calibration are not supplied by this primitive. |
| ARK-SEC-028 — Secret-Safe Diagnostics | Static error enums; sentinel suite drafted | Core `ArkError`, `Argon2Error`, and `HkdfError` use fixed classifications/messages without secret payload fields. A-04 sentinel tests now exist in the integration candidate. | **Primitive-level design implemented; test execution and future error-chain/log review remain pending.** Static error types do not prove every caller/log/panic/crash surface is safe. |
| ARK-SEC-019 — Anti-Rollback Is Separate | Version ordering only; trusted state deferred | Core version types expose numeric values/order; the implementation has no trusted monotonic state anchor. | **Documented / not implemented as anti-rollback.** |
| ARK-SEC-030/031 — Compatibility and No Silent Downgrade | Domain-separated version types, parsers deferred | Core types prevent cross-domain type substitution. They do not define serialized format parsing, migration, compatibility negotiation or anti-rollback. | **Partially implemented at vocabulary level; format enforcement deferred.** |
| ARK-SEC-032 — Security Review Gate | Manual process; ruleset automation pending | The workflow and CODEOWNERS are process evidence. The earlier E-02 audit found no required approval or required status-check gates in repository protection. | **Process requirement exists; technical enforcement is incomplete.** |

## 4. Core rows: preserve the original limits

The current Core source still supports the original cautionary statements:

- `WorkspaceId`, `FileId`, `SessionId`, and other typed IDs are fixed-width domain labels. They are not identity proof, capability tokens, or authorization.
- `RuntimeMode::Development/Execution` expresses workflow mode and is not an access-control boundary.
- `ArkError` contains a fixed `ErrorCode` and static display strings. This is a useful safe boundary for this type; it does not sanitize diagnostics produced by future subsystems.
- Version newtypes and `Ord` comparisons do not supply parser rejection, compatibility behavior, trusted monotonic state or anti-rollback.

The matrix's references to `crates/ark-core/tests/negative_core_tests.rs` and `security_invariants_tests.rs` must not be represented as main-branch CI evidence unless those files are actually integrated into the tested revision. Ali's historical branch contains such test files; the current main tree must be checked separately before attributing their coverage to main.

## 5. Evidence gaps and next steps

1. Run the workspace test suite on the exact integrated candidate SHA after the missing KAT module and integration tests are present.
2. Run Clippy and formatting on the same SHA; do not transfer old PR #3 CI results to this branch.
3. Run both bounded KDF fuzz targets and record the exact commit, nightly toolchain, cargo-fuzz version, command line, runtime, corpus, coverage and crash triage.
4. Obtain Security Validation owner review of the updated matrix and the integrated test/fuzz artifacts.
5. Keep invariants for AEAD, nonce management, storage, identity, broker, session, recovery and filesystem enforcement marked not implemented until those components exist.

## 6. Disposition

A-03's artifact is found and its stale KDF rows are now reconciled here. This does **not** mean Ali has approved the changes, that the source matrix has been updated, that the candidate is merged, or that tests have passed. The source matrix remains Ali's deliverable; this document is a Systems Engineering cross-check to make the current evidence and gaps explicit.
