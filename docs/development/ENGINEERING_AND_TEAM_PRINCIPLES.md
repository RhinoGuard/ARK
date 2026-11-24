# Engineering and Team Principles

**Status:** Proposed team-wide working principles  
**Scope:** Product direction, engineering execution, security, collaboration, and project stewardship  
**Audience:** Maintainers, subsystem owners, contributors, reviewers, and AI-assisted contributors

## Purpose

ARK is a security-sensitive engineering project. Its quality depends not only on code, but also on the decisions, evidence, review practices, collaboration, and operational discipline surrounding that code.

These principles establish a shared standard for how the team makes decisions and delivers work. They are intended to support sound judgment—not replace technical specifications, accepted Architecture Decision Records (ADRs), security invariants, repository policies, or explicit maintainer decisions.

The principles below are an adaptation of lessons drawn from project and open-source engineering practice. They should be applied in context, documented when they affect project decisions, and improved through experience.

## The 18 Principles

### 1. Product First — Optimize for the Real User and the Intended Outcome

**Principle.** Engineering activity exists to deliver a useful, coherent, and trustworthy product—not merely to produce code, commits, or technical novelty.

**Application.**
- Begin work by identifying the user problem, expected behavior, and intended outcome.
- Evaluate architecture and implementation choices against product requirements, security constraints, usability, maintainability, and lifecycle cost.
- Avoid building abstractions, features, or integrations solely because they are technically interesting.
- Prefer a smaller, complete, verifiable capability over a broad collection of disconnected or unreliable features.
- Keep the product's actual maturity and limitations visible.

**ARK context.** A feature is not successful simply because its Rust implementation exists. It must fit ARK's security model, preserve the relevant invariants, integrate with the intended workflow, and have evidence appropriate to its claimed maturity.

### 2. Evidence First — Make Claims Traceable to Facts

**Principle.** Decisions, status reports, reviews, and security claims must be grounded in observable evidence rather than assumptions, confidence, or activity signals.

**Application.**
- Support claims with relevant source code, tests, logs, reproducible steps, specifications, CI results, or reviewed artifacts.
- Distinguish confirmed facts from hypotheses, estimates, interpretations, and unresolved questions.
- When reporting progress, identify what changed and how it was verified.
- When evidence is missing, state that explicitly; do not present uncertainty as confirmation.
- Prefer reproducible demonstrations over screenshots or verbal assurances when stronger evidence is available.

**ARK context.** A passing unit test does not prove end-to-end security. A merged PR does not by itself prove that a subsystem is complete. Claims must match the scope and strength of the evidence.

### 3. Root Cause Over Symptoms — Fix the Systemic Failure

**Principle.** When a defect or process failure occurs, investigate why it happened and address the underlying cause, not only the visible symptom.

**Application.**
- Reproduce and characterize the failure before changing code when practical.
- Trace the failure to its source: design assumption, API boundary, state transition, missing validation, process gap, or test weakness.
- Avoid repeated patches that conceal the same underlying defect.
- Where appropriate, add regression tests, invariant checks, documentation, or automation to prevent recurrence.
- Keep corrective actions proportional to the risk and complexity of the failure.

**ARK context.** If a security boundary is repeatedly misunderstood or violated, improve the contract, types, tests, or review checklist rather than relying indefinitely on reviewers to notice the same issue manually.

### 4. Scoped Work — Define a Clear, Reviewable Unit

**Principle.** Every task and pull request should have a bounded scope, a clear purpose, and an outcome that can be reviewed independently.

**Application.**
- State the problem, expected behavior, boundaries, dependencies, and acceptance criteria before implementation.
- Keep changes focused; separate unrelated refactoring, feature work, and cleanup.
- Identify what is explicitly out of scope.
- Split work when doing so improves ownership, reviewability, testing, or integration.
- Avoid splitting work so finely that interfaces and coordination cost outweigh the benefit.

**ARK context.** Security-sensitive work should identify affected trust boundaries, invariants, interfaces, and relevant ADRs. Cross-subsystem changes require coordination with the owners of the affected contracts.

### 5. Tests Prove Done — Completion Requires Appropriate Verification

**Principle.** A task is complete when its agreed behavior has been implemented and verified to an appropriate standard—not when coding stops or a commit is pushed.

**Application.**
- Define verification expectations as part of acceptance criteria.
- Test success paths, failure paths, boundary conditions, and relevant misuse cases.
- Select test types according to risk: unit, integration, negative, property-based, fuzz, compatibility, fault-injection, or recovery tests.
- Report tests that were not run, could not run, or only partially cover the requirement.
- Do not overstate what a test environment demonstrates.

**ARK context.** Distinguish simulated faults, process-crash tests, virtual-machine power-cut tests, and physical hardware testing. Passing one category must not be described as proof of another.

### 6. Document Decisions — Preserve Rationale, Not Just Conclusions

