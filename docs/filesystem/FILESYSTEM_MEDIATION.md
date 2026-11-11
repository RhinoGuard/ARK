# Filesystem Mediation Design

## Purpose
ARK Secure Access Layer (ASAL) is a working name for a common filesystem mediation layer. It is not a finalized product/API name.

## Design
Use a common filesystem contract with real OS-specific mediation backends. External applications should interact through ordinary supported filesystem APIs; ARK must mediate the operations it claims to enforce.

Candidate backends:
- Windows: evaluate WinFsp.
- Linux: evaluate FUSE.
- macOS: evaluate current File Provider/FSKit or another suitable supported mechanism.

Selection requires prototype, maintenance/platform support review, performance measurements, security review, and compatibility tests.

## Operation coverage
Explicitly define support and semantics for open, read, write, create, rename, delete, enumerate, locks, truncate, flush, notifications, memory mapping, and handle lifetime. A stub, path rewrite, or userspace API facade that does not intercept the real operation is not mediation.

## Process identity and IPC
Authenticate the caller using platform peer/process evidence. IPC must have restricted ACLs, authenticated peer identity, replay resistance, bounded message sizes, timeouts, rate limits, and strict input validation. Do not trust caller-supplied PID/path as proof.

## Limitations
Do not claim control over raw disk access, kernel/privileged processes, unsupported APIs, or all possible OS paths. Publish an operation matrix per platform/backend. Existing handles/mappings may outlive revocation unless backend semantics prove otherwise.

## Authorization
The broker and policy engine are the authority. The filesystem backend enforces their decision for each supported operation and resource identity. User-controlled path strings alone are never authorization tokens.
