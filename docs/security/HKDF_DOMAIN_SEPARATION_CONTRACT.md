# ARK HKDF-SHA256 Domain Separation and Context Contract

- **Status:** Normative KDF Subsystem Contract (`[S-04]` / Issue #7)
- **Owning Subsystem:** `ark-crypto` (`crates/ark-crypto/src/kdf/hkdf_sha256.rs`)
- **Author:** Saleh (`@ZeroTrace79`)
- **Related ADRs & Invariants:** ADR-0003, ADR-0005, `ARK-SEC-016`, `ARK-SEC-028`

---

## 1. Purpose and Scope

This document defines the canonical domain-separation contract, context encoding rules, Extract/Expand semantics, salt policy, and input/output bounds for HKDF-SHA-256 (`RFC 5869`) in `ark-crypto`.

This specification enforces cryptographic key separation (`ARK-SEC-016`) across ARK subsystems so that subkeys derived from a shared parent key cannot collide or be confused across distinct protocol purposes.

> **Security Boundary Note:** Cryptographic domain separation guarantees that distinct purpose labels produce computationally independent subkeys from the same high-entropy input key material (IKM). It does **not** provide process authorization, access control, or protection if the underlying root IKM is compromised.

---

## 2. HKDF-SHA-256 Extract and Expand Semantics

`ark-crypto` uses full two-stage **HKDF-SHA-256** (`Hkdf<Sha256>` from `RustCrypto`):

1. **Extract Stage (`HKDF-Extract(salt, IKM) -> PRK`):**
   - Extracts a fixed 32-byte pseudorandom key (`PRK`) from the caller-supplied input key material (`IKM`) and optional `salt`.
2. **Expand Stage (`HKDF-Expand(PRK, info, L) -> OKM`):**
   - Expands the `PRK` with the canonical domain-separation `context` (`info`) into a zeroizing 32-byte output key (`Zeroizing<[u8; KEY_LEN]>`).

---

## 3. Canonical Context / Info Encoding and Purpose Separation

### 3.1 Canonical Grammar (`"ARK/v1/<purpose>"`)

To remediate `SEC-KDF-04` (unstructured ad-hoc context strings), every context passed to `derive_subkey` or `HkdfContext::parse` MUST obey the following grammar:

```text
context := "ARK/v1/" <purpose-suffix>
```

Where:
- **Protocol & Version Prefix:** Must begin with the exact ASCII byte prefix `ARK_V1_CONTEXT_PREFIX` (`b"ARK/v1/"`).
- **Purpose Suffix (`<purpose-suffix>`):**
  - Must be non-empty (`>= 1` byte).
  - Must contain only printable ASCII graphic characters (`0x21..=0x7E`), strictly excluding whitespace (`0x20`), control characters (`0x00..=0x1F`, `0x7F`), and non-ASCII bytes.
- **Total Length Bound:** Must not exceed `MAX_CONTEXT_LEN = 256` bytes (`8..=256` bytes total for valid v1 contexts).

Any context that violates these rules is rejected fail-closed with:
- `Err(HkdfError::EmptyContext)` if `context.is_empty()`
- `Err(HkdfError::ContextTooLong)` if `context.len() > 256`
- `Err(HkdfError::InvalidContextFormat)` if the `"ARK/v1/"` prefix is missing, the suffix is empty (`b"ARK/v1/"`), or the suffix contains whitespace/non-printable bytes.

### 3.2 Purpose-Bound Enumeration (`HkdfKeyPurpose`)

To prevent callers from constructing ad-hoc or ambiguous strings, `ark-crypto` provides the closed enum `HkdfKeyPurpose` and the typed entry point `derive_purpose_subkey`:

| `HkdfKeyPurpose` Variant | Canonical Context Bytes (`info`) | Intended Subsystem Role |
| :--- | :--- | :--- |
| `HkdfKeyPurpose::WorkspaceKey` | `b"ARK/v1/workspace-key"` | Workspace master key wrapping |
| `HkdfKeyPurpose::ProjectKey` | `b"ARK/v1/project-key"` | Project-level key isolation |
| `HkdfKeyPurpose::FileKey` | `b"ARK/v1/file-key"` | File payload encryption |
| `HkdfKeyPurpose::MetadataKey` | `b"ARK/v1/metadata-key"` | Authenticated metadata protection |
| `HkdfKeyPurpose::AuditKey` | `b"ARK/v1/audit-key"` | Local audit log integrity |
| `HkdfKeyPurpose::RecoveryKey` | `b"ARK/v1/recovery-key"` | Recovery bundle wrapping |

### 3.3 Usage Example (Non-Secret Test Inputs)

```rust
use ark_crypto::kdf::{
    HkdfIkm, HkdfKeyPurpose, derive_purpose_subkey,
};

let root_key = [0x42_u8; 32]; // 256-bit high-entropy parent key
let ikm = HkdfIkm::from_key_array(&root_key);

let project_subkey = derive_purpose_subkey(ikm, None, HkdfKeyPurpose::ProjectKey)?;
let file_subkey = derive_purpose_subkey(ikm, None, HkdfKeyPurpose::FileKey)?;
assert_ne!(&*project_subkey, &*file_subkey);
```

---

## 4. Input Key Material (IKM) Contract (`SEC-KDF-03`)

- **Minimum Length Enforcement:** Both `HkdfIkm::from_slice` and `derive_subkey` reject empty IKM with `Err(HkdfError::EmptyInputKeyMaterial)` and reject IKM shorter than `MIN_IKM_LEN = 16` bytes (128 bits) with `Err(HkdfError::InputKeyMaterialTooShort)`.
- **Recommended Root Key Size:** `RECOMMENDED_IKM_LEN = 32` bytes (256 bits), matching `KEY_LEN` and `HkdfIkm::from_key_array(&[u8; 32])`.
- **Key Quality vs. Input Length:** A minimum byte length check is a supplementary defense against truncated tokens, **not** proof of cryptographic entropy. Callers MUST only pass high-entropy key material produced by `derive_kek` (Argon2id) or an OS CSPRNG. Raw user passwords MUST NEVER be passed directly to HKDF.

---

## 5. HKDF Salt Semantics vs. Argon2id Password Salt Semantics

HKDF salts and Argon2id password salts serve fundamentally different cryptographic roles and must not be conflated:

| Dimension | Argon2id Password Salt (`derive_kek`) | HKDF Extract Salt (`derive_subkey`) |
| :--- | :--- | :--- |
| **Input Source** | Low-entropy human password | Already high-entropy cryptographic key (`IKM`) |
| **Requirement** | **Mandatory** (`16..=64` bytes, non-zero, unique per workspace) | **Optional** (`Option<&[u8]>`); `None` uses RFC 5869's 32-byte zero-salt default |
| **Primary Goal** | Defeat offline precomputation / rainbow tables and equalize work across users | Strengthen extraction independence across distinct salt domains when IKM is reused |

---

## 6. Output Length Constraints and Zeroization Wrapper Rationale

- **ARK Subkey Output (`KEY_LEN = 32`):** `derive_subkey` and `derive_purpose_subkey` return `Zeroizing<[u8; 32]>` (`256` bits). Returning a fixed-size `Zeroizing<[u8; 32]>` guarantees automatic memory scrubbing on drop while keeping `ark-crypto` decoupled from downstream AEAD or storage key types.
- **RFC 5869 Theoretical Ceiling (`HKDF_SHA256_MAX_OUTPUT_LEN = 8160`):** Per RFC 5869 Section 2.3, HKDF-SHA-256 can expand at most `255 * HashLen = 255 * 32 = 8160` bytes. The RFC conformance entry point `hkdf_sha256_rfc5869_expand` explicitly validates `1..=8160` bytes and returns `Err(HkdfError::InvalidOutputLength)` for `0` or `> 8160` bytes.