**Principle.** Important decisions must remain understandable after the people who made them have moved on.

**Application.**
- Record architectural and security-relevant decisions in the appropriate ADR or specification.
- Explain context, constraints, alternatives, rationale, consequences, and migration implications.
- Mark decisions clearly as accepted, proposed, open, deferred, rejected, or superseded according to repository conventions.
- Update documentation when implementation or behavior changes.
- Treat documentation as a maintained engineering artifact, not a one-time announcement.

**ARK context.** Documentation is a shared baseline for contributors and AI coding agents. It should reduce conflicting interpretations and preserve the reasoning behind security boundaries and subsystem contracts.

### 7. Automation — Make Correctness Repeatable

**Principle.** Repeated, objective checks should be automated where automation is reliable and cost-effective.

**Application.**
- Automate formatting, linting, tests, build checks, dependency checks, and enforceable repository conventions.
- Prefer machine-checkable constraints for rules that can be expressed precisely.
- Keep automation understandable, maintainable, and aligned with the actual policy.
- Investigate flaky or misleading checks instead of normalizing their failure.
- Use automation to reduce routine human error and review fatigue—not to eliminate expert judgment.

**ARK context.** Architectural boundaries, public API restrictions, dependency direction, and security-sensitive regression checks should be enforced mechanically where feasible, while threat analysis and design review remain human responsibilities.

### 8. Learn from Failures — Convert Experience into Better Practice

**Principle.** Failures should produce understanding and durable improvement, not blame or repeated surprise.

**Application.**
- Describe failures accurately and respectfully.
- Identify contributing conditions and missed signals.
- Extract practical lessons and assign proportionate corrective actions.
- Share knowledge that could help other contributors avoid the same problem.
- Review whether the corrective action actually reduces recurrence.

**ARK context.** A failed review, broken integration, missed deadline, or test gap is useful feedback when it leads to a stronger contract, clearer ownership, improved tooling, or better risk awareness.

### 9. Clear Ownership — Make Responsibility Explicit

**Principle.** Every meaningful area of work needs an identifiable owner responsible for coordination, progress, and technical follow-through.

**Application.**
- Define subsystem, task, and decision ownership clearly.
- Make ownership visible in repository documentation, issues, or project planning.
- Respect established ownership; coordinate before changing another owner's accepted design or implementation.
- Ownership includes communicating blockers and maintaining the quality of the deliverable.
- Ownership does not mean unilateral authority over shared interfaces, security policy, or project-wide decisions.

**ARK context.** Subsystem owners lead their assigned areas, while cross-cutting architecture and security decisions follow the project's review and approval process. Role boundaries must not be silently reassigned.

### 10. Outcome-Based Management — Measure Progress by Delivered Capability

**Principle.** Manage work by meaningful outcomes and verified milestones, not by activity volume or arbitrary percentages.

**Application.**
- Define milestones in terms of capabilities, artifacts, or validated behavior.
- Track implementation, integration, testing, and verification separately where useful.
- Use estimates as planning aids, not as proof of completion.
- Report blockers and changed assumptions early.
- Reassess plans when evidence or capacity changes.

**ARK context.** “The subsystem is 70% complete” is useful only when the percentage has a defined basis. A clearer report identifies completed interfaces, remaining implementation, unresolved security gates, and evidence still required.

### 11. Human Accountability — AI Assistance Does Not Transfer Responsibility

**Principle.** A human contributor remains accountable for code, documentation, decisions, and changes submitted under their authorship, regardless of whether AI tools assisted.

**Application.**
- Understand and review generated or suggested changes before submission.
- Verify factual claims, dependencies, licensing, security properties, and test results.
- Do not treat AI output as authoritative evidence or as a substitute for source inspection.
- Disclose AI assistance when repository policy, licensing, or project practice requires it.
- Maintain a clear human owner for every accepted change.

**ARK context.** AI agents must follow repository instructions, accepted ADRs, security invariants, and scope boundaries. Their output must be reviewed and tested like any other contribution, with particular care around cryptography, authorization, recovery, and storage.

### 12. Protect Maintainer Attention — Make Collaboration Efficient and Respectful

**Principle.** Maintainer time and attention are finite project resources. Collaboration should make review and coordination easier, not create avoidable overhead.

**Application.**
- Provide context, concise summaries, links, and specific questions when requesting review.
- Search existing discussions, documentation, issues, and code before raising duplicate questions or proposals.
- Avoid unnecessary pings, fragmented updates, and repeated requests without new information.
- Prepare changes so reviewers can understand the rationale, scope, and evidence.
- Respect focus time, availability, and reasonable response expectations.

**ARK context.** A well-prepared PR with a clear risk summary and test evidence is more valuable than repeated status messages. Urgent security concerns should use the designated escalation path.

