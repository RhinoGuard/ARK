# ARK Threat Model

Status: Initial baseline; must be refined as implementation and platform prototypes reveal concrete attack surfaces.

## 1. Security objectives

- Protect workspace/project data at rest using authenticated encryption.
- Mediate supported application access through explicit identity, session, and policy checks.
- Minimize plaintext exposure during development and execution.
- Preserve committed data across supported crash/restart scenarios.
- Detect tampering, corruption, malformed packages, and unauthorized access attempts.
- Provide controlled recovery and portable encrypted transfer.

## 2. Assets

- Workspace master keys and derived keys.
- Passwords, recovery keys, package credentials, and device private keys.
- Encrypted file chunks, manifests, metadata, journals, snapshots, and package contents.
- Plaintext working-set data and process memory.
- Identity evidence, sessions, grants, policies, and device registrations.
- Audit records and update/signing metadata.

## 3. Adversaries and capabilities

Consider:
- malicious or compromised application process;
- malicious plugin/extension running inside an authorized editor;
- local unprivileged user;
- malicious process under the same user;
- attacker with access to copied encrypted package/storage;
- attacker modifying, truncating, reordering, replaying, or replacing storage/package data;
- malicious or compromised dependency/build tool;
- compromised update channel or signing key;
- privileged administrator, kernel, hypervisor, or fully compromised OS.

The final category is outside the core confidentiality guarantee. ARK must state this clearly and consider defense-in-depth where practical.

## 4. Trust boundaries

- UI to local control API.
- Application process to filesystem/runtime backend.
- OS identity evidence to ARK identity verifier.
- Broker to policy/session/identity components.
- Runtime to storage and crypto.
- Encrypted storage to parser/format implementation.
- Portable package import from external/untrusted media.
- Update manifest/package to updater.
- Device recovery proof to key-wrapper release logic.

Every boundary requires authentication where relevant, validation, bounded input, explicit errors, and tests.

## 5. Threats to analyze

- unauthorized process access and process impersonation;
- PID reuse, stale process handles, parent/child process abuse;
- symlink/path traversal, hard links, alternate data streams, mount points, namespace changes, TOCTOU;
- IPC spoofing, replay, confused deputy, malformed messages, denial of service;
- stale policy/session caches and incomplete revocation;
- open handles and memory mappings surviving apparent file closure;
- plaintext leakage through editor caches, search indexes, swap/pagefile, hibernation, crash dumps, backups, antivirus, temporary files, and logs;
- nonce reuse after crash, restore, clone, or concurrent writer;
- chunk truncation, reordering, substitution, replay, and metadata tampering;
- WAL corruption, partial commit, reordered flush, filesystem durability differences;
- rollback to an older authentic storage snapshot;
- package parser bombs, oversized lengths, path traversal on import, duplicate paths, symlink escape, decompression bombs if compression is added;
- recovery-key theft, package-password guessing, malicious recovery device;
- compromised dependencies, build pipeline, release signing, and update rollback.

## 6. Explicit limitations

- If an authorized application receives plaintext, ARK cannot by itself stop that application from copying or sending it elsewhere.
- A fully compromised OS/kernel, malicious administrator, or hypervisor can inspect process memory and interfere with enforcement.
- Plaintext may appear in RAM and may be paged, dumped, cached, indexed, backed up, or copied by the OS/application.
- Deleting a temporary file does not guarantee physical erasure on SSDs, copy-on-write filesystems, snapshots, backups, or journaling filesystems.
- A shutdown handler cannot run reliably after sudden power loss.
- Revocation cannot automatically retract plaintext already copied or guarantee that all existing handles/mappings stop immediately.
- Generic filesystem mediation is platform- and application-dependent; universal compatibility is not promised.

## 7. Security analysis method

For each feature:
1. Identify assets, actors, entry points, trust boundaries, and assumptions.
2. Enumerate confidentiality, integrity, availability, spoofing, tampering, repudiation, information disclosure, denial of service, and elevation-of-privilege threats (STRIDE as a checklist, not a substitute for reasoning).
3. Define prevention, detection, and recovery controls.
4. Define negative tests and fault-injection cases.
5. Record residual risk and user-visible warnings.
6. Update the model when code, dependencies, platform behavior, or product claims change.

## 8. Release posture

No release should carry unresolved known Critical or High security findings. This is a release policy target, not proof that no unknown vulnerabilities exist. Security claims require evidence, independent review, and documented residual risk.
