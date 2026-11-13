# Comprehensive Security Architecture Assessment and Testing Framework

**Subsystem:** ARK Security & Verification  
**Author:** Ali (`@alieliteworks-debug`)  
**Target Subsystems:** `ark-core` (Systems Engineering), `ark-crypto` (Cryptography)  
**Status:** Approved Security Baseline  

---

## 1. Threat Model & Trust Boundaries

The ARK platform mediates access to on-disk source code and development workspaces. The security objective is zero unencrypted plaintext at rest without breaking IDE workflows and compilers.

### 1.1 Trust Domains
- **Fully Trusted:** In-memory execution state of the ARK core process. Master secrets protected in isolated memory and scrubbed via `Zeroize` upon deallocation.
- **Semi-Trusted:** Authorized developer tools (`cargo`, `rustc`, language servers) granted explicit, bounded file mediation scoped to approved Process IDs (PIDs) following interactive user confirmation.
- **Untrusted / Adversarial:** The raw underlying filesystem, untrusted peripheral drives, unprivileged operating system processes, local network interfaces, and hostile third-party binaries.

### 1.2 STRIDE Threat Assessment
1. **Spoofing:** Hostile process masquerading as an authorized compiler.  
   *Mitigation:* Binary cryptographic hash validation, executable path verification, PID verification, and interactive user consent through GUI broker.
2. **Tampering:** Bit manipulation on ciphertext chunks residing on disk.  
   *Mitigation:* Authenticated Encryption with Associated Data (AEAD); strict fail-safe rejection on tag mismatch.
3. **Repudiation:** Disputing mediation and file-access events.  
   *Mitigation:* Tamper-evident local audit records for every cryptographic transformation.
4. **Information Disclosure:** Plaintext leakage via temporary directories, hibernation files, or swap memory.  
   *Mitigation:* In-memory transient mediation; mandatory `mlock` memory pinning; enforced deterministic scrubbing.
5. **Denial of Service:** Resource exhaustion through rapid, unauthenticated broker requests.  
   *Mitigation:* Strict rate-limiting at the local broker IPC boundary.
6. **Elevation of Privilege:** Local unprivileged processes attempting memory inspection.  
   *Mitigation:* Principle of least privilege; sandbox boundary enforcement.

---

## 2. Non-Negotiable Security Invariants

1. **Zero-Plaintext on Disk Invariant:** No unencrypted source code or sensitive metadata may be persisted to permanent storage outside bounded, verified mediation contexts.
2. **Deterministic Memory Scrubbing Invariant:** Every data structure handling keys, salts, passwords, or raw plaintext must implement `Zeroize` and `ZeroizeOnDrop`.
3. **Authenticated Decryption Invariant:** No ciphertext payload is decrypted without successful tag verification.
4. **Nonce Uniqueness Invariant:** Nonce/IV values must never be reused under identical key streams (enforced by 192-bit random nonces with XChaCha20).
5. **Strict Domain Segregation Invariant:** Identifier types and runtime modes (`Development` vs `Execution`) are segregated at the type level to eliminate category confusion.

---

## 3. Adversarial & Negative Testing Matrix

| Test Suite | Malformed Input / Fault Scenario | Expected Security Behavior |
| :--- | :--- | :--- |
| **Tampered Ciphertext** | Single-bit modification in encrypted payload | Explicit `IntegrityFailure` error; zero plaintext returned |
| **Replay Attack** | Transmitting stale valid message with old Nonce | Rejected immediately due to nonce/session mismatch |
| **Invalid Key / Password** | Erroneous passphrase provided to KDF | Constant-time failure without timing leakage |
| **Truncated Header** | Corrupted or abbreviated container header | Safe parsing failure without panic or memory leakage |
| **Unapproved PID Access** | Unauthorized process reading transient buffer | Immediate denial with `AccessDenied` classification |
| **Memory Leak Verification** | Core memory dump post-session termination | No secret artifacts remaining in allocated pages |

---

## 4. Cryptographic Dependencies & Policy

Mandatory crates for cryptographic implementation:
- **AEAD:** `chacha20poly1305` (v0.10) with XChaCha20-Poly1305 (192-bit nonce space).
- **Key Derivation (KDF):** `argon2` (v0.5) adhering to the Argon2id standard, paired with `hkdf` (v0.12) using SHA-256 for domain-separated subkey extraction.
- **Memory Sanitization:** `zeroize` (v1.7+) with derive macros.
- **CSPRNG:** `rand::rngs::OsRng` exclusively.

---

## 5. Fuzz Testing Architecture

The `fuzz/` package deploys `libfuzzer-sys` to stress parser boundaries and primitives:
- `fuzz_core_ids`: Validates parsing, hex formatting, and round-trip consistency across all 7 domain identifiers.
- `fuzz_core_versions`: Stresses monotonic version ordering and bounds checks against random multi-byte inputs.
- `fuzz_kdf`: Evaluates input boundary tolerance for password and salt buffers.