### 13. Trust Before Strictness — Build Reliability Through Clarity and Consistency

**Principle.** A healthy team begins with clear expectations, good-faith collaboration, and trust, while maintaining proportionate safeguards for project risk.

**Application.**
- Assume good faith unless evidence indicates otherwise.
- Set expectations explicitly and apply them consistently.
- Give contributors room to own decisions within their authority.
- Address missed commitments privately and factually where appropriate.
- Increase controls based on demonstrated risk, responsibility, and the sensitivity of the work—not on arbitrary suspicion.

**ARK context.** Trust is not a substitute for security review or access control. It is a management foundation that helps contributors take responsibility while technical safeguards protect the project.

### 14. Remove Blockers — Enable Contributors Before Reassigning Work

**Principle.** When progress stalls, first understand the obstacle and help remove it; do not immediately interpret delay as lack of commitment.

**Application.**
- Ask what is blocking progress: unclear requirements, missing access, dependency work, technical uncertainty, capacity, or competing obligations.
- Provide the missing context, decision, review, or interface when possible.
- Adjust scope or sequencing when circumstances justify it.
- Agree on a realistic next step and communicate revised expectations.
- Reassign work when necessary for project health, but do so transparently and respectfully.

**ARK context.** Subsystem owners should be supported with stable interfaces, prepared issues, timely decisions, and access to reviewers. The goal is to develop independent ownership rather than permanent dependence on the founder.

### 15. Honest Releases — Communicate Maturity and Limitations Precisely

**Principle.** Project status and release claims must accurately reflect what exists, what has been tested, and what remains uncertain.

**Application.**
- Use maturity labels consistently, such as prototype, experimental, MVP, beta, or production-ready.
- Separate planned, designed, implemented, tested, reviewed, and independently verified states.
- Document known limitations, unsupported environments, and incomplete security properties.
- Avoid marketing language that exceeds the available evidence.
- Update status when new findings materially change the assessment.

**ARK context.** Architecture documents and foundation tests do not establish end-to-end protection. ARK must not claim production security until the relevant implementation, threat analysis, testing, and review gates have been satisfied.

### 16. Reproducibility — Make Results Independently Verifiable

**Principle.** Important engineering results should be repeatable by another contributor using documented inputs, versions, and procedures.

**Application.**
- Record relevant toolchain, dependency, configuration, and environment details.
- Provide deterministic tests and reproducible failure steps where feasible.
- Keep build and test instructions current.
- Avoid relying on undocumented local state or a single contributor's machine.
- Preserve appropriate artifacts and logs without exposing secrets or sensitive data.

**ARK context.** Reproducible builds and tests, pinned or controlled dependencies, documented test environments, and safe fixtures support reliable security review and reduce integration ambiguity.

### 17. Attribution and Licensing — Respect the Provenance of Every Contribution

**Principle.** Open-source collaboration requires accurate attribution and compliance with the licenses and terms governing code, documentation, dependencies, and other assets.

**Application.**
- Preserve existing copyright and license notices as required.
- Verify the license and provenance of imported code, examples, assets, and dependencies.
- Do not copy material into ARK without understanding its applicable terms.
- Record attribution and third-party notices in the appropriate location.
- Resolve licensing uncertainty before integrating or distributing affected material.

**ARK context.** License compliance is part of project integrity and long-term maintainability. Dependency and contribution reviews should consider provenance as well as technical quality.

### 18. Continuous Improvement — Improve the Project Without Exhausting the Team

**Principle.** A strong project improves its product, engineering system, and team practices continuously at a sustainable pace.

**Application.**
- Review processes periodically and change them when evidence shows they are ineffective.
- Prefer small, durable improvements over disruptive process changes without clear benefit.
- Balance delivery, security, technical debt, learning, and contributor capacity.
- Preserve knowledge and reduce single-person dependencies.
- Treat sustainability as a quality requirement, not as an excuse for lowering standards.

**ARK context.** The long-term objective is a project whose architecture, documentation, tests, ownership, and workflows allow contributors to deliver secure improvements consistently—without relying on constant founder intervention or sacrificing security for speed.

## Applying These Principles

These principles guide judgment and collaboration. They do not override:

1. Accepted ADRs and explicit architectural decisions.
2. Security invariants, threat-model requirements, and subsystem specifications.
3. Repository contribution, review, and release policies.
4. Applicable licensing and legal obligations.
5. Explicit project-owner decisions within the established governance model.

When principles appear to conflict, document the trade-off, consider the risk and evidence, and involve the relevant owners. A principle should lead to a practical improvement in decisions or behavior—not become a slogan or an inflexible rule detached from context.

## Review and Evolution

This document is intended to evolve with ARK. Proposed changes should explain the lesson or evidence motivating them, the behavior they are expected to improve, and any effect on existing project policies. Changes should be reviewed through the normal documentation pull-request process.
