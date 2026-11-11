# ARK System Architecture

Status: ACCEPTED direction with implementation details and platform mechanisms subject to prototypes and review.

## 1. Product goal

ARK is a Secure Workspace for developers. It stores project/workspace data in an encrypted format and mediates access by authorized applications through controlled runtimes. The target platforms are Windows, Linux, and macOS.

## 2. Architectural principles

- Security by default and default deny.
- Least privilege and explicit trust boundaries.
- Separation of identity evidence, authentication, authorization, and execution.
- UI is not a security authority.
- No path-as-authority.
- Fail closed on uncertain identity, policy, session, parsing, or integrity state.
- Platform-specific enforcement behind narrow interfaces.
- Crash consistency, concurrency, revocation, migration, and performance are design requirements.
- No custom cryptographic primitives or protocols.
- No premature microservices/process decomposition; begin with a modular Rust workspace and split processes only where a real security or reliability boundary justifies it.

## 3. Logical architecture

User Interface (future Tauri app)
  -> Local Control API
  -> Security Broker
  -> Identity Verification + Session Validation + Policy Evaluation
  -> Runtime
  -> Filesystem/Temporary Working Set backend
  -> Storage Engine
  -> Cryptographic Engine
  -> Encrypted Workspace Storage

All protected data operations must pass through the appropriate trusted authorization and storage boundary. UI requests are untrusted input and must be validated by the backend.

## 4. Intended subsystem responsibilities

- ark-core: platform-independent domain types, typed IDs, error categories, and narrow contracts.
- ark-crypto: reviewed primitives, KDFs, key derivation/wrapping, authenticated chunk operations, and secret-handling helpers.
- ark-storage: versioned encrypted format, chunk index, journal, transactions, snapshots, locking, and recovery.
- ark-identity: OS-backed identity evidence for users, devices, processes, and executables.
- ark-policy: deterministic default-deny evaluation, scopes, explicit deny precedence, and policy versioning.
- ark-session: session binding, expiry, revocation, and cache invalidation.
- ark-broker: authorization orchestration and the controlled access path.
- ark-runtime: Development Mode and Execution Mode lifecycle.
- ark-filesystem: common filesystem operations/semantics and mediation contracts.
- platform backends: OS-specific identity and filesystem enforcement.
- ark-package: portable encrypted package builder, verifier, and importer.
- ark-recovery: recovery methods and key-wrapper lifecycle.
- ark-audit: privacy-conscious audit events and integrity chaining.

These are boundaries to guide implementation, not a requirement to create empty crates. Avoid cyclic dependencies and keep ark-core independent of concrete OS, GUI, storage, and crypto implementations.

## 5. Runtime modes

### Development Mode

A limited active working set exposes only files needed by an authorized editor. The editor must receive plaintext for active file operations, so plaintext exposure cannot be eliminated completely. Minimize exposure duration, scope, persistence, and copies. Durable changes must be protected by an authenticated journal/transaction mechanism; editor tab closure alone is not a reliable signal that all handles are gone.

### Execution Mode

The user grants an application an explicit directory/resource scope and operation set for a bounded session. The runtime serves dynamically requested dependencies within that scope. ARK does not attempt to discover all runtime dependencies by parsing source code. A scope can expose many files; the UI must explain that breadth and associated risk.

## 6. Filesystem mediation

A shared filesystem abstraction is intended to support ordinary applications without per-application source adapters. Actual mediation requires a platform filesystem mechanism or equivalent OS enforcement, not passive monitoring or API hooks alone.

The implementation must evaluate open/read/write/create/delete/rename/enumerate/flush/truncate/locks/mmap/notifications, handle lifetime, directory semantics, symlinks, hard links, alternate paths, child processes, process restart, and concurrent access. Backend capability gaps must be documented and tested.

No universal guarantee is made that every application or every possible access path can be mediated. Compatibility is an explicit, measured support matrix.

## 7. Security levels

Standard, Elevated, and Critical map operations to policy and step-up authentication requirements. They are not different cryptographic algorithms or three master passwords.

## 8. Portable packages and recovery

Portable packages contain authenticated encrypted data and versioned metadata. Each package uses an independent random package key and an independent password-derived wrapper. Recovery routes are specified separately and must not silently collapse into one shared password.

## 9. Platform order

Windows first, Linux second, macOS third. Cross-platform domain and storage contracts must remain portable. OS-specific identity and filesystem behavior is implemented behind platform-specific backends.

## 10. Trust boundaries

Treat as untrusted: GUI, application processes, filesystem requests, IPC messages, persisted metadata, package input, environment variables, paths, and data read from storage. The broker, policy, crypto, and storage code are security-critical and require narrow interfaces, tests, and independent review.
