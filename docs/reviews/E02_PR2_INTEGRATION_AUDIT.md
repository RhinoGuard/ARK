# E-02 — PR #2 Integration and Merge-Readiness Audit

**Audit date:** 2026-11-19  
**Issue:** [#16 — E-02](https://github.com/RhinoGuard/ARK/issues/16)  
**Reviewed PR:** [#2 — Rust Core foundation](https://github.com/RhinoGuard/ARK/pull/2)  
**Audit mode:** GitHub metadata, source, history, review, CI, and ruleset inspection. No local Rust commands were executed as part of this audit.

## 1. Disposition

**Overall E-02 disposition: CHANGES REQUIRED for downstream Foundation integration.**

PR #2 itself is already merged and its Core baseline is consistent with the accepted ADR-0005. This audit found no blocking defect in the merged `ark-core` implementation within the reviewed scope. The remaining blockers are in the dependent Crypto and Security Validation workstreams and in their integration history/API contracts; they must not be misreported as defects in PR #2.

PR #2 must not be reopened or re-merged. Its merge is recorded below as historical evidence. The next-stage Foundation cannot be called integrated until the downstream blockers in Sections 4–7 are addressed.

## 2. Exact revision and decision evidence

| Item | Verified value |
|---|---|
| PR #2 final reviewed head | `03336335ae5f87dfc93f70ea56f5448265cb1749` |
| PR #2 merge commit | `bafa677d6b42e7b04b736fcc386cd8e4ea4d9943` |
| Current `main` at audit time | `68dad185bd5572f3abd557a61059415a932ca95d` |
| PR #2 state | Closed / merged |
| ADR-0005 | **Accepted** on current `main` |
| E-01 owner record | `docs/decisions/ADR-0005-REVIEW.md` exists on current `main` |

The ADR status in the current repository is authoritative. Older PR descriptions and stacked branches that still say ADR-0005 is Proposed or PR #2 is open are stale historical text and must be updated when those branches are ported.

## 3. PR #2 implementation and verification

The merged Core baseline provides the Rust workspace and `ark-core` domain vocabulary. The reviewed boundaries remain intact:

- `ark-core` is dependency-free with respect to third-party crates and contains no cryptographic implementation, persistence, OS access, or authorization engine.
- Typed identifiers are domain labels, not identity proof, capabilities, or permission grants.
- `RuntimeMode` describes workflow, not authorization or isolation.
- Version newtypes provide domain separation and numeric comparison only; they do not implement serialization, migration, or anti-rollback.
- The shared `ArkError` taxonomy provides coarse fixed classifications; it does not prove that diagnostics in future subsystems are automatically sanitized.

### CI evidence

The final PR #2 head `03336335ae5f87dfc93f70ea56f5448265cb1749` has GitHub Actions run [`43006325892`](https://github.com/RhinoGuard/ARK/actions/runs/43006325892). All five jobs completed with conclusion **success**:

| Job | Conclusion |
|---|---|
| Rust checks (Ubuntu) | success |
| Rust checks (Windows) | success |
| Rust checks (macOS) | success |
| Minimum supported Rust version | success |
| Validate commit messages | success |

This is CI evidence for the recorded PR head. It is not evidence that this audit ran Cargo locally, and it is not a security audit of unimplemented subsystems.

### Review evidence

PR #2's recorded formal review is `COMMENTED`, not `APPROVED`. Owner confirmations were subsequently recorded in the PR discussion and E-01 issue record. The repository ruleset currently requires zero approving reviews, so this history is not equivalent to a configured GitHub approval gate.

## 4. Repository policy and merge controls

The active ruleset [ARK-Main-Protection](https://github.com/RhinoGuard/ARK/rules/27117937) applies to the default branch. Its effective rules are:

- branch deletion prohibited;
- non-fast-forward updates prohibited;
- pull request required;
- linear history required;
- required approving review count: **0**;
- Code Owner review: **not required**;
- required status checks: **empty**;
- bypass actors: none; current user bypass: never.

The default branch endpoint reports protection enabled, while the classic branch-protection endpoint is disabled; the active repository ruleset is the relevant control source. No required-signatures rule is present in the retrieved ruleset. Consequently, PR workflow and linear history are enforced, but GitHub does not enforce an approval or CI-check gate through this ruleset. The repository's contributor/review policy is stricter than the configured minimum and should be honored manually until the ruleset is deliberately strengthened.

**Policy finding — High (governance):** zero required approvals and zero required status checks allow a PR to satisfy GitHub's configured gate without an independent review or green CI requirement.  
**Owner:** repository administrators / Systems Engineering.  
**Disposition:** keep this as an explicit governance risk; do not silently change ruleset settings during E-02.

## 5. Crypto dependency assessment — PR #3

PR [#3](https://github.com/RhinoGuard/ARK/pull/3), owned by `ZeroTrace79`, is open at head:

`d705efd042ca31113eab3b2bdf63fb643fd27f8b`

The PR currently targets `main`, but the exact comparison against current `main` is **diverged: 4 commits ahead / 2 behind**, with merge base `bafa677d6b42e7b04b736fcc386cd8e4ea4d9943`. Its metadata reports `mergeable: true`; that flag does not make the compare graph up to date and must not replace an explicit synchronization/review step.

The current PR head has five GitHub Actions jobs completed successfully (Ubuntu, Windows, macOS, MSRV, and commit-message validation). The prior formal review remains `CHANGES_REQUESTED`; it has not been dismissed or superseded by a new formal approval.

### Findings requiring resolution before PR #3 can be accepted

| Severity | Finding | Evidence / consequence | Owner | Required disposition |
|---|---|---|---|---|
| High | KAT-only helpers are exported through the public production KDF module | `derive_argon2id_rfc9106_kat` and `hkdf_sha256_rfc5869_expand` are re-exported from `crates/ark-crypto/src/kdf/mod.rs`. The Argon2 helper deliberately accepts RFC test-vector conditions outside the normal production salt/parameter policy; the HKDF helper deliberately accepts generic RFC IKM/info/output inputs. This expands production API surface and permits callers to bypass the intended production entry points. | Crypto | Move conformance-only helpers behind test-only/internal support and keep published KATs executable without exporting test escape hatches in production builds. |
| High | Raw IKM remains accepted by the general HKDF derivation entry point | `derive_subkey(input_key_material: &[u8], ...)` performs a length check by constructing `HkdfIkm`, but the public function still accepts arbitrary caller bytes. A 16-byte minimum does not establish entropy or provenance; a human password of sufficient length can still be passed. | Crypto + Systems Engineering | Make the accepted IKM boundary explicit in E-03 and align the public API with the approved typed contract. Do not claim the wrapper proves entropy. |
| Medium | Context parsing is broader than the closed purpose enum suggests | `HkdfContext::parse` checks the `ARK/v1/` prefix and printable suffix, so arbitrary purpose strings are accepted by `derive_subkey`. This conflicts with documentation that describes the purpose set as closed and with the typed `HkdfKeyPurpose` path. | Crypto | Choose and document one policy: closed enum-only purposes, or an explicitly extensible namespace with collision/ownership rules. Add negative tests for the selected grammar. |
| Medium | Argon2 parameter calibration is not yet demonstrated across supported targets | PR #3 describes local benchmark execution on one Windows 10 / Intel i3-10100F / 16 GiB system. The current evidence does not establish performance acceptability across all supported platforms or low-resource configurations. | Crypto + Systems Engineering | Treat current defaults as provisional until a reproducible calibration matrix and target resource budget are documented. |
| High | Branch synchronization and formal review remain incomplete | Compare reports 2 commits behind/diverged; the prior formal review is still `CHANGES_REQUESTED`. | Crypto | Synchronize against current `main`, answer each review item, obtain independent review, and rerun CI on the final head. |

These findings concern the proposed Crypto integration, not the already-merged Core code. KDF-only scope must also remain explicit: AEAD, key wrapping, nonce lifecycle, key hierarchy orchestration, storage, and recovery are not delivered by PR #3.

## 6. Security Validation and fuzzing integration

The Security Validation owner is `alieliteworks-debug`. Relevant branches were compared against current `main`:

| Branch | Compare result against current `main` | Integration assessment |
|---|---:|---|
| `feat/security-testing-framework` | 13 ahead / 3 behind; diverged | Do not merge wholesale. It includes the original Foundation/workspace history as well as Security Validation changes. Port only owner-specific files/commits onto a fresh branch based on current `main`. |
| `feat/kdf-fuzzing-harness` | 14 ahead / 3 behind; diverged | Do not merge wholesale. Port the fuzz package/targets only after adapting them to the current `ark-crypto` API and current workspace. |
| `docs/kdf-threat-model` | 3 ahead / 3 behind; diverged | Rebase/port the document onto current `main`; reconcile claims with the current KDF code and review findings. |
| `docs/security-traceability-matrix` | 3 ahead / 3 behind; diverged | Rebase/port the matrix and map claims to actual code/tests rather than planned controls. |
| `docs/crypto-security-review` | 3 ahead / 3 behind; diverged | Update the review against the current PR #3 head; the existing report predates the current PR changes. |
| `test/adversarial-negative-tests` | 4 ahead / 3 behind; diverged | Do not merge as-is: its diff introduces a separate root package layout and removes the root README; port/adapt only relevant tests into the current `crates/ark-crypto` layout. |

### Fuzzing finding — High

The current `feat/kdf-fuzzing-harness` target code explicitly documents that the KDF targets do **not** call `derive_kek` or `derive_subkey`. The Argon2 target locally simulates parameter/password/salt bounds; the HKDF target locally simulates non-empty IKM/context checks. Its `fuzz/Cargo.toml` depends on `ark-core`, not `ark-crypto`.

Therefore A-05 must not be described as fuzzing the actual cryptographic API. It is currently validation scaffolding only.  
**Owner:** Security Validation (`alieliteworks-debug`), with Crypto API coordination from `ZeroTrace79`.  
**Required disposition:** add `ark-crypto` as a fuzz-package dependency on the synchronized workspace, invoke the real public KDF entry points with tightly bounded inputs/configuration, and retain negative/error-path assertions. Record actual fuzz duration, corpus, command, and run result; do not claim fuzzing occurred until it does.

The Core-specific fuzz targets on `feat/security-testing-framework` do call `ark-core` constructors and formatting/accessor methods. They are distinct from KDF fuzzing and should be assessed on their own merits.

## 7. E-04 stacked PR and trust-boundary document

PR [#20](https://github.com/RhinoGuard/ARK/pull/20) currently targets `feat/rust-core-foundation` at `03336335ae5f87dfc93f70ea56f5448265cb1749`, not current `main`. Its document and PR body still say ADR-0005 is Proposed and PR #2 is open/unmerged. Both statements are stale: ADR-0005 is Accepted and PR #2 is merged.

**Severity: Medium (documentation/integration).**  
**Owner:** Systems Engineering.  
**Required disposition:** retarget PR #20 to current `main`, correct its status/baseline text, then review the system-level trust-boundary content against the accepted ADR, current PR #3 KDF behavior, and explicit implemented/proposed/unknown labels. Do not merge the stale version.

## 8. Required next actions

1. **Crypto / `ZeroTrace79`:** resolve the KAT export, IKM provenance/API, HKDF purpose grammar, benchmark-evidence, and synchronization/review findings in PR #3.
2. **Security Validation / `alieliteworks-debug`:** rebuild the relevant test/fuzz branches on current `main`; ensure A-05 fuzzes actual `ark-crypto` APIs; port A-02/A-03/A-04/A-06 artifacts without importing the old Foundation history.
3. **Systems Engineering:** complete E-03 before locking the Core-to-Crypto public boundary; retarget and update PR #20 for E-04; retain the ruleset gap as a governance finding for a separate decision.
4. **Integration:** require independent owner review and fresh CI on each final PR head. No branch should be merged solely because the GitHub metadata says `mergeable: true`.

## 9. Scope and limitations

- This report is based on the exact refs and GitHub evidence listed above as retrieved on 2026-11-19.
- No local `cargo test`, `cargo clippy`, `cargo fmt`, benchmark, or fuzzing command was run for this audit.
- The successful PR #2 CI run verifies the recorded Rust workflow jobs; it does not prove every security invariant is implemented.
- Static inspection and branch comparison do not constitute a cryptographic audit, formal verification, or production-readiness certification.
- No repository ruleset, branch, existing review, or merge state was modified as part of this audit artifact.


## 10. Incremental revalidation — 2026-11-20

This addendum does not rewrite the 2026-11-19 historical audit baseline. It corrects the downstream integration snapshot after the staged KDF and Security Validation work was advanced.

### 10.1 PR #3 and staged Crypto candidate are different refs

- PR #3 remains open against `main` at its old head `d705efd042ca31113eab3b2bdf63fb643fd27f8b`. Its prior formal review remains `CHANGES_REQUESTED`. The old PR #3 CI success applies only to that exact older head.
- A separate branch, `fix/crypto-kdf-production-api-boundary`, is staged at `552041395ee9097902eb6a7d1c934ed55b57a99f`. It hardens the production KDF API boundary but is not itself merged or a replacement PR for Saleh's PR #3.
- [Draft PR #26](https://github.com/RhinoGuard/ARK/pull/26) adds KAT/test recovery, adversarial tests, fuzz integration and current review reconciliations on top of that staged Crypto branch. PR #26 is open and deliberately targets `fix/crypto-kdf-production-api-boundary`, not `main`. Its head at this addendum is `076c84082a6259b9c1c724327bdc32ba614581b8`.

### 10.2 Updated candidate evidence

The following checks passed on staged code, not on PR #3's old head:

- [ARK CI #42](https://github.com/RhinoGuard/ARK/actions/runs/44624226086), source head `be376f62ee454ca359f79c278f69fe3d882e3231`: all five jobs succeeded (Ubuntu, Windows, macOS, Rust 1.85 MSRV and Conventional Commit validation). Rust formatting, workspace build, Clippy and tests passed.
- [ARK CI #46](https://github.com/RhinoGuard/ARK/actions/runs/44624894201), latest PR #26 head `076c84082a6259b9c1c724327bdc32ba614581b8`: all five jobs succeeded.
- [KDF fuzz smoke #7](https://github.com/RhinoGuard/ARK/actions/runs/44624226096), source head `be376f62ee454ca359f79c278f69fe3d882e3231`: real API-connected Argon2id and HKDF targets completed bounded 20-second runs without a reported crash.
- [KDF fuzz smoke #10](https://github.com/RhinoGuard/ARK/actions/runs/44624894126), latest PR #26 head `076c84082a6259b9c1c724327bdc32ba614581b8`: formatting, compilation, both fuzz targets and artifact upload all succeeded; no crash was reported. It used nightly Rust `1.102.0-nightly`, Cargo `1.102.0-nightly`, and cargo-fuzz `0.13.2`. Argon2id completed 428 runs / 21 seconds (coverage 338, features 348, RSS 330 MiB); HKDF completed 2,718,772 runs / 21 seconds (coverage 101, features 115, RSS 466 MiB). The artifact was uploaded as ID `14579133051`, SHA-256 `2d80c6fa8c221d41b1705dffd5bcbc032a604d7127fc8ef0e3d80ac910dff210`.

These are staged-candidate results only. They do not transfer to PR #3 until its branch is updated to the reviewed implementation and exact-head CI/review are repeated.

### 10.3 Reclassification of historical Security Validation findings

The original Section 6 statements about `feat/kdf-fuzzing-harness` remain correct for that historical branch: its KDF targets simulate bounds and do not call the real `ark-crypto` derivation API. They are no longer a description of the replacement candidate in Draft PR #26:

- A-03 traceability artifact was located at `docs/security/TRACEABILITY_MATRIX.md` on `docs/security-traceability-matrix`; a path-guessing error in the earlier integration note was corrected.
- A-04: eight adversarial negative tests are part of the staged `ark-crypto` tests and passed the candidate CI runs.
- A-05: replacement fuzz targets call actual `derive_kek`, `HkdfContext::parse`, `HkdfIkm::from_slice`, and `derive_purpose_subkey` entry points. Bounded smoke runs #7 and #10 succeeded. Sustained fuzzing and broader triage remain outstanding.
- A-06: Ali's original report remains historical/preliminary and predates this candidate. The new A-06 document is authored as a Systems Engineering cross-check; Ali's fresh independent review and Saleh's response have not been recorded.

### 10.4 E-04 and governance status

- PR #20 is closed without merge. The clean, current-`main` replacement is PR #24, whose trust-boundary document now contains a dated candidate/KDF evidence snapshot. PR #24 remains open and owner review is outstanding.
- The ruleset finding remains unchanged: the active ruleset still has zero required approving reviews and an empty required-status-check list. This addendum does not change repository policy.
- PR #22 remains documentation-only and open. It does not itself approve, merge, or close any dependent work.

### 10.5 Current E-02 disposition

**PR #2 remains successfully merged and has no blocking defect identified in the audited Core scope. E-02 remains CHANGES REQUIRED for downstream Foundation integration and governance until PR #3 is synchronized and owner-reviewed, PR #26 is reconciled with the Crypto owner branch, E-03/E-04 owner decisions are recorded, and the repository review/CI policy gap is addressed through an explicit administrator decision.**

No issue was closed, no review/approval was submitted, no repository rule was changed, and no branch was merged by this addendum.


## 11. Revalidation after hardened Crypto PR creation (2026-11-20)

The current integration plan now uses a three-PR staged candidate rather than the older PR #3 branch:

- PR #28 is the hardened Crypto/KDF candidate against main, currently at c22acc0474ef067ad90dd5bbced7ed32f9f6d343. Its previous CI attempt at b5d0c6ab1c0cab6729a15a03f6c301a803c9a071 failed due to test-only KAT helper visibility/dead-code errors. Those source issues were corrected in later commits, but the latest head still needs a fresh CI run.
- PR #26 is Security Validation stacked on PR #28. Its head 7e9f47393628e605e8d0ff400c579157da52f5bd passed ARK CI #64, including the dependency audit, and bounded KDF fuzz smoke #21. The PR branch is four commits behind the current PR #28 head; therefore this green evidence is not proof of the current combined stack.
- PR #27 adds follow-up negative tests on PR #26. Its previous green runs are historical for an earlier base snapshot and must be repeated after synchronization.

The current exact-head PR #28 CI status is pending; do not describe it as passed. The E-03 contract still has open CCB-01/06 provenance/source semantics and CCB-03/04/05/07 platform entropy, calibration, service error mapping, and format metadata/migration decisions. The ruleset approval/status-check gap remains a separate governance issue.

Updated E-02 disposition: PR #2 remains merged and its audited Core scope remains accepted. Downstream Foundation integration remains CHANGES REQUIRED until PR #28 current head passes, PR #26/#27 are synchronized and revalidated, the open E-03 decisions are dispositioned, and E-04/E-05 reviews are recorded. No merge, close, approval, or ruleset change is implied by this addendum.


### Latest exact-head check (2026-11-20)

PR #28 advanced to head 3ab35c3acf00d0dbd4df7e14ec0fc199e904a2a9. ARK CI run #44775710932 on this exact head has passed Ubuntu, Windows, macOS, Rust 1.85 MSRV, and commit-message validation. The pinned cargo-audit 0.23.0 job is still running; the overall run is not yet green until that job completes. The current PR #26 branch has identical blob contents for the KDF implementation files and the CI workflow, but its commit graph remains four commits behind PR #28; its docs index and fuzz workflow are Security Validation additions. PR #26's earlier CI/audit/fuzz success remains tied to its own exact head, and PR #27 still needs a fresh combined-stack validation. No merge-readiness claim is made.
