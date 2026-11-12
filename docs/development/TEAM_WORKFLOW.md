# Team Workflow

ARK is being developed by a small, experienced three-person team. Keep process lightweight but security-critical review mandatory.

## Responsibility areas
Working ownership areas:
- Security architecture and cryptography.
- Systems engineering and core runtime.
- Offensive security and verification.

These are primary focus areas, not silos. Offensive review must participate from initial design, threat modeling, implementation planning, and testing—not only at final audit.

## Review rules
- No author self-approval for security-sensitive changes.
- Pair-review crypto, key lifecycle, storage/WAL, recovery, identity/policy, IPC, filesystem mediation, and release trust.
- Keep PRs small and focused.
- Review the complete trust boundary, not only the changed function.
- Record accepted design decisions in ADRs.
- Track requirements to implementation and tests.

## AI-assisted development
AI agents may inspect, explain, draft, and implement bounded changes. They do not approve their own code, decide security policy, or replace human threat analysis. Follow [AGENTS.md](../../AGENTS.md). Every agent task ends with actual validation evidence and a residual-risk summary.

## Definition of done
A change is done only when code, tests, docs, compatibility, security review (when required), and implementation status agree. "Compiles" alone is not done.
