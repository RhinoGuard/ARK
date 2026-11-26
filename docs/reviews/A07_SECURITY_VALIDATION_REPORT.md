# A-07 Independent Security Validation and Foundation Gate Report

- **Issue:** [#30 — `[A-07]` Finalize independent Security Validation and Foundation gate evidence](https://github.com/RhinoGuard/ARK/issues/30)
- **Owner / Reviewer:** `@alieliteworks-debug` (Security Validation)
- **Date:** 2026-11-26
- **Scope:** Independent technical review of PRs #28, #26, #27, and #35; substantive documentation reviews of PRs #22–#25; API-connected 30-minute sustained fuzzing of Argon2id and HKDF-SHA-256; repository governance and merge-readiness audit.
- **Boundary statement:** This report records independent validation evidence and gate dispositions for the KDF-only foundation candidate. It makes **no blanket claim** that ARK or `ark-crypto` is "secure" or that Foundation exit is complete. No production `crates/ark-crypto/src/` files were edited by Security Validation, no PRs were merged, no issues were closed, and no repository settings were modified.

---

## 1. Exact Reviewed PR Heads and Validation Environment

### 1.1 Live PR heads audited

| PR | Branch | Exact Head SHA | Base Branch | CI Status on Head | Formal Review Link & Disposition |
|---|---|---|---|---|---|
| [#28](https://github.com/RhinoGuard/ARK/pull/28) | `fix/crypto-kdf-production-api-boundary` | `3ab35c3acf00d0dbd4df7e14ec0fc199e904a2a9` | `main` (`68dad185`) | 6/6 passing | [Review #6071624169](https://github.com/RhinoGuard/ARK/pull/28#pullrequestreview-6071624169) — `COMMENT` (not approved for merge) |
| [#26](https://github.com/RhinoGuard/ARK/pull/26) | `feat/security-validation-kdf-fuzz-integration` | `52199228ce18a8231b517eed909501d7f65d07ad` | `fix/crypto-kdf-production-api-boundary` | 7/7 passing (incl. 20s fuzz smoke) | [Review #6071624494](https://github.com/RhinoGuard/ARK/pull/26#pullrequestreview-6071624494) — `COMMENT` |
| [#27](https://github.com/RhinoGuard/ARK/pull/27) | `fix/kdf-negative-test-hardening` | `dd04d4d03f9a915faec8ffc96b356fd01febf68f` | `feat/security-validation-kdf-fuzz-integration` | 7/7 passing (incl. 20s fuzz smoke) | [Review #6071624944](https://github.com/RhinoGuard/ARK/pull/27#pullrequestreview-6071624944) — `COMMENT` |
| [#35](https://github.com/RhinoGuard/ARK/pull/35) | `feat/crypto-kdf-contract-gates` | `4e2891b5358a78bfb8e17ac9f2f9a87244e567ce` | `fix/crypto-kdf-production-api-boundary` | 6/6 passing | [Review #6071625274](https://github.com/RhinoGuard/ARK/pull/35#pullrequestreview-6071625274) — `COMMENT` |
| [#22](https://github.com/RhinoGuard/ARK/pull/22) | `docs/e02-pr2-integration-audit` | `948bd394b0b273df8b3f06becbafff9a242d3261` | `main` | 5/5 passing | [Review #6043309047](https://github.com/RhinoGuard/ARK/pull/22#pullrequestreview-6043309047) — `COMMENT` |
| [#23](https://github.com/RhinoGuard/ARK/pull/23) | `docs/e03-core-crypto-contract` | `7e9287a110af306ed9ec7ab796a98ea6a6c55daa` | `main` | 5/5 passing | [Review #6043309624](https://github.com/RhinoGuard/ARK/pull/23#pullrequestreview-6043309624) — `COMMENT` |
| [#24](https://github.com/RhinoGuard/ARK/pull/24) | `docs/e04-system-trust-boundaries-main` | `b78a24b5aa2d9aa5e8340312451c0b5962cbcf26` | `main` | 5/5 passing | [Review #6043310263](https://github.com/RhinoGuard/ARK/pull/24#pullrequestreview-6043310263) — `COMMENT` |
| [#25](https://github.com/RhinoGuard/ARK/pull/25) | `docs/next-stage-roadmap-exit-gates` | `dba2538f7a766b60a7b0878fd09a3e9c60f2ef6b` | `main` | 5/5 passing | [Review #6043311252](https://github.com/RhinoGuard/ARK/pull/25#pullrequestreview-6043311252) — `COMMENT` |

*(Note: PR #22 review #6043309047 is attached to the exact live commit `948bd394b0b273df8b3f06becbafff9a242d3261`; its body text contained a 1-character typo `...3661` vs `...3261`.)*

### 1.2 Validation environment

- **Local workstation:** Windows host with Git CLI and GitHub API access; no local Rust/Cargo toolchain or WSL distribution installed. No local `cargo` commands are claimed.
- **Sustained fuzzing runner:** GitHub Actions `ubuntu-latest` (`Linux 7.0.0-1020-azure x86_64`, 4 vCPUs, 16 GiB RAM), `rustc 1.102.0-nightly (6c709eb42 2026-11-25)`, `cargo 1.102.0-nightly (ae8f06eb4 2026-11-21)`, `cargo-fuzz 0.13.2`.

---

## 2. Independent Technical Findings and Dispositions

| ID | Severity | Type | Target & Location (SHA) | Summary & Impact | Owner | Disposition / Tracking |
|---|---|---|---|---|---|---|
| `F-28-01` | **Medium** | Confirmed Defect | PR #28 `crates/ark-crypto/src/kdf/hkdf_sha256.rs:140` (`HkdfIkm<'a>`) @ `3ab35c3a` | `HkdfIkm` derives standard short-circuiting `PartialEq, Eq` on secret key material, creating a timing side-channel risk if callers compare IKM wrappers. | Crypto (`@ZeroTrace79`) | Open — tracked in [#38](https://github.com/RhinoGuard/ARK/issues/38) |
| `F-28-02` | **Medium** | Confirmed Design Gap | PR #28 `crates/ark-crypto/Cargo.toml:11-14` @ `3ab35c3a` | `argon2`, `hkdf`, and `sha2` do not enable crate-level `zeroize` features where supported; intermediate PRK/HMAC/SHA-256/Argon2 state is not scrubbed on drop. | Crypto (`@ZeroTrace79`) | Open — tracked in [#38](https://github.com/RhinoGuard/ARK/issues/38) |
| `F-28-03` | **Low** | Design Concern | PR #28 `crates/ark-crypto/src/kdf/argon2id.rs:114-122` (`Argon2idConfig`) @ `3ab35c3a` | Public struct fields allow constructing unvalidated configs; mitigated because `derive_kek` re-validates fail-closed before allocation. | Crypto (`@ZeroTrace79`) | Accepted-with-limitation for KDF-only milestone |
| `F-28-04` | **Low** | Bounded Input (`ARK-SEC-018`) | PR #28 `crates/ark-crypto/src/kdf/hkdf_sha256.rs:188-206` (`derive_purpose_subkey`) @ `3ab35c3a` | Optional HKDF `salt: Option<&[u8]>` has no upper length bound; unbounded untrusted metadata could cause unnecessary HMAC hashing work. | Crypto (`@ZeroTrace79`) | Open — tracked in [#38](https://github.com/RhinoGuard/ARK/issues/38) |
| `F-28-05` | **Info** | API Hygiene | PR #28 `crates/ark-crypto/src/kdf/hkdf_sha256.rs:284` (`HkdfError::InvalidOutputLength`) @ `3ab35c3a` | Public enum variant is unreachable in production builds (test-only KAT helper). | Crypto (`@ZeroTrace79`) | Informational |
| `F-26-01` | **Medium** | Coverage Gap | PR #26 `fuzz/fuzz_targets/fuzz_argon2id.rs:13-22` @ `52199228` | Original harness only fuzzed 16-byte salts and fixed parameters; skipped empty/oversized passwords, variable salt lengths (`0..=80`), `derive_kek_with_salt`, and `Argon2idConfig::new` bounds. | Security Validation (`@alieliteworks-debug`) | **Remediated** in `test/a07-sustained-kdf-fuzz` (`1b935a72`) |
| `F-26-02` | **Low** | Coverage Gap | PR #26 `fuzz/fuzz_targets/fuzz_hkdf.rs:34` @ `52199228` | Original harness only passed `salt = None` to `derive_purpose_subkey`. | Security Validation (`@alieliteworks-debug`) | **Remediated** in `test/a07-sustained-kdf-fuzz` (`1b935a72`) |
| `F-27-01` | **Low** | Merge Coordination | PR #27 `crates/ark-crypto/src/kdf/kat_tests.rs:332-347` @ `dd04d4d0` | Overlaps with PR #35 additions to `kat_tests.rs`; minor conflict resolution required during integration. | Systems / Crypto / Validation | Tracked for merge sequencing |
| `F-35-01` | **Medium** | Operational Blocker | PR #35 `.github/workflows/argon2id-calibration.yml:3-4` @ `4e2891b5` | `workflow_dispatch`-only workflow cannot be triggered on a feature branch until present on `main`, blocking CI-based CCB-04 calibration prior to merge. | Crypto (`@ZeroTrace79`) / Systems (`@ElmanCod110`) | Open — coordinate with CCB-04 |
| `F-35-02` | **Low** | Doc Accuracy | PR #35 `docs/security/KDF_CONTRACT_GATE_DISPOSITION.md:7,52-55,73` @ `4e2891b5` | Self-referential SHA drift: cites `6b52fed` (L52) and `87a6c43` (L7/L73) as pending while live head `4e2891b5` (25 ahead) already has 6/6 green CI. | Crypto (`@ZeroTrace79`) | Open — clean up before merge |
| `F-GOV-01` | **High** | Governance Gap | Ruleset `ARK-Main-Protection` (`27117937`) & `.github/CODEOWNERS` | `required_approving_review_count: 0`, `require_code_owner_review: false`, `required_status_checks: []`, `dismiss_stale_reviews_on_push: false`, and all `.github/CODEOWNERS` rules are commented out. | Maintainers / Repo Admin | Open — blocks merge-readiness |
| `F-GOV-02` | **Medium** | Orphaned Tracking | Issues [#31](https://github.com/RhinoGuard/ARK/issues/31)–[#34](https://github.com/RhinoGuard/ARK/issues/34) | All four follow-up issues cited as active deferred-gate trackers in PR #35 (`CCB-03`, `CCB-04`, `CCB-05`, `CCB-07`) were closed as `not_planned` on 2026-11-24. | Crypto (`@ZeroTrace79`) / Maintainers | Open — reopen or replace trackers |

---

## 3. API-Connected Sustained Fuzz Validation (≥ 30 Minutes per Target)

### 3.1 Real API connection verification
- `fuzz/fuzz_targets/fuzz_argon2id.rs` directly invokes `ark_crypto::kdf::Argon2idConfig::new`, `Argon2Salt::from_slice`, `derive_kek`, and `derive_kek_with_salt`.
- `fuzz/fuzz_targets/fuzz_hkdf.rs` directly invokes `ark_crypto::kdf::HkdfContext::parse`, `HkdfIkm::from_slice`, and `derive_purpose_subkey` across all six `HkdfKeyPurpose` variants with both `None` and `Some(salt)`.

### 3.2 Campaign configuration and results

- **Branch & Commit:** `test/a07-sustained-kdf-fuzz` @ `1b935a7226ec96d2a6e28f834d8e5d01af0edc94` (stacked on PR #27 head `dd04d4d03f9a915faec8ffc96b356fd01febf68f`)
- **Workflow Run:** [KDF sustained fuzz validation — Run #45792917582](https://github.com/RhinoGuard/ARK/actions/runs/45792917582) (`completed` / `success`)
- **Toolchain:** `rustc 1.102.0-nightly (6c709eb42 2026-11-25)`, `cargo 1.102.0-nightly (ae8f06eb4 2026-11-21)`, `cargo-fuzz 0.13.2`
- **Commands:**
  - `cargo +nightly fuzz run fuzz_argon2id -- -max_total_time=1800 -rss_limit_mb=1024 -max_len=1152 -timeout=10 -print_final_stats=1`
  - `cargo +nightly fuzz run fuzz_hkdf -- -max_total_time=1800 -rss_limit_mb=1024 -max_len=1152 -timeout=10 -print_final_stats=1`

| Target | Seed Provenance | Duration | Total Executions | Avg Exec/s | Coverage (`cov` / `ft`) | Final Corpus | Peak RSS | Crashes / Sanitizer Findings | Retained Artifact & SHA-256 |
|---|---|---:|---:|---:|---|---|---:|---:|---|
| `fuzz_argon2id` | `fuzz/corpus/fuzz_argon2id/seed-valid-password-salt` (17 B) | **1,801 s** (`09:33:12Z`–`10:03:35Z`) | **94,543** | 52 | `388` / `435` *(up from `338`/`348` smoke)* | 47 units / 3,110 B | 333 MiB | **0** | [Artifact #15105082567](https://github.com/RhinoGuard/ARK/actions/runs/45792917582/artifacts/15105082567) (`sha256:4445fb72...e4f620`) |
| `fuzz_hkdf` | `fuzz/corpus/fuzz_hkdf/seed-32-byte-ikm` (32 B) | **1,801 s** (`09:33:30Z`–`10:03:59Z`) | **254,841,152** | 141,499 | `106` / `123` *(up from `101`/`115` smoke)* | 28 units / 2,556 B | 531 MiB | **0** | [Artifact #15105806914](https://github.com/RhinoGuard/ARK/actions/runs/45792917582/artifacts/15105806914) (`sha256:8e9ce22b...0a0805`) |

### 3.3 Crash triage & explicit fuzzing limitations
- **Crash / sanitizer triage:** 0 crashes, 0 panics, 0 timeouts, and 0 ASan/RSS-limit violations across 94,543 Argon2id runs and 254,841,152 HKDF runs.
- **Limitations:** Hosted Linux x86_64 only; valid Argon2id derivations were bounded to `MIN_MEMORY_KIB` (16 MiB) and `1..=2` iterations to fit within `-rss_limit_mb=1024` and achieve sustained throughput. Fuzzing demonstrates panic-free/memory-safe boundary behavior on exercised inputs, not cryptographic strength, side-channel resistance, or multi-threaded concurrency safety.

---

## 4. Governance and Merge-Readiness Audit

Read-only audit against `RhinoGuard/ARK` on 2026-11-26 (no settings modified):

| Control | Documented Requirement (`TEAM_WORKFLOW.md` / `AGENTS.md` / `ARK-SEC-032`) | Actual Live GitHub Configuration | Status |
|---|---|---|---|
| Branch protection / Ruleset | Mandatory independent review; no author self-approval on security-sensitive changes | Legacy branch protection: `404`. Active Ruleset `ARK-Main-Protection` (`27117937`) targets `~DEFAULT_BRANCH` with `deletion`, `non_fast_forward`, `required_linear_history`. | **Partial** |
| Approving reviews | ≥ 1–2 independent reviewers required | `required_approving_review_count: 0`, `dismiss_stale_reviews_on_push: false`, `require_last_push_approval: false`, `required_review_thread_resolution: false`. | **Discrepancy (High)** |
| CODEOWNERS enforcement | Security-sensitive paths owned by ≥ 2 qualified reviewers | `require_code_owner_review: false`, AND every line in `.github/CODEOWNERS` is commented out. | **Discrepancy (High)** |
| Required status checks | CI (`Ubuntu`, `Windows`, `macOS`, `MSRV 1.85`, `commit-messages`, `cargo-audit`) enforced | `required_status_checks: []` (empty list), `strict_required_status_checks_policy: false`. | **Discrepancy (High)** |
| Merge methods | Linear history with squash/rebase per `COMMIT_CONVENTIONS.md` | `allowed_merge_methods: ["merge", "squash", "rebase"]` alongside `required_linear_history`; `delete_branch_on_merge: false`. | **Minor discrepancy** |
| Follow-up issue hygiene | Deferred KDF gates tracked in open issues (#31–#34) | Issues #31–#34 are `closed` (`state_reason: not_planned`) as of 2026-11-24. | **Discrepancy (Medium)** |

---

## 5. Foundation Gate Assessment

| Gate | Status | Evidence & Remaining Blockers |
|---|---|---|
| Core workspace & `ark-core` baseline (ADR-0005 / PR #2) | **Validated** | Merged to `main` (`bafa677`), CI green across Windows/Linux/macOS/MSRV 1.85. |
| KDF public API visibility & fail-closed bounds (PR #28) | **Validated-with-limitations** | Public API narrow, KAT helpers `#[cfg(test)]`, `forbid(unsafe_code)`, 6/6 CI + `cargo-audit` passing; open Medium findings `F-28-01` & `F-28-02` tracked in [#38](https://github.com/RhinoGuard/ARK/issues/38). |
| API-connected sustained KDF fuzzing (≥ 30 min/target) | **Validated-with-limitations** | Run [#45792917582](https://github.com/RhinoGuard/ARK/actions/runs/45792917582): 1,801s Argon2id (94,543 execs) + 1,801s HKDF (254.8M execs), 0 crashes, artifacts retained. Linux x86_64 only; valid Argon2id bounded to 16 MiB. |
| Documentation & contract reviews (PRs #22–#25, #35) | **Validated-with-limitations** | All PRs reviewed on exact live heads with concrete dispositions; minor doc/contract updates and explicit Crypto-owner confirmation of CCB-02 six-purpose semantics remain open. |
| Argon2id cross-platform calibration (CCB-04) | **Not-validated** | Harness exists in PR #35, but no calibration run/artifacts exist yet (`F-35-01` blocks `workflow_dispatch` prior to merge; Issue #34 closed `not_planned`). |
| OS-CSPRNG salt lifecycle (CCB-03), service error mapping (CCB-05), persisted metadata/migration (CCB-07), global concurrency admission control | **Deferred** | Correctly deferred outside KDF-only scope, **but** tracking Issues #31–#33 are currently closed `not_planned` (`F-GOV-02`) and must be reopened or replaced. |
| Repository governance & merge readiness | **Blocked** | Ruleset `27117937` enforces 0 approvals, 0 required checks, and empty `CODEOWNERS` (`F-GOV-01`). |
| **Overall Foundation Exit** | **Blocked / Not Approved** | Do not merge KDF stack to `main` or declare Foundation exit until `F-28-01`/`F-28-02` (#38), CCB-02 owner confirmation, CCB-04 calibration evidence, `F-GOV-01`, and `F-GOV-02` are resolved. |

---

## 6. Concrete Next Actions

1. **Crypto (`@ZeroTrace79`):** Address [#38](https://github.com/RhinoGuard/ARK/issues/38) (`F-28-01` `HkdfIkm` `PartialEq`, `F-28-02` upstream `zeroize` features, `F-28-04` HKDF salt bound), record explicit owner acceptance of the CCB-02 six-purpose registry, and resolve `F-35-01`/`F-35-02`.
2. **Maintainers / Repo Admin:** Enable required approving reviews (≥ 1–2), populate real GitHub handles in `.github/CODEOWNERS` with `require_code_owner_review: true`, configure required CI status checks on Ruleset `27117937` (`F-GOV-01`), and reopen or replace closed `not_planned` Issues #31–#34 (`F-GOV-02`).
3. **Systems Engineering (`@ElmanCod110`):** Incorporate PR #22–#25 review refinements (including global concurrent Argon2id admission-control ownership in `SYSTEM_TRUST_BOUNDARIES.md` and F2 PR #28/#35 alignment in the roadmap) and coordinate final merge order across PR #28, PR #35, PR #26, PR #27, and the A-07 sustained-fuzzing PR.
