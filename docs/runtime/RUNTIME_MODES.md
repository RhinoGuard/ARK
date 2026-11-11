# ARK Runtime Modes

Status: ACCEPTED separation of Development Mode and Execution Mode. Platform implementation and exact UI flow remain subject to prototypes.

## 1. Why two modes

Development tools and runtime applications have different access patterns. A developer edits a small active set of files; a running web application may dynamically load many files and dependencies. One access policy should not force both patterns into the same behavior.

## 2. Development Mode

Purpose: allow authorized editors/tools to modify only files they need.

Expected behavior:
- establish a scoped Development Session;
- verify application identity and policy;
- make a file available when requested and authorized;
- maintain a bounded active working set;
- persist changed data through authenticated encrypted journaling and transactional chunk updates;
- coalesce rapid writes within bounded latency;
- flush on explicit barriers and lifecycle events;
- release working copies only when handles and operations are safely quiescent;
- recover committed changes after process/OS crash according to the documented durability contract.

Do not assume closing an editor tab is equivalent to releasing all file handles. Do not promise that sudden power loss preserves volatile, uncommitted changes.

## 3. Execution Mode

Purpose: provide runtime access to resources required by an authorized application without trying to statically enumerate all dynamic dependencies.

The user configures an Execution Profile:
- application/process identity requirements;
- workspace/project;
- allowed directories or resource scopes;
- allowed operations (for example read-only or read/write);
- session duration/expiry;
- explicit exclusions for sensitive areas;
- warnings describing the breadth of the granted scope.

At runtime, each supported filesystem request is mediated against verified process evidence, current session, and policy. A directory scope can expose many files; ARK must explain this risk.

Dynamic paths do not bypass scope checks. The system resolves requested paths against a trusted workspace namespace and prevents traversal, link escape, and alternate-path bypass to the extent supported by the backend.

## 4. Application compatibility

A shared filesystem mediation layer aims to avoid per-application adapters for ordinary filesystem semantics. It does not guarantee every application works. Applications may use mmap, unusual locking, file watchers, direct OS APIs, child processes, plugins, privileged helpers, or assumptions not supported by a backend.

Maintain a compatibility matrix by OS, application, version, operation, and known limitation. Never claim “works with all apps.”

## 5. Process identity and descendants

Process name/path/PID alone are insufficient. Bind access to platform identity evidence with freshness and lifetime checks. Decide explicitly whether child processes inherit scope, require separate grants, or are denied. Track process restarts and PID reuse. Do not blindly trust an editor's plugin or child process as the editor itself.

## 6. Revocation and existing handles

Revocation blocks new mediated operations after policy/session invalidation. Existing handles, mappings, cached plaintext, and copied data may persist. Backend-specific close/terminate behavior must be explicit and user-visible; do not claim instantaneous universal revocation.

## 7. MVP direction

Windows-first prototype. Validate the real PHP/VS Code development flow and Apache execution flow. Start with a constrained, testable backend and record compatibility limitations. A VFS backend is a real filesystem implementation, not a passive watcher.
