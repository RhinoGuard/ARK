# ADR-0005 Technical Review Record

- **Related ADR:** [ADR-0005: Initial Rust Workspace and Core Domain Foundation](ADR-0005-core-foundation.md)
- **Related implementation:** PR #2 — `feat/rust-core-foundation`
- **Review snapshot:** 2026-11-18
- **Review status:** Technical cross-check prepared; subsystem-owner review and final decision are still pending.
- **ADR status:** **Proposed for team review** — unchanged.

> This document records a static comparison of the proposed decisions with the current PR #2 implementation and the available Crypto and Security Validation work. It is not an approval, security certification, or substitute for explicit owner feedback. No subsystem owner's silence is interpreted as approval.

## 1. Executive result

The current `ark-core` implementation is substantially consistent with the ten decisions stated in ADR-0005. The workspace baseline, Core scope, typed identifiers, version-domain types, fixed error taxonomy, RuntimeMode semantics, unsafe-code policy, and CI checks are present on the PR #2 head.

The review did not identify a reason to expand `ark-core` or to move cryptographic, identity, authorization, persistence, or runtime implementation into it.

Two matters remain open before the ADR can be accepted:

1. The Crypto owner must explicitly confirm the proposed dependency direction and the compatibility boundary between the current KDF API and Core. PR #3 is currently a separate root package based on `main`, not an integrated workspace member.
2. The Security Validation owner must explicitly confirm that the Core-facing tests and security-validation assumptions are compatible with the current public API and do not imply controls that Core does not implement.

The PR #2 CI run for head `03336335ae5f87dfc93f70ea56f5448265cb1749` completed successfully. The recorded jobs passed on Ubuntu, Windows, macOS, and Rust 1.85 MSRV, including formatting, build, Clippy, tests, and commit-message validation.

**Recommendation:** keep ADR-0005 in `Proposed` status until the Crypto and Security Validation owners provide explicit review feedback and any resulting amendments are recorded.

## 2. Decision-by-decision implementation check

| # | Proposed decision | Evidence in PR #2 | Assessment |
|---|---|---|---|
| 1 | Cargo workspace, resolver 3, Rust 2024, MSRV 1.85 | Root `Cargo.toml`: `[workspace]`, `resolver = "3"`, `edition = "2024"`, `rust-version = "1.85"` | **Matches** |
| 2 | Materialize only `ark-core` initially | Root workspace members contain only `crates/ark-core` | **Matches** |
| 3 | Keep Core free of third-party and concrete subsystem implementations | `crates/ark-core/Cargo.toml` has no dependencies; `src/lib.rs` documents the dependency contract | **Matches** |
| 4 | Seven distinct opaque 16-byte identifiers | `ids.rs` defines Workspace, Project, File, Session, Device, Package, and Transaction IDs around private `[u8; 16]` values | **Matches** |
| 5 | Separate version-domain types | `versions.rs` defines Application, Storage, Metadata, IPC, Policy, Recovery, and Package version types | **Matches** |
| 6 | Fixed error-code taxonomy and sanitized shared error | `errors.rs` stores only `ErrorCode` in `ArkError`; Display strings are fixed | **Matches** |
| 7 | RuntimeMode is workflow vocabulary, not authorization | `runtime.rs` explicitly states that the mode does not authorize a process or grant access | **Matches** |
| 8 | Forbid unsafe Rust and require public API documentation | Workspace lints set `unsafe_code = "forbid"` and `missing_docs = "deny"`; Core inherits workspace lints and also uses `#![forbid(unsafe_code)]` | **Matches** |
| 9 | Cross-platform CI and commit-header validation | PR #2 Actions run has successful Ubuntu, Windows, macOS, MSRV, and commit-message jobs | **Matches** |
| 10 | Keep crypto, key hierarchy, nonce, storage, identity, policy, and broker decisions outside this ADR | Core has no crypto, storage, identity, or authorization implementation or dependency | **Matches** |

### 2.1 Important interpretation boundaries

The implementation matches the ADR only when its types are interpreted at the level the ADR specifies:

- `WorkspaceId`, `FileId`, and the other IDs are labels. Their constructors do not establish identity, ownership, or permission.
- `RuntimeMode` distinguishes workflow modes. It does not isolate processes or enforce access.
- Version types provide type separation and numeric comparison. They do not define serialization, migration, compatibility, or anti-rollback protection.
- `ArkError` provides a low-detail shared error vocabulary. It does not guarantee that subsystem-specific logs, diagnostics, or error chains are sanitized.
- The 16-byte identifier representation is an in-memory API contract in this milestone, not an accepted wire or persistence encoding.

These limitations are already reflected in the ADR and implementation documentation. They should remain explicit in any acceptance record.

## 3. Crypto compatibility assessment

### 3.1 Core boundary

The proposed crate map in `docs/architecture/CRATE_BOUNDARIES_AND_DEPENDENCIES.md` shows `ark-crypto` depending on `ark-core`, with no reverse dependency. This direction is consistent with Core remaining the bottom of the dependency graph.

The current PR #3 does not yet demonstrate that integration: it is based on `main` and introduces a root-level standalone `ark-crypto` package manifest. PR #2 instead makes the repository root a Cargo workspace and owns the root `Cargo.toml`. These two branch structures cannot simply be merged as-is. This is an integration/branch-alignment issue, not evidence that the Core domain API is unsuitable.

### 3.2 Concrete KDF API boundary

The current PR #3 KDF API exposes:

