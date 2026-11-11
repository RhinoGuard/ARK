# Encrypted Storage Design

**Status:** Requirements baseline. Binary format and transaction protocol are not finalized.

## Goals
Authenticated encrypted storage, random-access chunk updates, crash-consistent metadata, recoverable interrupted writes, and portable cross-platform interpretation.

## Logical elements
- Versioned storage header.
- Authenticated workspace metadata and manifest.
- File identity independent of user-controlled path strings.
- Chunk map and authenticated chunk records.
- Encrypted write-ahead journal (WAL).
- Transaction/generation metadata and checkpoints.
- Locking and snapshot metadata.

Exact field layout, serialization, integer widths, limits, and canonical encoding require a storage-format ADR.

## Random-access chunking
Only changed chunks should need re-encryption for a small edit. Chunk size is a benchmarked design parameter, not a guess. The protocol must bind each chunk to workspace/file identity, chunk index, generation/context, and crypto suite. It must detect missing, duplicated, reordered, truncated, or substituted chunks. Metadata updates must be atomic with respect to the committed chunk set.

## WAL and durability
The durable encrypted journal is the source of recoverable pending transactions. A commit is acknowledged only after the defined ordering has been flushed durably:
1. Write authenticated encrypted data/journal records.
2. Flush the required records according to platform guarantees.
3. Publish authenticated metadata/commit marker atomically.
4. Flush metadata and directory entries as required by the backend.
5. Only then report durable success.

The exact ordering must be validated per filesystem/platform. A shutdown hook is cleanup, not a durability guarantee. Long debounce timers must not be the only durability mechanism.

## Crash recovery
On restart, validate the header, manifest, journal records, authentication tags, transaction sequence, and commit markers. Recover only transactions meeting the documented commit protocol. Preserve the last known good committed state; quarantine or report corrupt/incomplete state rather than silently accepting it.

Fault-injection tests must cover interruption at every write/flush/rename boundary, partial records, torn metadata, disk-full, concurrent operations, and repeated recovery.

## Consistency and concurrency
Define lock granularity, writer ownership, snapshot semantics, generation ordering, conflict handling, and behavior for concurrent editors/processes. Never resolve conflicting writes silently. Snapshot export must correspond to one consistent durable generation, including relevant WAL state.

## Rollback and versioning
AEAD does not prevent replay of an older valid storage image. Define an anti-rollback strategy and its trust assumptions. Storage format version, metadata schema, crypto suite, and application version are independent version dimensions. Migration must be atomic and recoverable.

## Plaintext boundary
Plaintext working buffers may exist in RAM and OS-managed areas. The design must not claim that pagefile/swap, hibernation, crash dumps, editor extensions, indexers, or application caches are impossible.
