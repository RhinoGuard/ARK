# ARK

**Cross-platform secure workspace for developers.**

ARK is designed to provide encrypted project storage and controlled runtime access for developer tools. The architecture separates durable encrypted storage, cryptographic key management, identity, authorization policy, runtime mediation, and platform-specific filesystem integration.

> **Project status:** The initial Cargo workspace and platform-independent ark-core domain foundation are implemented on the foundation branch. The runtime, cryptographic engine, encrypted storage, identity/session/policy/broker services, filesystem mediation, platform backends, package/recovery engines, and user-facing applications are not yet implemented. Design documents and foundation tests are not evidence of end-to-end security.

## Start here

- [Developer Guide](docs/development/DEVELOPER_GUIDE.md)
- [Team Workflow](docs/development/TEAM_WORKFLOW.md)
- [Commit Conventions](docs/development/COMMIT_CONVENTIONS.md)
- [AI Agent Instructions](AGENTS.md)
- [Contributing](CONTRIBUTING.md)
- [Architecture Overview](docs/architecture/OVERVIEW.md)
- [Crate Boundaries and Dependencies](docs/architecture/CRATE_BOUNDARIES_AND_DEPENDENCIES.md)
- [Versioning and Migration](docs/architecture/VERSIONING_AND_MIGRATION.md)
- [Security Model](docs/security/SECURITY_MODEL.md)
- [Threat Model](docs/security/THREAT_MODEL.md)
- [Security Invariants](docs/security/SECURITY_INVARIANTS.md)
- [Identity and Policy](docs/security/IDENTITY_AND_POLICY.md)
- [Sensitive Operations](docs/security/SENSITIVE_OPERATIONS.md)
- [Cryptography Design](docs/cryptography/CRYPTOGRAPHY.md)
- [Encrypted Storage Design](docs/storage/STORAGE_DESIGN.md)
- [Runtime Design](docs/runtime/RUNTIME_DESIGN.md)
- [Filesystem Mediation](docs/filesystem/FILESYSTEM_MEDIATION.md)
- [Recovery Design](docs/recovery/RECOVERY_DESIGN.md)
- [Portable Package Format](docs/package/PORTABLE_PACKAGE.md)
- [Test Strategy](docs/testing/TEST_STRATEGY.md)
- [Architecture Decision Records](docs/decisions/README.md)
- [Security Reporting](SECURITY.md)

## Product principles

Security by default; default deny; least privilege; separate identity, authentication, session, and authorization; standard cryptography; explicit trust boundaries; durable encrypted state; UI is never security authority; and no convenience bypass.

## Platform and implementation direction

Windows, Linux, and macOS are intended targets, delivered incrementally: Windows first, Linux second, macOS third. Rust is the primary language. Use a modular Cargo workspace with OS-specific backends. Separate processes only where a real security or reliability boundary exists.

## MVP target

A real PHP project stored in encrypted workspace storage, edited through Development Mode, and served by Apache through scoped Execution Mode access. The MVP must exercise authenticated chunk storage, durable journaling/crash recovery, process identity/policy checks, and portable encrypted export/import.

## Security limitations

ARK does not claim protection against a fully compromised kernel, malicious privileged administrator, or an authorized application that copies/exfiltrates plaintext. Filesystem authorization is not DLP. Revocation cannot be called instantaneous for existing handles/mappings unless the selected backend proves it.

## Status discipline

Track designed, implemented, tested, and verified separately. Never advertise a planned feature as complete.
