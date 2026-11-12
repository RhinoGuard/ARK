# ARK — focused `ark-crypto` contribution

This archive contains only a starter contribution for the assigned key-derivation tasks:

1. Candidate cryptographic crate dependencies.
2. Argon2id password-to-KEK derivation.
3. HKDF-SHA-256 context-separated subkey derivation.
4. Initial unit tests.

It does **not** modify or replace any existing ARK repository files. Copy this crate into the repository only after checking the current branch and the workspace layout with the project maintainers.

## Important status

- The dependency versions are candidate pins, not yet accepted by ARK's maintainers.
- Argon2id defaults (64 MiB, 3 iterations, 1 lane) are provisional development values. Benchmark them on supported target hardware before adopting a product policy.
- Tests use reduced Argon2id parameters for speed. They are functional tests, not production-parameter validation or a security audit.
- No build or test has been run while preparing this archive because the Rust toolchain is not installed in the preparation environment.

## What is intentionally out of scope

This contribution does not implement key wrapping, workspace master-key storage, file encryption, nonce allocation, streaming/chunk encryption, recovery, authorization, filesystem mediation, or GUI integration. Those designs require their own reviewed contracts before implementation.

## Contents

```text
ark-crypto/
├── Cargo.toml
├── README.md
├── src/
│   ├── lib.rs
│   └── kdf/
│       ├── mod.rs
│       ├── argon2id.rs
│       └── hkdf_sha256.rs
└── tests/
    └── kdf_tests.rs
```

## Local validation (when Rust is installed)

From this directory:

```bash
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
```

Review the output and dependency graph before integrating into ARK. Do not claim the crate is production-ready based only on these tests.
