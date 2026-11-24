# KDF Contract-Gate Disposition and PR Reconciliation

- Related work: Issues #5, #6, #29.
- Branch: `feat/crypto-kdf-contract-gates`, based on PR #28 head `3ab35c3acf00d0dbd4df7e14ec0fc199e904a2a9`.
- Status: implementation and disposition record for Draft review. This is not an independent security approval or merge authorization.

## 1. CCB disposition

| Gate | Disposition in this branch | Evidence | Remaining condition |
|---|---|---|---|
| CCB-01 / CCB-06 — IKM trust and provenance | Contract clarified; regression test proves length validation is not entropy validation. | `HKDF_DOMAIN_SEPARATION_CONTRACT.md`; `hkdf_ikm_length_validation_does_not_claim_entropy`. | Caller provenance must be enforced by future credential/key-lifecycle owner; independent review. |
| CCB-02 — Six HKDF purposes | Exact labels and intended roles documented; test locks all six byte labels and checks distinct derivations. | `HKDF_DOMAIN_SEPARATION_CONTRACT.md`; `hkdf_registered_purpose_registry_is_stable`. | Crypto-owner explicit confirmation and independent review still required; no existing label may be silently repurposed. |
| CCB-03 — CSPRNG, salt, key generation | Ownership boundary clarified: `ark-crypto` validates/derives only; future credential lifecycle obtains OS CSPRNG bytes, persists salt, and reuses stored salt on unlock. API docs corrected to say it does not generate randomness. | `ARGON2ID_SECURITY_CONTRACT.md`; updated `derive_kek` docs. | No credential-lifecycle service exists, so no OS RNG integration is implemented. Tracked by [Issue #31](https://github.com/RhinoGuard/ARK/issues/31). |
| CCB-04 — Argon2 calibration | Reproducible harness and manual Windows/Linux/macOS workflow added. | `crates/ark-crypto/benches/argon2id_calibration.rs`; `.github/workflows/argon2id-calibration.yml`. | Workflow must be run; artifacts and representative constrained-hardware/peak-RSS evidence remain outstanding. [Issue #34](https://github.com/RhinoGuard/ARK/issues/34). |
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

The repository compare endpoint reports the branches as **diverged** from merge base `bafa677d6b42e7b04b736fcc386cd8e4ea4d9943`: comparing PR #3 head `d705efd042ca31113eab3b2bdf63fb643fd27f8b` to PR #28 head `3ab35c3acf00d0dbd4df7e14ec0fc199e904a2a9` reports 16 commits ahead and 4 behind in that direction.

Disposition:

1. PR #28 remains the selected hardened Crypto integration candidate, as directed by Issue #29.
2. PR #3 must not be merged independently, rebased, force-pushed, or rewritten.
3. This branch is stacked directly on PR #28's exact head and does not merge PR #3 history.
4. The PR #3 Argon2id/HKDF contract documentation is restored here in an updated form; the old single-machine benchmark claim is not presented as fresh multi-platform evidence.
5. The missing IKM provenance regression and all-six-purpose registry regression are added directly to the current PR #28-based source tree.
6. Any further PR #3-only behavior must be ported only if a concrete acceptance requirement is demonstrated to be absent from PR #28 and reviewed in this stacked PR.

This preserves the staged ancestry and avoids creating a second independent Crypto merge path.

## 4. Validation status

- The branch is based on PR #28 head `3ab35c3acf00d0dbd4df7e14ec0fc199e904a2a9`.
- A benchmark harness and manual cross-platform workflow are committed, but the workflow has not yet been run from this branch in this handoff.
- No local Rust toolchain execution has been performed through this GitHub editing session.
- Do not treat existing historical CI or benchmark records from PR #3/#28 as validation of this branch's exact head.
- The exact new PR merge result must pass repository CI; the benchmark workflow must be run separately and its artifacts reviewed.
- Independent security review is mandatory under ARK-SEC-032.

## 5. Explicit non-claims

This branch does not implement or claim OS CSPRNG salt generation, credential persistence, authentication throttling, global KDF admission control, service-level error mapping, authenticated metadata, AEAD, nonce lifecycle, key wrapping, storage, recovery, or a security audit.

## 6. Human review required

- [ ] Crypto owner confirms the CCB-02 six-purpose registry semantics and the CCB-01/06 provenance contract.
- [ ] Crypto owner confirms the CCB-03 salt-generation ownership split.
- [ ] Independent reviewer approves the KDF API/docs/tests and RNG ownership boundary.
- [ ] CI passes on the exact new PR merge result.
- [ ] Cross-platform benchmark workflow artifacts are collected and reviewed.
- [ ] Issues #31–#34 remain linked as explicit follow-up work.
