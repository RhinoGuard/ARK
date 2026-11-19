# ARK Next-Stage Architecture Roadmap and Foundation Exit Gates

**Status:** Proposed engineering roadmap; owner review required.  
**Issue:** [#19 — E-05](https://github.com/RhinoGuard/ARK/issues/19)  
**Baseline checked:** 2026-11-19  
**Source of truth:** current `main`, accepted ADRs, open PRs, subsystem specifications, and owner-assigned issues.

> This roadmap sequences work; it is not an implementation claim or a delivery-date promise. A document, branch, closed issue, or passing isolated test is not by itself evidence that a subsystem is implemented or ready for production.

## 1. Current state at the roadmap baseline

### Implemented and accepted

| Area | Current state | Evidence | Boundary / caveat |
|---|---|---|---|
| Rust workspace and `ark-core` | Implemented on `main` | PR #2 merged at `bafa677d6b42e7b04b736fcc386cd8e4ea4d9943`; final PR head `03336335ae5f87dfc93f70ea56f5448265cb1749` | Domain vocabulary only. No crypto, storage, identity, authorization, filesystem mediation, or runtime engine. |
| ADR-0005 | Accepted for initial workspace/Core foundation | `docs/decisions/ADR-0005-core-foundation.md`; E-01 record in `ADR-0005-REVIEW.md` | Acceptance is narrow; it does not approve all future crate designs or declare the product foundation complete. |
| Security invariants | Accepted baseline of 32 invariants | `docs/architecture/SECURITY_INVARIANTS.md` | Requirements require enforcement/test evidence. An invariant without enforcement remains a requirement, not a proven property. |
| Architecture direction | Accepted at the product/architecture level, implementation details open | `docs/architecture/ARCHITECTURE.md`, subsystem specifications | Direction is not a proof of working platform mechanisms. |

### In progress / not integrated

| Workstream | Current evidence | Remaining gate |
|---|---|---|
| E-02 Core post-merge integration audit | PR #22 | Independent review and merge; then close Issue #16. |
| E-03 Core-to-Crypto contract | PR #23 | Crypto and Security Validation owner review; settle or explicitly defer CCB-01..CCB-08 decisions; merge accepted contract; close Issue #17. |
| E-04 system trust boundaries | PR #24 (clean branch from current `main`) | Owner review, current baseline verification, merge; close Issue #18. Earlier stacked PR #20 was superseded and closed, not merged. |
| Crypto KDF foundation | PR #3, owner `ZeroTrace79`, open at the baseline head recorded in PR metadata | Resolve existing `CHANGES_REQUESTED` review, synchronize branch with current `main`, close API/security gaps, obtain independent review, and rerun CI on the final head. Not yet on `main`. |
| Security Validation A-01..A-06 | Ali's outputs are on separate branches; integration not complete | Port only relevant artifacts onto clean current-main branches. Do not merge old Foundation history wholesale. Ensure KDF fuzzing invokes real `ark-crypto` APIs. |
| Next-stage architecture decisions | Several accepted directions; detailed protocols remain open | Dedicated ADRs and subsystem-owner reviews per stage below. |

The current repository ruleset requires a PR and linear history, but the retrieved ruleset has **zero required approving reviews and an empty required-status-check list**. Until that policy is deliberately changed, the team must enforce the stricter review/CI requirements manually. This roadmap does not change repository settings.

## 2. Roadmap principles

1. **Main is the integration baseline.** Work begins from the latest `main` or is ported onto a clean branch based on it. A historical feature branch is not assumed to be a safe merge base.
2. **Contracts precede coupling.** Agree on ownership, inputs/outputs, failure semantics, versioning, and threat assumptions before dependent subsystems integrate.
3. **No premature crate materialization.** Create a crate when it owns a concrete testable responsibility; do not create empty placeholders for the whole architecture.
4. **Security success is evidence-based.** Green CI verifies only the checks it actually ran. Benchmarks, fuzzing, adversarial tests, independent review, and platform capability tests are separate evidence classes.
5. **Fail closed at trust boundaries.** Unknown identity, malformed input, stale state, failed integrity, unavailable policy, or resource-limit violations must not become successful access.
6. **No security claims from type names.** Core IDs, versions, modes, and fixed errors do not themselves enforce identity, authorization, anti-rollback, persistence, or secret sanitization across future services.
7. **Preserve subsystem ownership.** Crypto decisions belong to Crypto; Security Validation owns independent threat/test validation; Systems Engineering coordinates architecture and integration. Future Storage, Identity, Platform, and Release owners must be explicitly assigned before their decisions are finalized.
8. **No speculative deadlines.** Sequence by prerequisites and acceptance evidence, not invented calendar commitments.

## 3. Foundation exit criteria

The Foundation is considered stable only when **all mandatory gates below** have evidence on the current `main` history. Passing one gate does not waive another.

### F0 — Decision and repository baseline

- [x] ADR-0005 accepted for the Rust workspace and `ark-core` scope.
- [x] Owner-review record for ADR-0005 committed.
- [x] PR #2 merged; final PR head and merge commit recorded.
- [ ] E-02 audit report accepted and merged (PR #22).
- [ ] E-03 Core-to-Crypto boundary contract owner-reviewed and merged (PR #23).
- [ ] E-04 trust-boundary model owner-reviewed and merged (PR #24).
- [ ] E-05 roadmap accepted and merged.
- [ ] No outstanding stale document describes ADR-0005 as Proposed or PR #2 as unmerged.

### F1 — Core foundation integrity

- [x] Root workspace uses resolver 3, Rust edition 2024, declared MSRV 1.85, workspace lints, and a narrow `ark-core`.
- [x] PR #2 CI passed on its recorded final head across Ubuntu, Windows, macOS, and Rust 1.85 MSRV, including commit-message validation.
- [ ] Any future Core changes must have focused tests and pass the current workspace CI on the exact PR head.
- [ ] The Core boundary must remain free of concrete Crypto, Storage, OS, GUI, persistence, and authorization responsibilities unless a reviewed ADR explicitly changes that boundary.

### F2 — Crypto KDF integration contract

- [ ] PR #3 is synchronized with current `main`; compare state is no longer stale/diverged at the time of final review.
- [ ] Existing formal `CHANGES_REQUESTED` findings are answered and a fresh independent review records the final disposition.
- [ ] RFC 9106 Argon2id and RFC 5869 HKDF known-answer tests run in CI; production builds do not export test/conformance escape hatches.
- [ ] The production HKDF API makes approved IKM handling explicit. A minimum length is not represented as proof of entropy or source provenance.
- [ ] HKDF purpose/context grammar is explicit, canonical, versioned, and tested. Closed-enum versus extensible-purpose behavior is decided by Crypto with Systems Engineering and Security Validation review.
- [ ] Argon2 salt validation is distinct from salt generation. The CSPRNG provider, caller contract, and uniqueness domain are documented before persistent wrappers depend on them.
- [ ] Parameter defaults and hard ceilings are documented as provisional or calibrated. Performance/resource evidence covers named target hardware/classes and supported platforms, not only one developer machine.
- [ ] Crypto errors are translated at a deliberate service boundary into fixed Core error codes; no secret or low-level sensitive diagnostic crosses into `ArkError` or logs.
- [ ] API documentation does not imply that KDF success is password authentication, authorization, session establishment, or key hierarchy completion.

### F3 — Security Validation integration

- [ ] Owner-specific A-01..A-06 artifacts are ported onto clean current-main branches/PRs without replaying the historical Core foundation commit set.
- [ ] Core security and negative tests compile and execute in the actual current workspace.
- [ ] KDF fuzz targets call the real `ark-crypto` production API entry points. Local reimplementations or simulations of bounds are not counted as API fuzzing.
- [ ] Fuzz evidence records exact commit, command, target, corpus/seed policy, duration, and outcome. No unexecuted fuzzing is described as passing.
- [ ] Adversarial tests are integrated into the current `crates/ark-crypto` layout; no separate root Cargo project or unrelated README removal is imported.
- [ ] KDF threat model, traceability matrix, and independent Crypto review are updated against the code actually proposed/merged, not a superseded implementation.
- [ ] Every one of the 32 security invariants has a traceability entry with owner, enforcement component, test/evidence status, failure behavior, and known limitation. Unimplemented requirements are explicitly marked unimplemented; they are not required to be falsely marked complete at Foundation exit.

### F4 — Trust boundaries and residual risk

- [ ] System trust-boundary model is merged and owner-reviewed.
- [ ] Implemented, accepted, proposed, in-progress, and unknown elements are clearly distinguished.
- [ ] Each major trust boundary has an input/source, threat, enforcement owner, fail-closed behavior, and evidence requirement.
- [ ] Host OS/kernel, privileged administrator, authorized application/plugin, copied plaintext, open-handle, swap/pagefile, crash-dump, secure-erasure, and revocation limitations are explicit.
- [ ] No component is described as enforced merely because an API, type, diagram, or planned crate exists.

### F5 — Human review and CI discipline

- [ ] Each PR has an independent reviewer; the author is not treated as their own reviewer.
- [ ] Crypto, key management, storage formats, identity, policy, broker, filesystem enforcement, and recovery changes receive the required qualified peer review (two reviewers where the team policy calls for it).
- [ ] Final CI evidence belongs to the exact final head SHA. A passing check on an earlier commit does not validate later changes.
- [ ] No unresolved Critical/High security finding remains in the Foundation scope. Lower-severity residual risks are documented with owner and disposition.
- [ ] Where repository rules do not enforce approvals/status checks, the PR record explicitly shows human review and CI completion before merge.
- [ ] No bypass, review dismissal, or policy weakening is used to make a PR mergeable.

### Foundation exit decision

Foundation exit may be declared only after F0–F5 are reviewed together. The decision must include:
- the exact `main` commit SHA;
- merged PR/ADR links;
- independent review records;
- exact CI/check-run conclusions;
- KAT, negative-test, fuzz, and benchmark evidence actually obtained;
- invariant traceability and explicit unimplemented items;
- residual risks and named owners.

**Foundation exit is not product completion or production security certification.** It means the Core + KDF + validation + trust-boundary contracts are coherent enough to begin the next implementation stage under explicit gates.

## 4. Next-stage sequence

### Stage 1 — Cryptographic protocol and key lifecycle

**Purpose:** move beyond KDF primitives only after the KDF boundary is accepted.

**Owner:** Crypto (`ZeroTrace79`), with Systems Engineering and Security Validation review.

**Prerequisites:** F0–F5 Foundation exit; KDF API and IKM/context decisions accepted.

**Required artifacts and gates:**
- ADR for Workspace Master Key generation, key hierarchy, derivation purposes, and wrapper relationships.
- Separate contracts for password-derived KEK, random workspace/package/recovery keys, and any further derived keys.
- XChaCha20-Poly1305 library/API decision, authenticated-data binding, ciphertext framing, and failure semantics.
- Nonce uniqueness strategy under concurrency, crash/restart, restore, clone/fork, and retry. No implementation may assume random nonce collisions or counter persistence away.
- Explicit key ownership, clone/copy limits, zeroization boundaries, and process lifetime.
- Independent review, official vectors, negative tests, and actual fuzzing for parser/primitive boundaries.

**Exit evidence:** accepted ADRs; reviewed production APIs; KAT and negative tests on CI; nonce/key lifecycle analysis; independent Crypto/Security Validation disposition. No storage format is implied by this stage.

### Stage 2 — Encrypted storage format and transactional core

**Purpose:** establish durable, authenticated storage with bounded parsing and crash-safe transaction semantics.

**Owner:** Storage owner — **to be explicitly assigned**; Crypto and Systems Engineering are required partners.

**Prerequisites:** Stage 1 authenticated-encryption and key/wrapper contracts.

**Required artifacts and gates:**
- Versioned on-disk format and parser bounds for length, count, depth, allocation, and compatibility.
- Authenticated headers, manifests, indexes, chunk identity, generation, and all metadata that changes interpretation.
- Random-access/chunked encryption format with unique nonce allocation bound to key and chunk context.
- Write-ahead journal or equivalent transaction protocol, flush ordering, commit boundary, interrupted-write detection, and idempotent startup recovery.
- Concurrency, locking, readers during writes, rename/unlink with open handles, directory enumeration, and consistency semantics.
- Explicit rollback model. AEAD alone is not anti-rollback; if rollback resistance is claimed, define a trusted generation anchor and recovery semantics.
- Fault injection for crash points, power loss simulation where feasible, truncation, reorder, bit-flip, corrupted metadata, and concurrent writers.
- Performance measurements for small files, large sequential reads, random reads, editor save bursts, enumeration, journal growth, and recovery.

**Exit evidence:** accepted format/transaction ADRs; deterministic crash/fault tests; no unauthenticated plaintext success; documented durability contract; independent Storage/Crypto review; performance/capability envelope.

### Stage 3 — Identity, Session, Policy, and Broker contracts

**Purpose:** implement one default-deny decision path for protected operations.

**Owners:** subsystem owners **to be explicitly assigned**; Security Validation and Systems Engineering participate.

**Prerequisites:** Core domain contracts; trust-boundary model; clear operation and resource vocabulary; platform evidence capability plan.

**Required artifacts and gates:**
- Identity evidence bound to a live process instance, lifetime, freshness, and platform evidence source.
- Authentication/session-establishment separate from workspace unlock and separate from authorization.
- Session lifecycle, expiry, device/user/workspace binding, revocation, cache invalidation, and restart safety.
- Deterministic policy evaluation, least privilege, explicit deny precedence, policy versioning, and fail-closed behavior.
- Broker as sole orchestration boundary for identity/session/policy decisions; no UI, path, PID, process-name, runtime-mode, or stale-cache authority.
- Secret-safe audit event schema and explicit behavior when identity, session, policy, or audit dependencies are unavailable.
- Positive and negative tests including stale/ambiguous identity, replay, session expiry/revocation, conflicting rules, cache invalidation, broker outage, and confused-deputy attempts.

**Exit evidence:** accepted interface/policy ADRs; owner-assigned components; default-deny negative tests; traceability; independent security review. A type or mock response is not sufficient proof of OS-backed identity.

### Stage 4 — Windows identity and filesystem-mediation prototype

**Purpose:** prove a real supported Windows mediation path before describing ARK as controlling an editor/runtime filesystem.

**Owner:** Windows Platform owner — **to be explicitly assigned**; Broker/Identity/Filesystem owners participate.

**Prerequisites:** Stage 3 contracts and a chosen mediation mechanism.

**Required artifacts and gates:**
- Process evidence stronger than PID, path, or process name alone, with live-instance and restart/PID-reuse handling.
- A capability matrix covering open/read/write/create/delete/rename/enumerate/flush/truncate/locks/mmap/notifications, alternate paths, symlinks/reparse points, hard links, child processes, handle duplication, and concurrent access.
- Tests against named Windows versions and actual applications in the MVP support matrix.
- Explicit behavior for unsupported operations; never silently claim complete mediation when a path is not intercepted.
- Handle lifetime, revocation, process crash, and in-flight operation semantics.
- Threat model for local unprivileged attacker, same-user process, plugin, and compromised authorized application.

**Exit evidence:** working prototype with demonstrated OS enforcement; repeatable platform tests; capability gaps documented; independent platform/security review. A user-mode watcher or path mapping alone is not evidence of complete mediation.

### Stage 5 — Runtime and MVP access workflows

**Purpose:** connect the trusted backend to Development Mode and Execution Mode without making UI an authority.

**Owners:** Runtime owner — **to be explicitly assigned**; Systems Engineering coordinates; platform, broker, storage, and crypto owners review their boundaries.

**Prerequisites:** Stages 1–4, including durable storage and supported filesystem enforcement.

**Required artifacts and gates:**
- Development Mode limited active working set, authenticated encrypted journal/transaction integration, save/flush durability semantics, and cleanup/recovery.
- Execution Mode explicit resource/directory scope, operation set, duration, step-up requirements, and clear UI risk communication.
- Each protected request mediated by current process evidence, valid session, policy, and broker result.
- Explicit child-process/plugin/extension/handle inheritance semantics.
- Revocation applies to future mediated operations; open handles, mappings, in-flight I/O, and copied plaintext use documented backend behavior, not a universal instant-revocation claim.
- MVP compatibility tests for a real PHP project edited using VS Code and served by Apache, naming exact versions and tested operations.
- End-to-end tests for workspace create/open/unlock/edit/save/execute/close/crash/restart, with integrity and durability assertions.

**Exit evidence:** end-to-end MVP scenario passes on supported Windows baseline; no plaintext on integrity failure; durable-success only after defined commit; authorization and recovery failure paths tested; independent system review.

### Stage 6 — Portable package and recovery integration

**Purpose:** deliver portable encrypted transfer and independently understandable recovery paths on top of accepted key and storage contracts.

**Owners:** Package and Recovery owners — **to be explicitly assigned**; Crypto, Storage, Identity, and Systems Engineering review.

**Prerequisites:** Stage 1 key/wrapper design and Stage 2 authenticated storage/format primitives. Recovery also depends on the device identity/proof-of-possession contract from Stage 3 where Trusted Device is supported.

**Required artifacts and gates:**
- Versioned portable package format with independent random Package Key and package-specific password/KEK wrapper.
- Copy/export of an existing encrypted package must not require unlocking/decrypting the source workspace; package creation/import still authenticates and validates all untrusted inputs.
- Strict bounded parsing, authenticated manifest and metadata, compatibility rules, and rollback/substitution handling.
- Staged import with integrity verification before publication; atomic or recoverable commit semantics.
- Recovery Key: independent random 256-bit per Workspace; safe display/storage guidance and proof-of-possession verification; no raw server-side storage.
- Recovery File: authenticated wrapper/metadata backup, not a standalone independent factor; bind it to Workspace/purpose/version.
- Trusted Device: device-specific private key, proof of possession, key protection, enrollment, rotation, revocation, and lost-device semantics.
- Recovery-route independence, combinations, failure handling, and user-facing consequences documented. No invented escrow/backdoor or unsupported zero-knowledge recovery promise.
- Tests for wrong workspace, modified/truncated/stale package, corrupted wrapper, lost route, interrupted import/rotation, malicious metadata, and rollback.

**Exit evidence:** format/Recovery ADRs accepted; restore/import atomicity and hostile-input tests pass; each supported recovery route independently verified; independent Crypto/Storage/Recovery review.

### Stage 7 — Linux and macOS platform backends

**Purpose:** extend the proven portable contracts to additional operating systems without inheriting unsupported Windows assumptions.

**Owners:** Linux and macOS Platform owners — **to be explicitly assigned**.

**Prerequisites:** stable platform-neutral contracts and a documented Windows capability baseline.

**Required artifacts and gates:**
- Separate OS-specific identity and filesystem capability evidence.
- Platform-specific handling for process lifetime, file descriptors/handles, mappings, locks, notifications, permissions, links, and crash behavior.
- Actual mediation and compatibility tests on named distributions/kernel versions and macOS releases.
- Explicit support matrix and unsupported semantics per platform; no blanket cross-platform claim based on compilation alone.

**Exit evidence:** platform-specific test matrix and real enforcement evidence for each supported operation/application pair; independent platform/security review.

### Stage 8 — Applications, delivery, and release trust

**Purpose:** deliver CLI/desktop workflows and a trustworthy installation/update/release process.

**Owners:** Application and Release owners — **to be explicitly assigned**.

**Prerequisites:** Stable backend/runtime contracts and supported platform capability matrix.

**Required artifacts and gates:**
- Composition-root-only applications; UI can submit intent and display decisions, never grant authority.
- Tauri/Rust/TypeScript capability boundary reviewed; command allowlist and narrow IPC; no secret leakage through frontend logs, analytics, crash reports, or clipboard.
- Signed release artifacts, protected signing keys, dependency/provenance controls, reproducible build strategy where feasible, secure update verification, rollback-safe installation, and recovery from failed upgrades.
- Compatibility matrix, security response process, vulnerability disclosure path, backup/recovery guidance, and release threat model.

**Exit evidence:** independent release/supply-chain review; signed artifacts and update verification tested; install/upgrade/rollback tests; published compatibility and residual-risk statements.

## 5. Dependency graph and critical path

The following is the intended dependency direction, not a promise that all components exist:

```text
                         ┌──────────────────────────────┐
                         │       ark-core (main)        │
                         └──────────────┬───────────────┘
                                        │ shared vocabulary
              ┌─────────────────────────┼─────────────────────────┐
              ▼                         ▼                         ▼
      ark-crypto (KDF → AEAD)      ark-identity             ark-storage
              │                         │                    ▲
              │                         ▼                    │
              │                   ark-session                │
              │                         │                    │
              │                         ▼                    │
              │                    ark-policy                │
              │                         │                    │
              │                         ▼                    │
              └────────────────────► ark-broker               │
                                        │                     │
                          ┌─────────────┴──────────────┐      │
                          ▼                            ▼      │
                    ark-runtime                ark-filesystem  │
                          │                            ▲      │
                          └──────────┬─────────────────┘      │
                                     ▼                        │
                           OS platform backends               │
                                     │                        │
                      ┌──────────────┴────────────┐           │
                      ▼                           ▼           │
                 Applications              package/recovery ──┘
```

Interpretation:
- `ark-core` is a vocabulary foundation and does not become the authorization engine.
- `ark-crypto` provides cryptographic operations; it does not decide policy.
- Storage consumes Crypto for authenticated persistence but does not invent algorithms or nonce lifecycle.
- Identity produces evidence; Session represents bounded state; Policy makes deterministic decisions; Broker coordinates the protected-operation decision path.
- Platform adapters supply evidence/enforcement capabilities; they do not invent product policy.
- Runtime coordinates Development/Execution lifecycle but does not grant access.
- Package and Recovery depend on reviewed Crypto/Storage contracts, and Trusted Device additionally depends on device identity/proof-of-possession design.
- Composition-root applications remain non-authoritative.

### Critical path

1. Close Foundation integration and owner-review gates (E-02/E-03/E-04 plus KDF/security validation integration).
2. Accept Crypto key hierarchy, AEAD, nonce, and key-lifecycle contracts.
3. Specify storage format, authenticated metadata, transaction/journal ordering, and rollback model.
4. Define Identity/Session/Policy/Broker request and failure contracts.
5. Prove actual Windows process evidence and filesystem mediation.
6. Integrate Runtime with the broker, storage, and platform backend.
7. Add package/recovery workflows and additional OS backends; product delivery follows tested backend support.

Parallel work is permitted only where the interface is explicit, owners agree, and independent branches do not assume unreviewed contracts.

## 6. Ownership and review model

| Area | Primary owner at this baseline | Required collaborators / review |
|---|---|---|
| Systems architecture, Core integration, ADR coordination | Elman / Systems Engineering | Crypto and Security Validation owners |
| Crypto KDF, key derivation, future key hierarchy | Saleh / `ZeroTrace79` | Systems Engineering + Ali/Security Validation independent review |
| Threat model, adversarial tests, traceability, fuzz validation | Ali / `alieliteworks-debug` | Crypto and Systems Engineering |
| Storage engine and persistent-format ownership | TBD — explicit assignment required | Crypto, Systems Engineering, Security Validation |
| Identity / Session / Policy / Broker | TBD — explicit assignment required | Platform, Systems Engineering, Security Validation |
| Windows / Linux / macOS backends | TBD per platform | Identity, Filesystem, Broker, Security Validation |
| Runtime / Applications / Release | TBD — explicit assignment required | All affected subsystem owners |

No person should approve their own implementation. The author of a security-sensitive implementation cannot count as its independent reviewer.

## 7. Risk register and decision gates

| Risk | Severity | Gate / mitigation | Owner |
|---|---|---|---|
| Crypto PR branch is behind current main and retains formal changes-requested review | High | Synchronize, answer review findings, fresh review and CI on final head | Crypto |
| KAT/conformance helpers exposed in production API | High | Restrict to test-only/internal scope; preserve KAT coverage | Crypto |
| HKDF raw IKM API permits callers to mistake length for trusted provenance | High | Resolve in E-03, narrow production API, test misuse boundaries | Crypto + Systems Engineering |
| HKDF custom purpose grammar is wider than documented closed purpose set | Medium/High | Decide and enforce closed or formally extensible namespace | Crypto + Systems Engineering |
| Current KDF fuzzing targets simulate checks rather than exercise crypto implementation | High | Connect targets to real `ark-crypto` APIs and record actual runs | Security Validation + Crypto |
| Security Validation branches contain old Foundation history | Medium | Rebuild clean main-based branches; port only relevant owner artifacts | Security Validation |
| Ruleset does not require approvals or status checks | High (governance) | Enforce manually; create separate policy proposal and obtain repository-owner decision | Repository admins / Systems Engineering |
| Storage nonce/transaction/crash semantics are not specified | Critical before Storage implementation | Dedicated ADRs, fault injection, independent review before shipping data path | Storage + Crypto |
| Platform mediation gaps could be mistaken for complete control | Critical before Runtime claims | Per-operation capability matrix and real OS tests | Platform + Filesystem |
| Trusted Device and Recovery File lifecycle remain partly unspecified | High before recovery release | Protocol ADRs, proof-of-possession, revocation/rotation/crash tests | Recovery + Identity + Crypto |
| OS/kernel/admin compromise and authorized plaintext exfiltration | Accepted residual limitation | Communicate threat boundary; do not claim DLP or protection from privileged compromise | Systems Engineering |

Severity here is a planning priority, not a statement that each risk is a currently exploitable vulnerability in the merged Core code.

## 8. Change control and re-planning

- Re-fetch current `main`, issues, PRs, branch heads, review states, and CI before each integration or merge decision.
- Every PR must identify base/head SHAs, scope, owning subsystem, security invariants affected, tests run, and untested assumptions.
- Port code/docs from divergent branches by selecting the relevant owner-specific changes; never replay an old Foundation history wholesale.
- Preserve review history. Do not dismiss stale reviews to make GitHub mergeable; obtain updated review after the code changes.
- Never merge on `mergeable: true` alone. Require synchronized base, resolved review findings, human review, and successful checks on the exact final head.
- If a design conflict appears between accepted ADRs, owner decisions, and current code, stop that integration and record an explicit decision rather than weakening the contract silently.
- Update this roadmap whenever a stage exit gate changes or a critical-path dependency is accepted, deferred, or superseded.

## 9. Explicit non-goals at Foundation exit

Foundation exit does **not** claim:
- complete Workspace Master Key hierarchy or rotation;
- AEAD content encryption or streaming/chunk protocol;
- nonce allocation or anti-rollback;
- encrypted storage, journal, transaction, or crash recovery;
- identity verification, authentication, sessions, policy evaluation, or broker authorization;
- real OS filesystem mediation or process-control guarantees;
- package import/export engine or recovery engine;
- Trusted Device enrollment/revocation or Recovery File protocol;
- complete platform support, secure erase, DLP, or protection against a compromised OS/kernel;
- product-ready GUI, signed installer, or production release.

These are later-stage deliverables with their own owners, ADRs, tests, threat models, and independent reviews.