- `derive_kek(password: &[u8], salt: &[u8], config: Argon2idConfig)`
- `derive_subkey(input_key_material: &[u8], salt: Option<&[u8]>, context: &[u8])`

Both return zeroizing 32-byte output wrapped in their own KDF-specific error types.

Compatibility observations:

1. The API does not currently consume Core identifiers or Core version types. No conflict with those types is visible in the current KDF slice.
2. `derive_subkey` accepts an untyped byte slice for IKM. That leaves the source/purpose contract unresolved at this boundary. The proposed typed-IKM contract should be reviewed by the Crypto owner together with the Systems/Core owner; this ADR should not invent that contract.
3. `Argon2Error` and `HkdfError` are subsystem-specific error enums with static messages. This does not inherently conflict with `ArkError`; the owners should confirm whether and where errors are mapped when crossing a shared subsystem boundary.
4. The PR #3 package does not currently declare itself as a member of the PR #2 workspace. The integrated package must preserve the workspace's edition, MSRV, lint, and unsafe-code policy.
5. The current Core crate correctly contains no KDF, key, or crypto-provider abstraction. No Crypto implementation should be moved into `ark-core` to solve the integration issue.

### 3.3 Crypto owner confirmation required

The Crypto owner should explicitly confirm:

- the one-way dependency direction from `ark-crypto` to `ark-core`, if Core types are needed;
- whether the current byte-slice IKM API is temporary or the intended public boundary, and how it will align with the typed-IKM contract;
- how KDF-specific errors are handled at subsystem boundaries;
- that the final integrated crate will comply with the workspace edition, MSRV, lint, and unsafe-code settings.

No cryptographic primitive, parameter policy, key hierarchy, or KDF API change is accepted by this review record.

## 4. Security Validation compatibility assessment

### 4.1 Current Core API usage

The Security Validation work on `feat/security-testing-framework` imports the current public Core modules:

- `ark_core::errors::{ArkError, ErrorCode}`
- `ark_core::ids::{WorkspaceId, ProjectId, FileId, SessionId, DeviceId, PackageId, TransactionId}`
- `ark_core::runtime::RuntimeMode`
- `ark_core::versions::{ApplicationVersion, StorageFormatVersion, MetadataSchemaVersion, IpcProtocolVersion, PolicySchemaVersion, RecoveryFormatVersion, PackageFormatVersion}`

The tests use APIs present in PR #2:

- ID construction and extraction through `from_bytes`, `as_bytes`, and `into_bytes`;
- error construction and inspection through `ArkError::new` and `ArkError::code`;
- RuntimeMode's `Development` and `Execution` variants;
- version constructors, accessors, and numeric ordering.

No missing Core API required by the currently inspected Core security tests was identified.

### 4.2 Security meaning of the tests

The current Core tests describe the API at the correct level when they assert:

- direct equality/inequality of identifier values, not collision-free hashing;
- numeric version ordering, not anti-rollback enforcement;
- distinct RuntimeMode variants, not an authorization boundary;
- fixed error classification text, not comprehensive sanitization of future subsystem diagnostics.

The current negative Core tests compare identifier values directly and do not rely on distinct `DefaultHasher` outputs.

### 4.3 Security Validation owner confirmation required

The Security Validation owner should explicitly confirm:

- the current public API is sufficient for the present Core test scope;
- the test names, comments, and invariant mapping do not claim authorization, anti-rollback, runtime isolation, or broad error sanitization that Core does not implement;
- no Core API expansion is being requested merely to support a future subsystem or a deferred test.

The current evidence supports keeping Core narrow. It does not establish that future crypto, storage, identity, policy, broker, or runtime security controls are implemented.

## 5. Review feedback and decision status

At the time of this review snapshot:

- PR #2 is open and unmerged.
- PR #2 has one recorded review from the Systems Engineering owner, with state `COMMENTED`; it is not an approval from the Crypto or Security Validation owners.
- No Crypto or Security Validation owner review is recorded on PR #2.
- Issue #15 has no owner-feedback comments recorded.
- PR #3 is open and unmerged; its current branch structure is not integrated with the PR #2 workspace.

Therefore, the acceptance criteria requiring explicit owner feedback are not yet satisfied. Silence, authorship of a dependent branch, or an unrelated review comment must not be treated as approval.

## 6. Open questions to resolve before acceptance

| Owner | Required explicit response |
|---|---|
| Crypto | Confirm the Core/Crypto dependency boundary and API compatibility points in Section 3, or identify concrete amendments. |
| Security Validation | Confirm the Core test/API compatibility points in Section 4, or identify concrete amendments. |
| Systems Engineering | Review the owner feedback, decide whether ADR text needs amendment, and record the final decision and participants. |

For every objection or amendment, record:
1. the exact ADR decision or API reference affected;
2. the objection or requested change;
3. the security/compatibility rationale;
4. the impact on dependent work;
5. the final resolution and the owner who agreed to it.

## 7. Final decision gate

ADR-0005 must remain **Proposed** until the required subsystem owners have explicitly reviewed it and the Systems Engineering owner has recorded the decision.

If the owners agree without amendments, update the ADR status to **Accepted** and add a dated decision record listing the participants and the reviewed PR head.

If an owner requests a change, update the ADR with the rationale and compatibility impact before acceptance. If a material objection remains unresolved, keep the ADR Proposed and record the dissent; do not force an acceptance.

This record does not itself accept ADR-0005, merge PR #2, or approve PR #3.
