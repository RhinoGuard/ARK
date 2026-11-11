# ADR-0002: Separate Development and Execution Modes

- **Status:** Accepted (product behavior)
- **Implementation status:** Not yet implemented

## Context
Editing workflows need writable working data and rapid feedback. Runtime applications such as Apache need dynamic access to project resources. These patterns have different access needs and exposure risks.

## Decision
ARK has two distinct modes:

### Development Mode
Expose only the active working set needed for editing, rather than the whole workspace by default. Use encrypted-backed working data and durable encrypted storage/journal. Closing a tab is not proof that OS handles, mappings, extensions, indexers, child processes, and async operations have stopped.

### Execution Mode
The user grants a process a defined resource scope and duration. The application requests files dynamically. A directory grant exposes all covered files to that process while valid. All requests are checked against verified process identity, session, and policy.

## Consequences
- The two modes share crypto/storage/identity/policy boundaries.
- Development working-set lifecycle and Execution scope grants require distinct tests.
- Plaintext can exist in memory and OS-managed locations.
- An authorized process can copy or exfiltrate plaintext; this is not DLP.

## Verification
Test active-file exposure, cleanup/handle lifetime, dynamic runtime requests, process binding, scope denial, expiry, and revocation semantics on each supported backend.
