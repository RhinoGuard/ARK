# ARK Cryptography Specification

Status: Selected primitive families are ACCEPTED directionally. Exact library, version, feature set, parameters, wire format, nonce allocation, and misuse analysis remain implementation gates.

## 1. Rules

- Use established, reviewed cryptographic constructions and maintained libraries.
- Do not invent an encryption algorithm, KDF, MAC, stream construction, or key exchange.
- Separate keys by purpose and context.
- Use the operating system CSPRNG for key and salt generation.
- Authenticate encrypted content and security-critical metadata.
- Treat key lifecycle, nonce lifecycle, backup, rotation, crash, clone, and restore as part of the cryptographic design.
- Cryptographic APIs must make unsafe states difficult to express.

## 2. Selected primitive families

- XChaCha20-Poly1305 for authenticated encryption with associated data, subject to library suitability and reviewed nonce strategy.
- Argon2id for password-based key derivation.
- HKDF-SHA-256 for context-separated derivation from high-entropy key material.
- OS-provided CSPRNG for random keys, salts, identifiers where appropriate, and recovery material.

Do not substitute primitives or parameter sets without an ADR and review.

## 3. Key hierarchy direction

Workspace password -> Argon2id with unique salt and versioned parameters -> KEK -> unwrap random Workspace Master Key -> derive or unwrap scoped Project/File keys using a reviewed hierarchy and HKDF context labels.

Password is not the file-encryption key. Password rotation should normally rewrap the relevant random key rather than re-encrypt every file, unless a separate data-key rotation is required.

Recovery Key, Recovery File, Trusted Device, and Portable Package each have explicitly separated wrappers/credentials and scoped key-release paths. Do not store raw master keys in recovery/package files.

## 4. Chunked authenticated storage

Random-access chunk encryption is required for large files and frequent small edits. The final construction must define:
- chunk size and alignment;
- file identity and generation binding;
- chunk index and length binding;
- AAD fields and canonical encoding;
- nonce uniqueness per key across concurrent writes, crashes, restore, clone, and rollback;
- authenticated header/manifest;
- truncation, reordering, duplication, substitution, and stale-chunk detection;
- atomic metadata/index publication;
- version and algorithm-suite identifiers.

A plain loop that independently encrypts arbitrary chunks is not automatically a safe storage protocol. Select or adapt a reviewed streaming/chunked AEAD design and document its security argument. Do not release plaintext from a chunk until its authentication tag verifies.

## 5. Nonce and key lifecycle

Nonce uniqueness is a hard invariant. Random nonces alone require a carefully analyzed collision bound and key usage limit; counters require durable atomic allocation and rollback/clone handling. The exact strategy is OPEN until the chosen library/construction and storage transaction model are evaluated.

A key must not be reused across independent workspaces, files, packages, or purposes without a documented derivation scheme. Use explicit context labels and domain separation.

## 6. Password and package credentials

Use Argon2id with unique salt and versioned parameters calibrated on supported hardware. The parameters must be benchmarked and bounded to resist denial-of-service via attacker-controlled KDF metadata. The package password wraps a random Package Key; it does not directly encrypt file chunks.

Never store plaintext passwords. Avoid password exposure in logs, crash reports, command-line arguments, environment variables, and telemetry.

## 7. Secret memory

Use zeroization helpers and minimize secret lifetime, copies, and formatting. Recognize that Rust zeroization cannot guarantee removal from compiler-generated copies, registers, swap, hibernation, crash dumps, or a compromised OS. Do not claim perfect memory erasure.

## 8. Required validation

Before crypto code is accepted:
- review the library and exact version/features;
- document key hierarchy and context labels;
- define nonce allocation and crash/clone behavior;
- test wrong key, modified ciphertext/AAD, truncation, reordering, invalid metadata, and boundary lengths;
- use published test vectors where applicable;
- fuzz parsers and format boundaries;
- independent review by qualified team members;
- document known limits and algorithm/version migration path.
