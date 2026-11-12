# Test and Verification Strategy

Testing is a security control, not proof of bug-free software. Every security requirement must map to an enforcement point and observable evidence.

## Test layers
1. Unit tests for domain types, parsers, crypto wrappers, policy decisions, and state machines.
2. Property tests for serialization, round trips, invariants, chunk boundaries, and malformed inputs.
3. Fuzzing for storage/package parsers, metadata, IPC messages, and untrusted input.
4. Integration tests for broker-policy-runtime-storage interactions.
5. Platform tests for process identity, filesystem operations, handle lifetime, permissions, and durability.
6. Fault-injection tests for crash/power-loss boundaries and interrupted transactions.
7. Compatibility tests for versioned storage, packages, and recovery artifacts.
8. Performance benchmarks to detect regressions without relaxing security requirements.

## Security test cases
- Invalid, missing, stale, and ambiguous process identity must deny.
- Expired/revoked sessions and grants must deny new requests.
- Scope, operation, process, and duration mismatches must deny.
- Nonce reuse must be impossible by construction and tested.
- Modified headers, metadata, chunks, order, truncation, and substitution must fail authentication/validation.
- Old valid storage snapshots must be handled according to the anti-rollback design.
- Import path traversal, duplicate paths, oversized inputs, decompression/resource exhaustion, and malformed versions must be rejected.
- Concurrent writers, crash during every transaction stage, disk full, and partial flush must preserve documented consistency.
- Revocation with open handles/mappings must match the published backend contract.

## Fuzzing
Fuzz all parsers and protocol boundaries. Seed corpora should include valid minimal/maximum examples and previously discovered regressions. Reproduce every crash as a regression test. Fuzzing timeouts and corpus coverage must be reported honestly.

## Release criteria
- No known unresolved Critical or High security vulnerability.
- Required tests and platform compatibility checks pass.
- Sensitive changes have independent review.
- Dependency/license/SBOM and release-signing checks complete.
- Known limitations and unsupported operations documented.
- No unverified claim of security or compatibility in release notes.

## Evidence
Record command, toolchain, platform, commit SHA, result, and relevant artifact for validation. A green unit test suite does not replace threat analysis or security review.
