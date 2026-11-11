# ADR-0002: Separate Development Mode and Execution Mode

Status: Accepted

## Context

Editors usually modify a limited active set of files. Runtime applications may dynamically request many files and dependencies. Trying to infer every source-level dependency is unreliable.

## Decision

Provide distinct Development Mode and Execution Mode.

Development Mode uses a bounded active working set backed by encrypted durable storage and an authenticated crash-recoverable journal/transaction design.

Execution Mode uses explicit user-defined directory/resource scopes, operation permissions, verified application identity, and bounded sessions. It serves dynamic file requests inside the granted scope rather than relying on complete static dependency discovery.

## Alternatives

- Decrypt entire workspace for all applications: rejected due to excessive plaintext exposure.
- Parse source code to discover every dependency: rejected because dynamic paths, reflection, plugins, generated names, and runtime configuration defeat completeness.
- Use one identical access model for editing and execution: rejected due to incompatible access patterns.

## Consequences

- Scope breadth must be clearly explained to users.
- Development mode must handle editor handles, mappings, caches, and background operations.
- Execution mode requires real filesystem mediation and a compatibility matrix.

## Security implications

An authorized app may copy plaintext it receives. Directory scopes limit what it can read through ARK but do not by themselves provide DLP or network egress control.

## Validation

VS Code save/open/rename/watch behavior and Apache/PHP dynamic include behavior on the Windows MVP backend.
