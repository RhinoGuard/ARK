# Security Model

## Objectives
- Protect workspace data at rest using authenticated encryption.
- Base access decisions on verified process/device/session identity and explicit policy.
- Deny by default when identity, policy, or state is missing, stale, or invalid.
- Preserve integrity and recoverability across interrupted writes.
- Provide explicit recovery and portable encrypted backup paths.
- Record security events without secrets or plaintext content.

## Principles
1. Security by default.
2. Default deny.
3. Least privilege over process, resource, operation, and time.
4. Identity is not authorization.
5. UI is not a security authority.
6. Cached identity/policy decisions have bounded lifetime and invalidation.
7. Fail securely; no silent downgrade.
8. Explicit trust boundaries.
9. Optimize without weakening security properties.
10. No convenience bypass.

## Policy security levels
These levels classify policy and step-up requirements; they are not separate encryption passwords.
- **Standard:** routine actions in a valid session.
- **Elevated:** export/import, permission changes, sensitive settings, Recovery File operations; step-up authentication.
- **Critical:** Recovery Key rotation, primary key changes, workspace deletion, ownership changes, primary trusted-device revocation; stronger authentication and explicit confirmation.

The policy engine decides the required level, not a UI label. Mechanisms and thresholds remain to be specified.

## Identity, session, authorization
Do not rely only on PID, process name, or executable path. Windows evidence may include process handle/creation time, token, executable identity/signature; Linux peer credentials, process start time, UID/GID and namespace; macOS audit token and code-signing identity.

Identity, session validity, and authorization are separate. Bind grants to process identity, session, resource scope, operation, and duration. Cache entries require short explicit lifetimes and invalidation.

## Revocation
Revocation should prevent new mediated operations after observed by the broker. Existing handles, mappings, cached pages, child processes, and in-flight operations may continue unless a backend can invalidate/terminate them. Never promise instantaneous revocation without proof.

## Sensitive operations
Export, import, recovery, password change, deletion, Recovery Key rotation, permission changes, and key changes require explicit policy classification and appropriate step-up authentication. A confirmation phrase/project-name challenge proves intent, not identity.

## Audit
Record decisions and lifecycle events without passwords, keys, recovery material, tokens, or file contents. A local log is not tamper-proof against a privileged host attacker.

## Assurance
A complex system cannot credibly promise zero bugs. Release objective: no known unresolved Critical/High vulnerability, required tests passing, and independent review of sensitive changes. These controls reduce risk; they do not prove absence of all defects.
