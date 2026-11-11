# Identity, Session, and Policy Design

## Separation of concerns
- **Identity:** evidence about who/what is making a request.
- **Authentication:** proof of control of an account, credential, or device.
- **Session:** bounded, time-limited state established after required authentication.
- **Authorization:** policy decision for a specific action/resource/request.

No one of these automatically implies the others.

## Process identity
PID, process name, and path alone are insufficient because of PID reuse, executable replacement, race conditions, and impersonation. Evidence must bind to a live process instance.

Platform evidence candidates:
- Windows: process handle, creation time, token/security context, executable identity and signature/hash as appropriate.
- Linux: peer credentials, PID start time, UID/GID, namespaces, executable identity.
- macOS: audit token and code-signing/designated requirement evidence.

Exact APIs and anti-race behavior require platform-specific design and tests.

## Policy decision inputs
A decision should include verified process identity, session state, workspace/resource identity, requested operation, explicit scope, grant duration, and relevant policy version. Missing or ambiguous inputs deny.

A path string is not authority. Resolve resource identity against authenticated workspace metadata/manifest. Avoid time-of-check/time-of-use races.

## Session and cache
Sessions have explicit creation, binding, expiry, and revocation. Identity/policy caches require short TTLs and invalidation on process exit, token/session change, policy update, or revoke. Cache entries must not outlive the evidence on which they depend.

## Descendants and plugins
Child processes, plugins, helper binaries, restarts, inherited handles, and handle passing require explicit rules. Do not automatically transfer authority to a process merely because it is a descendant or shares a path/name.

## Default deny
Failure to verify process, session, policy, resource identity, or scope must deny the request. Do not silently fall back to path-based allowlists or UI state.
