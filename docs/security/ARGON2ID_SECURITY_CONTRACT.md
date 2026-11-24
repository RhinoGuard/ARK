# ARK Argon2id Contract and Calibration

- Scope: S-02 / Issue #5, S-03 / Issue #6, and CCB-03 / CCB-04 under Issue #29.
- Status: implementation contract and benchmark harness are proposed for review; not an independent security approval.
- Implementation: `crates/ark-crypto/src/kdf/argon2id.rs`.
- Calibration harness: `crates/ark-crypto/benches/argon2id_calibration.rs`.

## 1. Scope and boundary

`ark-crypto` is a deterministic, platform-independent KDF primitive. It validates caller-supplied password bytes, salt bytes, and Argon2id cost parameters and derives a 32-byte zeroizing KEK. It does not own workspace creation, credential persistence, authentication throttling, or OS integration.

The crate has no production caller/service in the current workspace. The current workspace consists of `ark-core` and `ark-crypto`; no workspace credential lifecycle, session, broker, or storage crate exists. Therefore this document assigns responsibilities without pretending those future components are implemented.

## 2. Password and output contract (S-02)

- Password input is opaque bytes. The caller/UI owns text encoding (normally UTF-8), normalization, and any canonicalization. The KDF does not normalize or trim.
- Empty password input is rejected.
- Passwords longer than `MAX_PASSWORD_LEN = 1024` bytes are rejected before Argon2 memory allocation.
- Borrowed password bytes remain caller-owned. The caller should keep them in a zeroizing owner and clear them when their lifecycle permits; the KDF cannot scrub borrowed memory.
- Argon2id v1.3 returns exactly `KEY_LEN = 32` bytes wrapped in `Zeroizing<[u8; 32]>`.
- The result is a KEK, not a general-purpose file key. Do not log, format, persist in plaintext, or reuse it for unrelated purposes. Use the separately specified HKDF purpose separation for subkeys.
- Errors are fieldless/static and do not carry passwords, salts, derived output, or OS error details.

## 3. Salt ownership and lifecycle (CCB-03)

### 3.1 Ownership decision

The production salt-generation owner is the **higher-level credential lifecycle component** that creates a new password-derived wrapper (future workspace/package credential creation and password-rotation flow). It is not `ark-crypto`.

The split is intentional:

1. On creation or password re-keying, the credential lifecycle component requests fresh salt bytes from the platform's approved OS CSPRNG abstraction.
2. It requests 32 bytes (the recommended default), validates them through `Argon2Salt::from_slice`, and aborts creation if entropy acquisition or validation fails.
3. It persists the exact salt and exact Argon2id parameters with the corresponding wrapped-key record.
4. On unlock/verification, it loads and validates the persisted salt and parameters. It MUST NOT generate a replacement salt.
5. The KDF receives the caller-supplied salt and derives the KEK. It does not know or claim how the salt was generated.

### 3.2 RNG source and dependency boundary

The required randomness source is the operating system CSPRNG through a reviewed Rust/platform abstraction. The concrete dependency and feature set are intentionally selected by the future credential-lifecycle/platform owner when that boundary exists. The current architecture keeps OS APIs and platform dependencies out of `ark-crypto`; adding an RNG dependency to this deterministic primitive crate would cross that boundary without a current caller.

No current code path in this repository generates a production salt. No fixed, timestamp-derived, PID-derived, zero-filled, reused, or caller-low-entropy salt is an acceptable fallback. Any entropy-source failure must abort credential creation.

