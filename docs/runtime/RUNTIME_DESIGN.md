# Runtime Design

## Modes
ARK has two distinct runtime modes: Development Mode and Execution Mode. They have different access patterns and must share the same identity, policy, session, and storage security boundaries.

## Development Mode
- Expose only files that are actively needed for editing in the working set; do not expose the entire project by default.
- Use encrypted-backed working data. Durable source of truth remains encrypted storage plus encrypted journal.
- Buffer and group writes for performance, but durable commit must not depend on long timers or shutdown callbacks.
- Closing a tab is not evidence that all OS handles, memory mappings, editor extensions, indexers, child processes, or asynchronous writes have stopped.
- Release plaintext working data only after relevant handles/operations are accounted for. Do not claim perfect memory/disk erasure.
- Chunk-level updates should avoid re-encrypting unchanged chunks.

## Execution Mode
The user authorizes a process identity for a specific resource scope and duration. The application requests dependencies dynamically at runtime; static source analysis cannot reliably enumerate all dynamic file accesses.

A directory grant exposes every file covered by that scope to the authorized application. UI must clearly describe the scope, duration, and application identity. All requests are checked against current process identity, session, policy, and resource identity.

## Identity and descendants
PID/name/path alone are insufficient. Platform evidence must be bound to a live process instance and checked against replacement/race conditions. Child processes, plugins, restarts, inherited handles, and handle passing require explicit policy. Do not silently inherit grants to arbitrary descendants.

## Revocation
Revocation prevents future mediated requests once observed. Existing handles/mappings or in-flight work may continue unless the backend can reliably invalidate them. Document backend-specific semantics and test them.

## Filesystem mediation
The common filesystem layer defines supported operations. Platform backend must mediate the actual OS operations it claims to support, including relevant open/read/write/create/rename/delete/enumerate/lock/flush/truncate/notification/mapping semantics. A stub or path rewrite is not filesystem interception.

Candidate backends: WinFsp on Windows, FUSE on Linux, and a suitable current Apple mechanism on macOS. Prototype compatibility and security before selection. Publish an explicit operation/limitation matrix.

## Process model
Start with user-process runtime where that reduces complexity and meets threat requirements; do not assume a privileged service is inherently safer. If a service is later required, use a restricted service identity, narrow IPC ACLs, authenticated peer identity, and least privilege. Avoid LocalSystem/root privileges unless justified.

## Limits
An authorized application can copy or exfiltrate plaintext. Filesystem permissions do not provide network egress control or general DLP. ARK does not promise to mediate raw disk, kernel, privileged, or every possible OS access path.
