# GitHub Copilot Instructions

Read the repository-root AGENTS.md before making suggestions or edits. AGENTS.md is the primary instruction source for all AI coding agents.

For ARK, also read docs/README.md, docs/architecture/ARCHITECTURE.md, docs/architecture/SECURITY_INVARIANTS.md, and docs/security/THREAT_MODEL.md before substantial work. Read the relevant subsystem specification and ADRs before changing that subsystem.

Do not infer that a proposed feature is implemented or accepted. Preserve explicit decision status. Do not bypass the broker, policy engine, identity verification, session checks, authenticated storage, or key hierarchy. Do not invent cryptography. Do not authorize by path/name/PID alone. Default deny and fail closed.

Work on a feature branch, keep changes scoped, add tests, update docs/ADRs, and report tests actually run, security impact, limitations, and unresolved questions. Never claim a security property without implementation evidence and tests.
