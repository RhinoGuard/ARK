# Cryptography Design

**Status:** Design baseline. Exact wire/storage protocol, crate versions, parameters, and key lifecycle APIs require dedicated ADRs and review before implementation.

## Approved direction
- Authenticated encryption: XChaCha20-Poly1305.
- Password-based key derivation: Argon2id.
- Domain-separated key derivation: HKDF-SHA-256.
- Random keys/nonces/tokens: operating-system CSPRNG through a reviewed Rust abstraction.
- Use maintained, reviewed libraries. Do not design proprietary algorithms.

## Key hierarchy
Intended conceptual hierarchy:
```
Password
  └─ Argon2id → KEK
       └─ unwrap Workspace Master Key
            └─ derive/wrap Project Key
                 └─ derive per-file key
```
Password is not used directly as a file-encryption key. Domain separation and context binding must be specified. Password rotation normally changes a key wrapper and need not re-encrypt all data; data-key rotation is a separate operation.

## Nonces and AAD
Nonce uniqueness under a given key is mandatory. The construction, allocation/persistence strategy, and behavior across crash/restore/fork/concurrency must be specified before code. AAD should bind ciphertext to its context, such as format version, workspace/file identity, chunk index, and relevant generation. Authenticate headers and metadata that influence interpretation.

## Chunked/streaming encryption
Use a reviewed streaming AEAD construction or established library format. Do not assume that independently encrypting arbitrary chunks is sufficient. Specify and test:
- Unique nonce per chunk/key.
- Authenticated chunk index, file identity, generation, and relevant metadata.
- Order, missing chunk, duplicate chunk, truncation, extension, and resize detection.
- Header/manifest authentication.
- Concurrent writers and key/nonce allocation.
- Atomic metadata updates and crash recovery.
- Rollback detection strategy.

AEAD authenticity alone does not prevent rollback to an older valid ciphertext set. A trusted generation reference or other anti-rollback mechanism is required if rollback resistance is a product requirement.

## Key handling
Keep key material scoped and short-lived where practical. Use zeroization helpers where appropriate, while acknowledging that copies may exist and guaranteed removal from RAM cannot generally be promised. Never log or serialize raw master keys.

## Password and recovery
Recovery Key is a random independent 256-bit secret per Workspace. It is not the account password and is not stored raw on a server. Recovery File contains authenticated wrapped key material and required metadata, not a raw master key. Trusted Devices use device-specific private keys and proof of possession.

## Portable package
Each package has a random independent Package Key. A package-specific password derives a KEK using Argon2id, which wraps the Package Key. Package password differs from the Workspace password. Rewrapping a Package Key can change package password without re-encrypting every data object, subject to format design.

## Required before implementation
Create ADRs for algorithm/library selection, KDF parameter policy, key hierarchy, streaming/chunk protocol, nonce strategy, metadata authentication, rollback model, and zeroization expectations. Benchmark on target hardware and test across platforms.
