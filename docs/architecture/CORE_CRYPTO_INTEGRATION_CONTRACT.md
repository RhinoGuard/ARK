# Core-to-Crypto Integration Contract

**Status:** PROPOSED — requires Crypto, Systems Engineering, and Security Validation owner review.  
**Issue:** [#17 — E-03](https://github.com/RhinoGuard/ARK/issues/17)  
**Baseline:** Accepted ADR-0005 and merged PR #2; current KDF source is tracked through PR #3 and the stacked Security Validation candidate PR #26. PR #3 and PR #26 remain unmerged.  
**Last checked:** 2026-11-20.

> This is a subsystem boundary contract, not an implementation claim. PR #2 is merged. PR #3 remains open and has unresolved review/API findings. The contract does not make AEAD, key wrapping, key hierarchy, storage, or recovery available.

## 1. Purpose and scope

This document defines what crosses the boundary between the stable Core domain vocabulary and the cryptography subsystem. It records responsibilities, dependency direction, data ownership, secret handling, error translation, compatibility boundaries, and decisions that must be resolved before callers depend on the KDF API.

The current deliverable is KDF-only: Argon2id password-based key derivation and HKDF-SHA-256 subkey derivation. A successful KDF call is not an authorization decision, a credential-verification result, or proof that a key has been safely stored.

## 2. Normative boundary principles

1. **Core does not depend on Crypto.** `ark-core` must remain free of concrete crypto crates, passwords, key bytes, salts, nonces, KDF configuration, and crypto-specific lifecycle state.
2. **Crypto may consume Core vocabulary only when necessary.** The dependency direction is one-way: `ark-crypto -> ark-core` if a concrete integration requires Core-owned domain types. No reverse dependency or cycle is permitted. The current PR #3 crate does not need to import `ark-core` for its KDF primitives.
3. **Length is not provenance.** A minimum-length check does not prove entropy, randomness, or that input key material came from an approved source.
4. **Password and IKM are different security domains.** Human passwords go to the password KDF. Raw passwords must not be passed directly to HKDF as IKM.
5. **Core errors are deliberately coarse.** Crypto keeps detailed internal error classifications private and translates only sanitized classifications across subsystem boundaries.
6. **No authority leaks through cryptographic success.** Successful derivation or successful key unwrap does not create a session, grant access, or authorize a caller.
7. **Do not overstate memory erasure.** Zeroizing wrappers reduce exposure of owned buffers; they do not prove erasure from registers, compiler-created copies, swap, crash dumps, or OS-managed caches.

## 3. Responsibility matrix

| Responsibility | Owning boundary | Contract |
|---|---|---|
| Domain IDs, version-domain newtypes, fixed Core error codes | `ark-core` / Systems Engineering | Stable domain vocabulary only. No crypto algorithm, key, credential, or authorization semantics. |
| Password validation and Argon2id derivation | `ark-crypto` / Crypto | Validate password and cost bounds before expensive work; use the selected Argon2id version; return owned, zeroizing derived key output. Never log or persist passwords. |
| Argon2 salt representation and validation | `ark-crypto` / Crypto | Validate length and documented format. A validator does not generate a salt or prove uniqueness. |
| CSPRNG source and platform entropy availability | Platform / OS integration boundary (owner/API still open) | Supply cryptographically secure random bytes. Do not substitute timestamps, process IDs, UI-generated values, or general-purpose PRNG output. |
| HKDF input-key-material acceptance | `ark-crypto` plus the upstream key-material producer | Accept only material with documented high-entropy provenance. A 16-byte minimum is only a truncation guard, not an entropy proof. |
| HKDF purpose/context policy | `ark-crypto` / Crypto with Systems Engineering review | Define a canonical, versioned, collision-resistant purpose namespace. Do not silently accept undocumented custom purposes. |
| Crypto-specific errors and diagnostics | `ark-crypto` / Crypto | Keep detailed source errors internal; sanitize logs and externally visible diagnostics. |
| Cross-subsystem error classification | Owning service boundary using `ark-core::errors` | Convert to a fixed `ErrorCode`; never put a password, key, salt, context payload, path, or low-level error string into `ArkError`. |
| Authentication, authorization, sessions, recovery decisions | Future Identity / Session / Policy / Broker / Recovery owners | KDF success is evidence about derived bytes only. It is not identity proof, authorization, session establishment, or a recovery grant. |
| Independent tests, threat traceability, adversarial/fuzz validation | Security Validation / `alieliteworks-debug` | Exercise real public APIs and failure paths. Simulated validation logic must not be described as API fuzzing. |

## 4. Compile-time dependency graph

The allowed direction is:

```text
ark-crypto  ───────► ark-core   (optional; only if Crypto genuinely consumes Core types)
ark-core              (no dependency on ark-crypto or concrete crypto libraries)
higher-level service  ───────► ark-core
higher-level service  ───────► ark-crypto
```

The arrow means “may compile against.” It does not describe runtime authority or imply that Core should contain a generic crypto-provider abstraction.

The KDF implementation can remain independent of Core today: the current KDF entry points operate on password/IKM bytes, validated KDF wrappers, contexts, and crypto-specific error types. Do not add an `ark-core` dependency merely to make the graph look connected.

## 5. Data contract and ownership

### 5.1 Password input and Argon2id

- Password bytes are caller-owned input borrowed by the KDF for the duration of the call. The caller owns acquisition, lifetime minimization, and disposal of its input buffer.
- Password bytes are not a general-purpose key, not an HKDF IKM, and not a persisted credential representation.
- The current implementation declares a 1–1024 byte input bound. The bound must be enforced before expensive Argon2 work.
- Argon2id version, memory, iteration, parallelism, output length, and salt are compatibility-relevant parameters. Any values persisted in a future wrapper/package/workspace format must be validated against hard resource ceilings before allocation.
- The current default (64 MiB, 3 iterations, 1 lane) and hard bounds are implementation policy candidates, not yet a cross-platform performance calibration result.

### 5.2 Argon2 salt

- The current `Argon2Salt` validates a supplied slice; `from_slice` does not generate randomness.
- The caller must provide fresh, independently generated salt bytes from the approved OS CSPRNG for each password-derived wrapper/domain as specified by the owning format.
- The current implementation enforces 16–64 bytes and rejects all-zero values. The all-zero check is a defensive input sanity rule; it is not a substitute for entropy or uniqueness.
- Salt bytes are public derivation metadata, not a secret. Do not use the salt as a password, key, or authorization token.
- Ownership of the random source/API and the exact salt uniqueness domain must be decided before a persistent key-wrapper format is finalized.

### 5.3 HKDF IKM

- HKDF input must be high-entropy cryptographic key material, such as a key produced by the approved key-generation/KDF path or the OS CSPRNG. It must not be a raw human password.
- The current `HkdfIkm` wrapper checks a minimum of 16 bytes and redacts its `Debug` representation. This wrapper is borrowed and does not own or zeroize the source bytes.
- A 16-byte length floor cannot establish provenance or entropy. The upstream component creating the key remains responsible for the origin, scope, lifetime, and zeroization of the source material.
- The currently documented recommended size is 32 bytes. The owning Crypto design must decide whether 16-byte material remains acceptable for every purpose or whether production derivation should require a stronger typed source.

### 5.4 HKDF salt, context, and output

- HKDF salt is distinct from the Argon2 password salt. It is optional in the current API; omission follows RFC 5869's zero-salt default. This default does not make low-entropy IKM safe.
- `ARK/v1/<purpose>` is the current context namespace. The exact accepted purpose set and whether extension contexts are permitted remain an explicit decision (see Section 9).
- The current KDF returns a 32-byte output in a zeroizing owner. Callers should transfer ownership where possible, minimize copies, and avoid storing raw derived key bytes.
- The HKDF output is a derived key, not a permission grant, identity, or stable user identifier.

## 6. Error boundary and proposed mapping

`ark-core::ArkError` intentionally contains only a fixed `ErrorCode`. Crypto-specific errors such as invalid salt, invalid cost parameters, too-short IKM, invalid context, and primitive failure should remain in `ark-crypto` until a higher-level service translates them.

The following mapping is a **proposal for owner review**, not an implemented conversion function:

| Condition at the caller/service boundary | Candidate Core code | Notes |
|---|---|---|
| Caller supplied an invalid password length, salt length, KDF parameter, IKM, or context | `InvalidArgument` | Do not include raw input or detailed crypto diagnostics. |
| Configured KDF cost exceeds a caller/service resource budget | `ResourceExhausted` | Distinguish policy/resource rejection from an internal primitive failure. |
| Required OS entropy or crypto/platform capability is unavailable | `Unavailable` | Do not fall back to a non-CSPRNG source. |
| Internal KDF primitive failure with valid bounded inputs | `Internal` | Keep the detailed cause local to Crypto diagnostics. |
| Key-wrapper authentication fails during a future unlock operation | `Unauthenticated` (candidate) | Avoid distinguishing wrong credential from wrapper/tag corruption in externally observable detail unless a reviewed protocol explicitly requires it. KDF derivation alone does not validate a password. |
| Authenticated content/metadata integrity check fails in a future storage operation | `IntegrityFailure` | Storage/Crypto boundary owns the exact behavior; no plaintext success. |
| Caller is not permitted to request an operation | `AccessDenied` | This is a Policy/Broker decision, never a KDF result. |

Never convert an OS error string, dependency error, credential bytes, key bytes, or user-controlled context into Core error text. Never place secrets in telemetry, panic messages, test output, or issue artifacts.

## 7. Secret lifecycle responsibilities

1. Password acquisition and the caller's input buffer remain caller responsibilities; Crypto must not retain or log the input.
2. Derived key outputs should use the existing zeroizing owner and be held only for the duration required by the caller.
3. Borrowed wrappers such as `HkdfIkm` validate shape and reduce accidental misuse; they do not assume ownership and cannot scrub the underlying buffer.
4. Any future owned key type must define construction source, allowed purposes, clone/copy behavior, drop/zeroization behavior, and whether it may cross process or persistence boundaries.
5. Crypto errors and debug formatting must remain secret-safe. Core's fixed error code does not automatically sanitize private Crypto diagnostics.
6. A future key-wrap or storage design must define key rotation, nonce lifecycle, crash behavior, and persistence separately; none is implied by the current KDF types.

## 8. Versioning and compatibility

- The `ARK/v1/` prefix is a KDF context namespace. It is not a complete serialized format version, migration scheme, or anti-rollback mechanism.
- A future persisted wrapper must record enough non-secret metadata to reproduce derivation safely: algorithm identifier/version, validated cost parameters, salt, and wrapper/format version. The precise encoding belongs to the owning format ADR, not `ark-core`.
- Changing KDF algorithm version, cost parameters, context encoding, or output interpretation can make existing wrapped keys unrecoverable. Such changes require explicit compatibility/migration design and tests.
- Core version newtypes do not provide parsing, serialization, compatibility negotiation, migration, or trusted monotonic state.
- No silent fallback to weaker algorithms, cost parameters, context grammar, or key-source policy is permitted.

## 9. Decisions required before the boundary is frozen

| ID | Open decision | Recommended direction to evaluate | Decision owner(s) |
|---|---|---|---|
| CCB-01 | Should production HKDF derivation accept raw byte slices, or only a validated/provenance-bearing IKM type? | Prefer a single production entry point that makes approved key-material provenance explicit. Do not claim a length wrapper proves entropy. | Crypto (`ZeroTrace79`), Systems Engineering |
| CCB-02 | Is the HKDF purpose namespace closed or extensible? | Prefer the closed `HkdfKeyPurpose` set for production entry points; if extensions are required, define a registry/ownership and collision policy. | Crypto, Systems Engineering, Security Validation |
| CCB-03 | Who generates Argon2 salt and HKDF random material, and where is the CSPRNG boundary? | Use an approved OS CSPRNG provider; never invent a deterministic fallback. Keep the API decision separate from the Core crate. | Crypto + future Platform owner |
| CCB-04 | What parameter profiles are supported on Windows, Linux, and macOS? | Keep current defaults provisional until reproducible benchmarking establishes interactive latency and resource budgets on named target classes. | Crypto, Systems Engineering |
| CCB-05 | Which Crypto errors map to Core codes at which service boundary? | Adopt a fixed mapping table and generic externally visible result for wrapper authentication failure; retain private internal diagnostics. | Crypto, Systems Engineering, Security Validation |
| CCB-06 | Which key wrapper/source types may become HKDF IKM? | Define approved source classes (password-derived KEK, random workspace/package/recovery key, or explicitly reviewed derived key) before key hierarchy integration. | Crypto |
| CCB-07 | How are KDF metadata and future parameter migrations versioned? | Put algorithm/parameters in an authenticated, versioned wrapper/format ADR; do not serialize them in Core. | Crypto + Storage/Package/Recovery owners |
| CCB-08 | How are test-vector helpers kept out of the production API? | Keep RFC KAT-only escape hatches private/test-scoped while retaining official vectors in the test suite. | Crypto + Security Validation |

## 10. Security Validation requirements

Before a KDF API is treated as an accepted cross-subsystem contract, Security Validation should verify:

- production callers cannot pass passwords through the HKDF IKM path by accident;
- invalid lengths and resource bounds fail closed before expensive allocation;
- all KDF context/purpose variants follow the accepted namespace policy;
- error conversion does not expose secrets or low-level implementation details;
- KAT-only helpers are absent from the normal production API;
- official RFC vectors and negative tests run in CI;
- fuzz targets call the actual `ark-crypto` public APIs rather than reimplementing their checks;
- test and benchmark evidence identifies the exact commit, command, target, duration, and result.

A passing test suite proves only the exercised properties. It does not prove entropy provenance, key lifecycle correctness, or complete platform compatibility.

## 11. Explicit non-goals

This contract does not implement or approve:
- XChaCha20-Poly1305 encryption/decryption API details;
- key wrapping/unwrapping or the Workspace Master Key hierarchy;
- nonce allocation, crash-safe uniqueness, or anti-rollback;
- encrypted storage formats, WAL, snapshots, or transactions;
- workspace unlock/session establishment/authorization;
- Recovery Key, Recovery File, Trusted Device, or package formats;
- a generic crypto-provider trait in `ark-core`.

Those require owner-specific ADRs and independent review before implementation.


## 12. Current implementation reconciliation (2026-11-20)

This section records a source-and-CI snapshot; it does not convert proposed directions into owner-approved design decisions. The reviewed candidate is PR #26 head `27bc2e61a5e136aeb1febf147526fc2bc82b7d7c`, stacked on `fix/crypto-kdf-production-api-boundary` at `552041395ee9097902eb6a7d1c934ed55b57a99f`. The candidate is not merged to `main`; PR #3 also remains open at its older head.

### 12.1 CCB decision status against the current candidate

| Decision | Current source evidence | Current disposition |
|---|---|---|
| CCB-01 — IKM acceptance and provenance | Public production derivation is `derive_purpose_subkey(HkdfIkm, salt, HkdfKeyPurpose)`; raw `derive_subkey` is private. However, `HkdfIkm::from_slice` accepts arbitrary bytes of 16 bytes or longer, and `from_key_array` accepts any caller-provided 32-byte array. Neither constructor proves a source or entropy provenance. | **Partially mitigated; owner decision open.** The raw internal derivation route is closed, but the API does not technically prevent a caller from wrapping raw password bytes of sufficient length. A producer/source-typed key-material design or explicit call-site provenance contract remains necessary. |
| CCB-02 — HKDF purpose namespace | `HkdfKeyPurpose` defines six canonical `ARK/v1/<purpose>` values; `HkdfContext::parse` rejects unregistered and malformed values; production derivation takes the enum. | **Enforced in the current API candidate.** Whether the six-purpose set is the accepted long-term namespace and who may extend it still requires Crypto/Systems/Security Validation review. |
| CCB-03 — CSPRNG and salt/key generation ownership | `Argon2Salt::from_slice` validates caller-provided bytes; it does not generate them. No CSPRNG provider or random-key generation API is supplied by the KDF crate. | **Open / not implemented.** Platform owner, entropy availability behavior, salt uniqueness domain and random key generation contract remain undecided. |
| CCB-04 — Windows/Linux/macOS parameter profiles | Production defaults are 64 MiB, 3 iterations, 1 lane; hard bounds are 16–256 MiB, 1–10 iterations and 1–8 lanes. CI confirms functional tests on Ubuntu, Windows, macOS and Rust 1.85, not latency or memory-performance calibration. | **Bounds implemented; calibration open.** CI success is not a performance benchmark or supported-device profile approval. |
| CCB-05 — Crypto-to-Core error mapping | `Argon2Error` and `HkdfError` remain Crypto-specific sanitized types. The Core mapping table in Section 6 remains a proposal; no approved service-level conversion contract is included in the current KDF primitive. | **Proposed only; owner decision open.** The service boundary and uniform external behavior for future wrapper authentication failures must be approved before integration. |
| CCB-06 — Approved IKM source classes | Current `HkdfIkm` is a borrowed byte-slice wrapper with a minimum-length check; it has no source enum or producer-restricted constructor. | **Open / not implemented.** Password-derived KEK, random Workspace/Package/Recovery keys and any further derived key sources need explicit owner policy and call-site design. |
| CCB-07 — Persisted KDF metadata and migration | The current deliverable is KDF primitives. No authenticated key-wrapper, persisted KDF metadata record, migration parser or compatibility negotiation is implemented by this candidate. | **Deferred / not implemented.** Requires a format ADR and owning Storage/Package/Recovery decisions. |
| CCB-08 — KAT helper isolation | RFC 9106/RFC 5869 helpers are compiled only under `cfg(test)`, are imported inside the private KDF test module, and are absent from the normal production re-export surface. | **Implemented and CI-validated for this candidate.** Official KAT and KDF test targets passed the exact-head CI run below. |

### 12.2 Exact validation evidence

GitHub Actions [ARK CI run #32](https://github.com/RhinoGuard/ARK/actions/runs/44621678450) completed with conclusion **success** on candidate head `27bc2e61a5e136aeb1febf147526fc2bc82b7d7c`. All five jobs passed: Ubuntu, Windows, macOS, minimum supported Rust 1.85, and Conventional Commit validation. The Rust jobs completed formatting, workspace build, Clippy and tests.

The executed test output included 10 Core tests, 6 KDF unit/KAT tests, 8 KDF integration tests, and 8 adversarial negative tests, all passing. This validates only the code paths exercised by those tests. It does not validate the standalone fuzz workspace, dependency advisories, performance calibration, IKM provenance, CSPRNG generation, or future storage/recovery formats.

### 12.3 Contract implications and remaining acceptance gates

1. **Do not state that the current API prevents raw passwords from reaching HKDF.** The normative rule in Section 2 remains mandatory, but the current `HkdfIkm` constructors cannot enforce provenance. This is a real residual API-policy gap for CCB-01/06 and must be resolved with Crypto owner input before downstream services rely on it.
2. The six-purpose API and test-only KAT boundary are now implemented in the staged candidate, but PR #26 is stacked on an unmerged Crypto branch. The exact accepted Crypto base and final source SHA must be settled before the candidate can be treated as the repository's integrated contract.
3. No Core dependency on `ark-crypto` is introduced. The current boundary remains one-way and Core stays crypto-agnostic.
4. A-05 fuzz targets call the actual public KDF APIs. The latest [KDF fuzz smoke run #7](https://github.com/RhinoGuard/ARK/actions/runs/44624226096) passed on head `be376f62ee454ca359f79c278f69fe3d882e3231`: Argon2id completed 754 runs / 21 seconds and HKDF completed 4,633,173 runs / 21 seconds with no reported crash. This is a bounded smoke sample only; a sustained owner-reviewed fuzz campaign and broader crash triage remain outstanding.
5. Issue #17 and PR #23 remain open; this reconciliation is evidence for owner review, not a closure or approval record.

### 12.4 Decisions still requiring explicit owners

- Crypto owner (`ZeroTrace79`): CCB-01/02/06 API/provenance semantics, CCB-03 random-material generation contract, CCB-04 supported profiles and CCB-05 sanitized failure mapping.
- Systems Engineering: service boundary/error mapping placement, resource budgets and compatibility requirements.
- Security Validation (`alieliteworks-debug`): independent review of the current test/KAT/fuzz surface and residual password-to-IKM misuse risk.
- Platform and future Storage/Package/Recovery owners: CCB-03 entropy provider ownership and CCB-07 authenticated metadata/migration design.

Until these decisions are reviewed, this document remains **PROPOSED** even though parts of the KDF API and its tests have advanced.


## 13. Revalidation after hardened Crypto PR creation (2026-11-20)

The current implementation candidate is now tracked by Draft PR #28 (fix/crypto-kdf-production-api-boundary) against main, currently at head c22acc0474ef067ad90dd5bbced7ed32f9f6d343. PR #28 is the hardened Crypto/KDF candidate; it is distinct from the older PR #3. The current PR #28 head has not yet completed a fresh CI run.

- An earlier CI run on PR #28 head b5d0c6ab1c0cab6729a15a03f6c301a803c9a071 failed because test-only KAT helpers were re-exported at an invalid visibility boundary and then became dead code in non-test builds. The source was corrected by removing those re-exports and applying cfg(test) to the KAT-only validation helper/constant. This correction is at the current head above; it remains pending exact-head CI.
- PR #26 remains stacked on PR #28 but its head 7e9f47393628e605e8d0ff400c579157da52f5bd is four commits behind the current Crypto branch. Its exact-head ARK CI run #64 passed all six jobs, including dependency audit, and its bounded KDF fuzz smoke #21 passed. These results validate that exact PR #26 head, not the newer PR #28 head or a future rebased PR #26 head.
- PR #27 previous CI and fuzz successes are for its earlier base snapshot. It must be revalidated after the stack is synchronized.

The API-level disposition remains unchanged: CCB-01/06 are open because HkdfIkm checks length but does not attest source provenance; CCB-03/04/05/07 remain open for CSPRNG ownership, platform calibration, service-level error mapping, and persisted metadata/migration. The six-purpose production context registry is implemented in the candidate but still requires the relevant contract review.

Decision: PR #28 is not yet CI-accepted; PR #26/#27 are not yet synchronized to its current head; this contract remains PROPOSED. Do not infer a completed Crypto integration from the green older staged validation run.
