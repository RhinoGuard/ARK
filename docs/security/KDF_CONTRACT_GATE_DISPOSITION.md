# KDF Contract-Gate Disposition and PR Reconciliation

- Umbrella work: S-07 / Issue #37; prior coordination: Issues #5, #6, #29.
- Delivery PR: [#35](https://github.com/RhinoGuard/ARK/pull/35).
- Branch: `feat/crypto-kdf-contract-gates`, based on PR #28 head `3ab35c3acf00d0dbd4df7e14ec0fc199e904a2a9`.
- CI-validated source/documentation head: `87a6c438df5150715d36037b2453a53912f695c6`; later docs-only refresh commits are not yet CI-validated.
- Current PR state at S-07 refresh: open, Ready for Review, base `fix/crypto-kdf-production-api-boundary`; not merged.
- This record is an implementation/evidence disposition, not independent security approval or merge authorization.

## 1. CCB disposition

| Gate | Disposition in this branch | Evidence | Remaining condition |
|---|---|---|---|
| CCB-01 / CCB-06 — IKM trust and provenance | Contract clarified; regression test demonstrates length validation is not entropy validation. | `HKDF_DOMAIN_SEPARATION_CONTRACT.md`; `hkdf_ikm_length_validation_does_not_claim_entropy`. | Caller provenance must be enforced by future credential/key-lifecycle owner; independent review. |
| CCB-02 — Six HKDF purposes | Exact labels and intended roles documented; test locks all six byte labels and checks distinct derivations. | `HKDF_DOMAIN_SEPARATION_CONTRACT.md`; `hkdf_registered_purpose_registry_is_stable`. | Explicit Crypto-owner acceptance of the six purpose semantics is not yet recorded. Existing E-03 comment on Issue #17 confirms the Core-to-Crypto boundary, not the six-purpose registry. Independent review remains. |
| CCB-03 — CSPRNG, salt, key generation | Ownership boundary clarified: `ark-crypto` validates/derives only; future credential lifecycle obtains OS CSPRNG bytes, persists salt, and reuses stored salt on unlock. API docs corrected to say the KDF does not generate randomness. | `ARGON2ID_SECURITY_CONTRACT.md`; updated `derive_kek` docs; Crypto-owner E-03 comment on Issue #17 confirms the boundary. | No credential-lifecycle service exists, so no OS RNG integration is implemented. Tracked by [Issue #31](https://github.com/RhinoGuard/ARK/issues/31). |
| CCB-04 — Argon2 calibration | Reproducible six-profile harness and manual Windows/Linux/macOS workflow added. | `crates/ark-crypto/benches/argon2id_calibration.rs`; `.github/workflows/argon2id-calibration.yml`. | Workflow still needs to be dispatched; artifacts, representative constrained-hardware evidence, concurrency pressure, and process peak-RSS measurements remain outstanding. [Issue #34](https://github.com/RhinoGuard/ARK/issues/34). |
| CCB-05 — Service-level sanitized errors | Explicitly deferred; current workspace has no service/auth/session/broker boundary. Existing KDF errors are fieldless and static. | `ARGON2ID_SECURITY_CONTRACT.md`; `HKDF_DOMAIN_SEPARATION_CONTRACT.md`. | Implement mapping when service exists; [Issue #32](https://github.com/RhinoGuard/ARK/issues/32). |
| CCB-07 — Persisted KDF metadata and migration | Explicitly deferred; no storage or credential-record subsystem exists. No speculative persistence format added. | `ARGON2ID_SECURITY_CONTRACT.md`. | Storage owner must define authenticated metadata, versioning, migration, and rollback behavior; [Issue #33](https://github.com/RhinoGuard/ARK/issues/33). |

## 2. Architecture boundary decisions

The current workspace has only `ark-core` and `ark-crypto`. The accepted architecture keeps `ark-core` free of concrete OS/crypto implementations and keeps `ark-crypto` focused on deterministic cryptographic primitives. The repository has no implemented `ark-storage`, `ark-session`, `ark-broker`, workspace credential service, or authentication service.

Accordingly:

- Do not add OS entropy generation to `ark-core`.
- Do not add a random source to `ark-crypto` without a reviewed architecture/API change and a real lifecycle owner.
- Generate a fresh 32-byte salt at creation/re-key time using the approved OS CSPRNG abstraction in the future credential lifecycle component.
- Persist the exact salt and parameters with the wrapper. Unlock loads that same salt; it never regenerates it.
- Fail closed if OS entropy acquisition fails; no deterministic/time/PID/zero-filled fallback.
- The KDF validates shape and derives. It cannot prove caller provenance, enforce service rate limits, or authenticate persisted metadata.

## 3. PR #3 versus PR #28 reconciliation

The repository compare endpoint reports the branches as diverged from merge base `bafa677d6b42e7b04b736fcc386cd8e4ea4d9943`: comparing PR #3 head `d705efd042ca31113eab3b2bdf63fb643fd27f8b` to PR #28 head `3ab35c3acf00d0dbd4df7e14ec0fc199e904a2a9` reports 16 commits ahead and 4 behind in that direction.

Disposition:

1. PR #28 remains the selected hardened Crypto integration candidate, as directed by Issue #29.
2. PR #3 must not be merged independently, rebased, force-pushed, or rewritten.
3. PR #35 is stacked directly on PR #28's exact head and does not merge PR #3 history.
4. The PR #3 Argon2id/HKDF contract documentation is restored in updated form; the old single-machine benchmark claim is not presented as fresh multi-platform evidence.
5. The missing IKM provenance regression and all-six-purpose registry regression are added directly to the current PR #28-based source tree.
6. Any further PR #3-only behavior must be ported only if a concrete acceptance requirement is demonstrated to be absent from PR #28 and reviewed in this stacked PR.

This preserves the staged ancestry and avoids creating a second independent Crypto merge path.

## 4. Validation status — refreshed for S-07

- PR #35 exact head: `6b52fed41452782cc67a22a950d02411e815cb42`.
- Base PR #28 head: `3ab35c3acf00d0dbd4df7e14ec0fc199e904a2a9`.
- Live compare: 17 commits ahead / 0 behind.
- GitHub Actions run [#45400557291](https://github.com/RhinoGuard/ARK/actions/runs/45400557291) on source head `6b52fed`: completed successfully, 6/6 jobs.
- Later documentation-only commits advanced the branch; no CI run/status is currently visible for the newer head, so exact-current-head CI remains pending.
- Windows, macOS, and Ubuntu formatting, workspace build, Clippy, and tests passed; Rust 1.85 MSRV formatting/build/Clippy/tests passed; locked dependency audit and commit-message validation passed.
- The prior handoff's statement that completed exact-head CI was unavailable is superseded by this run.
- The Argon2id calibration workflow has not yet been dispatched for this candidate; no S-07 benchmark artifacts are available.
- The harness reports configured Argon2 memory cost, not process peak RSS. Two warm-ups and seven timed samples are currently configured in the workflow; p95 from this sample count is coarse and should not be overinterpreted.
- No local Rust toolchain commands were executed in this GitHub editing session.
- Independent Security Validation review remains mandatory under ARK-SEC-032. No PR #35 review submission is currently recorded.

## 5. Explicit non-claims

This branch does not implement or claim OS CSPRNG salt generation, credential persistence, authentication throttling, global KDF admission control, service-level error mapping, authenticated metadata, AEAD, nonce lifecycle, key wrapping, storage, recovery, cross-platform calibration completion, or an independent security audit.

## 6. Human review required

- [ ] Crypto owner explicitly confirms the CCB-02 six-purpose registry semantics and compatibility contract.
- [x] Crypto owner confirms the CCB-03 salt-generation ownership split in the E-03 comment on Issue #17.
- [ ] Independent reviewer approves the KDF API/docs/tests and RNG ownership boundary.
- [ ] CI passes on the latest documentation-only PR #35 head. Run #45645080815 passed on `87a6c43` (6/6 jobs), before the final Markdown evidence refresh.
- [ ] Cross-platform benchmark workflow artifacts are collected and reviewed.
- [ ] Process peak-memory and representative constrained-hardware/concurrency evidence are collected or explicitly dispositioned.
- [ ] Issues #31–#34 remain linked as explicit follow-up work.
