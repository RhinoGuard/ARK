# ADR-0005 Technical Review and Decision Record

- **Related ADR:** [ADR-0005: Initial Rust Workspace and Core Domain Foundation](ADR-0005-core-foundation.md)
- **Related implementation:** [PR #2 — Core Foundation](https://github.com/RhinoGuard/ARK/pull/2)
- **Review completed:** 2026-11-19
- **Review status:** Complete — required subsystem-owner confirmations received.
- **Final ADR status:** **Accepted** for the workspace and `ark-core` foundation scope.

> This record documents the technical cross-check and explicit owner feedback. Acceptance is limited to the scope of ADR-0005; it is not a security certification or approval of deferred subsystem implementations.

## 1. Executive result

The ten decisions in ADR-0005 were checked against the final PR #2 implementation. The implementation matches the accepted scope: Cargo workspace baseline, narrow dependency-free `ark-core`, typed domain identifiers, version-domain types, fixed shared error taxonomy, workflow-only RuntimeMode, unsafe-code prohibition, public API documentation policy, and cross-platform CI.

PR #2 was merged to `main` at `bafa677d6b42e7b04b736fcc386cd8e4ea4d9943`. Its final head was `03336335ae5f87dfc93f70ea56f5448265cb1749`; the recorded GitHub Actions checks passed for Ubuntu, Windows, macOS, Rust 1.85 MSRV, and Conventional Commit validation.

## 2. Decision-by-decision implementation check

| # | Decision | Evidence in PR #2 | Assessment |
|---|---|---|---|
| 1 | Cargo workspace, resolver 3, Rust 2024, MSRV 1.85 | Root Cargo workspace manifest | Matches |
| 2 | Materialize only `ark-core` initially | Workspace member list | Matches |
| 3 | Keep Core free of third-party and concrete subsystem implementations | `crates/ark-core/Cargo.toml` and crate documentation | Matches |
| 4 | Seven distinct opaque 16-byte identifiers | `ids.rs` | Matches |
| 5 | Separate version-domain types | `versions.rs` | Matches |
| 6 | Fixed error-code taxonomy and sanitized shared error | `errors.rs` | Matches |
| 7 | RuntimeMode is workflow vocabulary, not authorization | `runtime.rs` | Matches |
| 8 | Forbid unsafe Rust and require public API documentation | Workspace lints and crate attributes | Matches |
| 9 | Cross-platform CI and commit-header validation | Successful PR #2 Actions checks | Matches |
| 10 | Keep crypto, key hierarchy, nonce, storage, identity, policy, and broker decisions outside this ADR | Core scope and dependency boundary | Matches |

### 2.1 Interpretation limits retained

- Typed IDs are domain labels, not credentials, identity proof, capabilities, or authorization.
- RuntimeMode distinguishes workflow modes; it does not isolate processes or enforce access.
- Version types provide type separation and numeric comparison; they do not define serialization, migration, compatibility, or anti-rollback protection.
- ArkError provides a low-detail shared error vocabulary; it does not guarantee that future subsystem logs and diagnostics are sanitized.
- The 16-byte identifier representation is an in-memory API contract for this milestone, not an accepted wire or persistence encoding.

## 3. Crypto subsystem owner confirmation

**Owner:** ZeroTrace79 — Crypto / `ark-crypto`.

**Evidence:** [Issue #15 owner confirmation](https://github.com/RhinoGuard/ARK/issues/15#issuecomment-7017176050), cross-referenced by [PR #2 comment](https://github.com/RhinoGuard/ARK/pull/2#issuecomment-7017190351).

The Crypto owner explicitly confirmed:

1. The dependency direction remains one-way (`ark-crypto -> ark-core` when Core types are needed); `ark-core` remains at the bottom of the dependency graph without cryptographic dependencies or assumptions.
2. The KDF IKM/context and purpose boundary types (`HkdfIkm`, `HkdfContext`, `HkdfKeyPurpose`) and subsystem-specific error types are compatible with the Core foundation. The owner explicitly reserves the higher-level guarantee that callers supply key material from an approved source and never raw passphrases for E-03 / Issue #17.
3. KDF-specific errors remain local to `ark-crypto`; deterministic sanitized mapping at shared subsystem boundaries is to be specified in E-03.
4. The integrated crate follows Rust 2024, MSRV 1.85, workspace lints, and the unsafe-code prohibition.

**Disposition:** Approved / confirmed, with no requested amendments to ADR-0005.

## 4. Security Validation owner confirmation

**Owner:** alieliteworks-debug — Security Validation.

**Evidence:** [PR #2 owner confirmation](https://github.com/RhinoGuard/ARK/pull/2#issuecomment-7012352146).

The Security Validation owner explicitly confirmed:

1. The current public `ark-core` API is sufficient for the present Core testing scope.
2. Test semantics correctly avoid overclaiming: IDs are checked by direct value equality, versions by numeric ordering, RuntimeMode as workflow vocabulary, and error tests as static coarse labels—not as hashing guarantees, anti-rollback, sandboxing, or comprehensive future log sanitization.
3. The narrow Core boundary, `#![forbid(unsafe_code)]`, and absence of crypto, persistence, OS, and GUI dependencies are appropriate. No speculative Core API expansion is requested.

**Disposition:** Approved the technical content of ADR-0005 and PR #2; no amendment requested.

## 5. Final Systems Engineering decision

On 2026-11-19, Systems Engineering reviewed the final PR #2 implementation and the explicit confirmations above.

**Decision: ACCEPT ADR-0005** for the Cargo workspace baseline and initial `ark-core` domain foundation.

Rationale:
- All ten ADR decisions match the final PR #2 implementation.
- PR #2 is merged to `main`.
- The required Crypto and Security Validation owners explicitly confirmed compatibility and raised no amendment request within this ADR's scope.
- The final PR #2 CI checks passed.
- The known semantic limits of Core types remain explicit and are not represented as implemented security controls.

No unresolved dissent or amendment remains for ADR-0005.

## 6. Scope boundaries and follow-up work

Acceptance does **not**:
- approve PR #3 as a whole or certify the cryptographic implementation;
- close E-03 / Issue #17, which still owns the detailed Core-to-Crypto integration contract (typed IKM provenance, approved key-material sources, context/purpose rules, and error mapping);
- accept key hierarchy, AEAD, nonce lifecycle, storage formats, identity, authorization, broker, recovery, runtime, or platform behavior;
- convert the broader proposed crate map into a blanket approval for all subsystem designs.

Those items remain governed by their own issues, reviews, and ADRs.

## 7. Evidence index

- [ADR-0005](ADR-0005-core-foundation.md)
- [PR #2 — merged Core Foundation](https://github.com/RhinoGuard/ARK/pull/2)
- [Security Validation owner confirmation](https://github.com/RhinoGuard/ARK/pull/2#issuecomment-7012352146)
- [Crypto owner confirmation on Issue #15](https://github.com/RhinoGuard/ARK/issues/15#issuecomment-7017176050)
- [Crypto owner confirmation on PR #2](https://github.com/RhinoGuard/ARK/pull/2#issuecomment-7017190351)