The implementation work is tracked in [Issue #31](https://github.com/RhinoGuard/ARK/issues/31). Until that owner exists and the OS CSPRNG integration is reviewed and tested, production salt generation is a caller obligation, not an implemented ARK capability.

### 3.3 Salt validation contract

- Minimum accepted length: 16 bytes.
- Recommended generation length: 32 bytes.
- Maximum accepted length: 64 bytes.
- Empty, shorter-than-16, or longer-than-64 inputs fail with `Argon2Error::InvalidSaltLength`.
- An all-zero buffer fails with `Argon2Error::DegenerateSalt`. This catches common uninitialized-buffer mistakes; it does not establish randomness or entropy.
- Salt is public metadata, not a password substitute or secret.
- Each independently created password-derived wrapper must have a fresh salt. A random 32-byte salt makes accidental collisions negligible but does not strengthen a weak password.
- Caller-supplied salts are permitted because persisted records and deterministic tests need to supply exact bytes. Validation proves only shape/non-degeneracy, not CSPRNG provenance.

The public `derive_kek` API documentation explicitly says that the function does not generate randomness and that callers must supply either a freshly generated creation-time salt or the exact stored salt during unlock.

## 4. Argon2id cost policy

The current enforced policy is:

| Parameter | Minimum | Default | Maximum |
|---|---:|---:|---:|
| Memory | 16 MiB | 64 MiB | 256 MiB |
| Iterations | 1 | 3 | 10 |
| Lanes | 1 | 1 | 8 |
| Output | 32 bytes | 32 bytes | 32 bytes |

The API validates the policy even if a caller constructs `Argon2idConfig` directly. The test-only RFC 9106 KAT helper is isolated from the production API and retains a hard upper resource ceiling.

Per-call caps do not bound aggregate memory use. A caller that permits `K` concurrent derivations may incur roughly `K × configured Argon2 memory` for the Argon2 memory blocks alone, plus process/runtime overhead. The future credential/session boundary must apply rate limits and a bounded concurrency/admission-control policy before invoking the KDF.

These limits are safety bounds, not a universal performance claim or a promise that every device can comfortably run the default.

## 5. Calibration evidence and reproducibility (CCB-04)

A standalone benchmark harness is registered as:

```text
cargo bench -p ark-crypto --bench argon2id_calibration
```

Optional controls:

```text
ARK_BENCH_WARMUPS=2
ARK_BENCH_SAMPLES=9
```

The harness runs six profiles: 16 MiB / 1 / 1; 32 MiB / 2 / 1; 64 MiB / 3 / 1; 64 MiB / 3 / 2; 128 MiB / 3 / 2; and 256 MiB / 4 / 4. It emits CSV-compatible mean, median, p95, minimum, and maximum wall-clock latency for each profile. It also reports the configured Argon2 memory-block allocation in MiB.

A manually dispatched GitHub Actions workflow, `.github/workflows/argon2id-calibration.yml`, runs the harness on Ubuntu, Windows, and macOS hosted runners and uploads raw console output. The exact workflow run, commit SHA, runner image, and artifacts must be attached to the issue/PR handoff after the workflow is run.

**Measurement limitations:** configured Argon2 memory is not total process RSS or measured peak system memory. Hosted-runner labels do not identify a stable CPU/RAM profile. The harness output is not yet collected in this PR, and no new measured results are claimed here. A separate follow-up [Issue #34](https://github.com/RhinoGuard/ARK/issues/34) requires retained cross-platform artifacts, actual peak-memory measurement, and representative constrained hardware before the parameter recommendation can be called calibrated.

Historical single-workstation figures previously recorded in the older PR #3 documentation are not treated as multi-platform evidence and are not repeated as newly verified measurements.

## 6. Error behavior and diagnostics

Invalid password length, salt length, degenerate salt, and invalid cost parameters have deterministic fieldless error variants. Underlying derivation failure maps to a fixed `DerivationFailed` classification. No error includes password, salt, output, or low-level sensitive data.

There is no entropy acquisition inside this crate, so there is no RNG error variant here. The future lifecycle owner must map OS entropy failures to a sanitized creation failure and must never fall back to weak or deterministic bytes.

## 7. Security properties and non-claims

This contract supports:

- bounded KDF inputs and per-invocation resource use;
- deterministic validation and secret-safe KDF errors;
- explicit salt generation responsibility and persisted-salt reuse semantics;
- zeroizing derived output.

It does not implement or claim:

- an OS CSPRNG call or secure salt generation in the current repository;
- password database storage or credential creation;
- login throttling or global KDF concurrency control;
- authenticated KDF metadata, key wrapping, AEAD, nonce lifecycle, storage, or recovery;
- cross-platform performance calibration until the workflow artifacts are collected.

Relevant invariants: ARK-SEC-013, ARK-SEC-016, ARK-SEC-018, ARK-SEC-027, ARK-SEC-028, and ARK-SEC-032.

## 8. Review gates

- [ ] Crypto owner confirms the salt ownership/API boundary.
- [ ] Independent security reviewer approves the CSPRNG ownership and lifecycle contract.
- [ ] The calibration workflow is run and its artifacts reviewed.
- [ ] A representative constrained hardware profile and peak-memory measurements are added.
- [ ] Future credential lifecycle implementation closes Issue #31.
- [ ] Future storage owner approves authenticated KDF metadata and migration requirements tracked in Issue #33.
