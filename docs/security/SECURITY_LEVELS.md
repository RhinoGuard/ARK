# ARK Operation Security Levels

Status: ACCEPTED conceptual model. Per-operation mapping and authentication mechanisms require implementation-specific review.

## Purpose

Security levels classify the sensitivity of an operation and the strength of authentication/confirmation required before it is authorized. They do not represent different encryption strengths and are not three separate workspace passwords.

## Standard

For routine operations within an already authorized session, such as ordinary file reads/writes, browsing permitted project metadata, and normal development activity.

Requirements:
- valid session and current policy;
- verified subject identity appropriate to the platform;
- scope and operation must match the grant;
- no extra prompt when an existing grant is valid and sufficiently scoped.

## Elevated

For sensitive but recoverable operations, such as changing application permissions, exporting a project/workspace, importing a package, or changing security settings.

Requirements:
- step-up authentication or trusted-device confirmation according to workspace policy;
- explicit display of the target workspace/project, operation, scope, and destination where relevant;
- short-lived authorization bound to the exact operation and target;
- audit event without recording secrets or plaintext.

Proof of intent (for example, typing a project name) is not authentication. It may supplement, but cannot replace, step-up authentication.

## Critical

For high-impact key and recovery operations, such as rotating recovery material, changing key wrappers, revoking a primary trusted device, or destructive workspace operations.

Requirements:
- stronger step-up authentication and explicit confirmation;
- operation-specific authorization with short expiry;
- verify-before-commit and atomic state transition where possible;
- recovery and rollback plan where applicable;
- audit and user notification where appropriate.

## Policy rules

- Every operation must have an explicit level mapping; no implicit fallback.
- A level does not grant permission. Identity, session, scope, and policy checks still apply.
- The UI cannot assign itself a lower level or bypass step-up.
- A valid password alone does not make an otherwise unauthorized operation allowed.
- Re-authentication proves control of a credential under the threat model; it does not prove intent by itself.
- Avoid repeated prompts when an exact, unexpired authorization already covers the operation.
