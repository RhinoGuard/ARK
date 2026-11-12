# Encrypted Storage, Working Set, and Crash Consistency

Status: ACCEPTED design direction. Exact on-disk format and transaction protocol are OPEN until prototype, fault-injection, and independent review.

## 1. Source of truth

Encrypted durable storage is the source of truth. A working copy is a temporary access representation, not the authoritative version. A successful write/commit must have a documented durability meaning.

## 2. Development working set (Model B)

- Expose only files needed by an authorized editor or tool.
- Prefer memory-backed working buffers where practical; use encrypted-backed scratch where persistent temporary storage is unavoidable.
- The editor must receive plaintext for active editing, so plaintext exposure exists while a file is in use.
- Minimize active-file count, lifetime, copies, and cache size.
- Never assume a tab closing means all handles, mappings, watchers, or background tasks are finished.
- Track handles and outstanding operations; release a working copy only when the backend can safely establish that it is no longer in use.
- Account for editor swap, local history, indexing, extensions, backups, antivirus, OS cache, pagefile/swap, hibernation, and crash dumps.
- Deletion of a working copy is cleanup, not a guarantee of secure physical erasure.

## 3. Write path and journaling

The intended write path is:
application write -> bounded buffer/change tracking -> authenticated encrypted journal record -> durable journal commit -> storage/chunk/index update -> checkpoint/compaction.

The precise ordering must be designed so that acknowledged commits survive the documented failure model. Do not acknowledge durability before required journal data and metadata have been flushed according to platform semantics.

- Batch/coalesce rapid writes for performance, but impose a bounded maximum delay and explicit flush barriers.
- Do not rely on shutdown hooks or file-close events for durability.
- Journal records need authenticated framing, sequence/generation binding, strict lengths, and replay/truncation handling.
- Journal compaction and checkpoint publication must be atomic or recoverable.
- A crash during commit must result in a deterministic recover/rollback outcome.
- Do not reuse nonces when replaying or retrying journal writes.

## 4. Chunk updates

Random-access chunking allows updates to affect only changed chunks rather than re-encrypting an entire large file. The design must handle writes crossing chunk boundaries, sparse files, truncation, extension, rename, concurrent writers, and file-generation changes.

Chunk size is not fixed yet; benchmark realistic editor workloads and large files before choosing it.

## 5. Crash and power-loss model

Distinguish:
- process crash;
- OS crash/reboot;
- VM power cut/fault injection;
- physical power loss and storage hardware behavior.

A shutdown callback cannot be relied on after sudden power loss. Recovery guarantees apply only to records whose durable commit boundary was reached. Changes still only in volatile buffers may be lost.

## 6. Consistency and concurrency

Define file-level and workspace-level transaction boundaries. Consider editor and runtime concurrent access, readers during writes, rename/unlink with open handles, file locks, mmap, directory enumeration, watchers, and conflict policy. Do not silently expose mixed generations.

## 7. Integrity and rollback

Authenticate headers, manifests, journal records, and chunks. Detect truncation and malformed indexes. AEAD alone does not prevent replay of an older valid state; rollback resistance requires a trusted generation anchor or an explicitly weaker documented guarantee.

## 8. Recovery behavior

On startup, validate the storage header and metadata, inspect the journal, authenticate records, recover only complete valid transactions, quarantine invalid/incomplete tails as specified, and never present unauthenticated data as valid. Recovery must be idempotent and tested under injected failures.

## 9. Performance requirements

Measure editor save bursts, small-file operations, large-file sequential reads, random reads, directory enumeration, concurrent access, cache hit rate, journal growth, and recovery time. Performance optimizations must preserve authentication and transaction semantics.
