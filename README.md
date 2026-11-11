# ARK

**Cross-platform secure workspace for developers.**

ARK is intended to provide encrypted project storage and controlled runtime access for developer tools. Its design separates durable encrypted storage, cryptographic key management, identity, authorization policy, runtime mediation, and platform-specific filesystem integration.

> **Project status:** Architecture and documentation baseline. The repository is not yet a functioning secure-storage or runtime implementation. Do not interpret design goals as implemented or independently verified security properties.

## Start here

- [Developer Guide](docs/development/DEVELOPER_GUIDE.md)
- [Architecture Overview](docs/architecture/OVERVIEW.md)
- [Security Model](docs/security/SECURITY_MODEL.md)
- [Threat Model](docs/security/THREAT_MODEL.md)
- [Security Invariants](docs/security/SECURITY_INVARIANTS.md)
- [Cryptography Design](docs/cryptography/CRYPTOGRAPHY.md)
- [Storage Design](docs/storage/STORAGE_DESIGN.md)
- [Runtime Design](docs/runtime/RUNTIME_DESIGN.md)
- [Recovery Design](docs/recovery/RECOVERY_DESIGN.md)
- [Portable Package Format](docs/package/PORTABLE_PACKAGE.md)
- [Architecture Decision Records](docs/decisions/README.md)
- [AI Contributor Instructions](AGENTS.md)
- [Contributing](CONTRIBUTING.md)
- [Security Reporting](SECURITY.md)

## Product principles

1. Security by default; deny access unless an explicit, valid policy grants it.
2. Separate identity, authentication, session validity, and authorization.
3. Keep cryptographic algorithms standard and use maintained, reviewed implementations.
4. Treat the operating system, kernel, privileged administrators, and authorized applications as explicit trust boundaries.
5. Prefer durable, recoverable encrypted state over optimistic claims about cleanup.
6. Do not let the GUI or an AI coding agent become a security authority.
7. Make security properties testable and traceable to implementation and evidence.
8. Preserve a usable developer workflow without silently weakening policy.

## Supported platforms and implementation direction

The intended platforms are Windows, Linux, and macOS, delivered incrementally: Windows first, Linux second, macOS third. Rust is the primary implementation language. The architecture is a modular Rust workspace with platform-specific backends. Separate processes are introduced only where they create a meaningful security or reliability boundary.

## MVP target

Demonstrate a real PHP project stored in encrypted workspace storage, edited through a controlled Development Mode workflow, then served by Apache through scoped Execution Mode access. The MVP must exercise authenticated chunk storage, durable journaling and crash recovery, process identity and policy checks, and portable encrypted export/import. Platform mediation claims must be limited to the operations actually enforced by the selected backend.

## Non-goals and limitations

ARK must not claim protection from a fully compromised kernel, malicious privileged administrator, or an authorized application that deliberately copies or exfiltrates plaintext. Filesystem authorization is not general data-loss prevention. Revocation may block future mediated requests but cannot be described as instantaneous for already-open handles or mappings unless the backend proves that behavior.

## Project status

Architecture decisions and implementation status must be tracked separately. A design document is not evidence that a feature exists. See [Security Model](docs/security/SECURITY_MODEL.md) and [Developer Guide](docs/development/DEVELOPER_GUIDE.md).
