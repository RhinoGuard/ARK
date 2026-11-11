# ARK Commit Message Conventions

**Status:** Required contributor convention. Pull-request CI validates commit headers.

## 1. Header format

Use Conventional Commits:

    type(scope): imperative summary

The scope is strongly recommended for subsystem changes. It may be omitted for genuinely cross-cutting changes such as a repository-wide CI or documentation convention. A breaking change is marked with an exclamation mark before the colon:

    type(scope)!: imperative summary

Keep the summary at or below 72 characters. Use lowercase type and scope names. Write the summary in English, begin with an imperative verb, and do not end it with a period.

## 2. Supported types

| Type | Use |
|---|---|
| feat | Add a capability or externally observable behavior |
| fix | Correct a defect |
| refactor | Restructure code without changing intended behavior |
| perf | Improve performance without weakening security |
| test | Add or change tests |
| style | Formatting or non-semantic style changes |
| docs | Documentation-only changes |
| build | Build system, Cargo workspace, or toolchain changes |
| ci | Continuous integration and validation automation |
| chore | Maintenance that does not fit another type |
| security | A security-focused change or mitigation |
| revert | Revert a previous change |

## 3. Scope vocabulary

Prefer a scope that identifies the owning subsystem:
- core
- runtime
- storage
- crypto
- identity
- session
- policy
- broker
- filesystem
- platform-windows
- platform-linux
- platform-macos
- package
- recovery
- audit
- workspace
- ci
- docs

A scope is not a claim that the subsystem is production-ready. It only identifies the area changed.

## 4. Good examples

    feat(core): add typed workspace identifiers
    feat(runtime): model distinct runtime modes
    fix(storage): reject incomplete committed generations
    test(policy): cover stale-session denial
    security(broker): reject ambiguous process evidence
    docs(architecture): record crate dependency boundaries
    ci: validate the Rust workspace on supported platforms
    feat(core)!: replace the public identifier representation

## 5. Bad examples

Avoid subjects such as:
- update code
- fix bug
- changes
- final
- wip
- test 2
- improve security

They do not provide a useful, searchable explanation of the change.

## 6. Commit quality rules

- One commit represents one coherent, reviewable change.
- A commit should build and pass the tests relevant to that change whenever practical.
- Do not commit temporary debug code, broken intermediate states, generated build output, secrets, or unrelated formatting.
- Do not hide a behavior change inside a documentation or refactor commit.
- Keep mechanical formatting separate from semantic changes when doing so improves review.
- Use the commit body to explain why the change exists, important design choices, security invariants affected, and validation actually performed.
- For breaking changes, explain migration and compatibility consequences in the body and include a BREAKING CHANGE footer when appropriate.

## 7. Pull-request merge policy

For core architecture and other substantial engineering PRs, use Rebase and Merge only when every retained commit is coherent, understandable, and suitable for the permanent main history. Do not preserve fixup, WIP, or known-broken commits merely to retain activity history.

Use Squash Merge for exploratory branches, small maintenance PRs, or histories that do not meet the quality bar for individually retained commits. The PR itself remains available for discussion and review.

CI validates the commit headers in a pull request. Human review remains responsible for whether the commits are logically independent and whether their bodies explain consequential decisions.
