# Research: FastDisk Viewer MVP

## Decision: Keep MVP Scanner Recursive and Read-Only

**Rationale**: The constitution requires a stable recursive scanner before
advanced NTFS/MFT scanning. Recursive scanning is sufficient to deliver the MVP
experience and is safer to test across normal folders, protected folders,
symlinks, hidden files, and Unicode paths.

**Alternatives considered**: NTFS MFT scanning was rejected for MVP because it
adds low-level Windows-specific complexity and is explicitly out of scope.

## Decision: Do Not Follow Symlinks or Junctions in MVP

**Rationale**: Link-like folders can create cycles, duplicate accounting, or
surprising traversal outside the selected root. Recording link status while not
following links keeps scans robust and predictable.

**Alternatives considered**: Following links was rejected because cycle
detection and scope boundaries complicate the first release.

## Decision: One Active Scan at a Time

**Rationale**: The spec assumes one active scan in MVP. This keeps progress,
cancellation, database writes, and UI state easier to reason about while still
supporting multiple completed scan sessions over time.

**Alternatives considered**: Multiple simultaneous scans were rejected because
they add scheduling, isolation, and user-confusion risks without being required
for MVP value.

## Decision: Persist Scan Data Locally and Query Bounded Views

**Rationale**: Large scans may contain hundreds of thousands or millions of
entries. Persisting scan sessions, entries, and errors locally supports previous
scan browsing and keeps React from owning the full tree.

**Alternatives considered**: In-memory frontend scan state was rejected because
it violates the constitution and does not scale to large folders.

## Decision: Batch Writes and Aggregate After Traversal

**Rationale**: Batched inserts inside transactions reduce write overhead during
large scans. Folder sizes can be calculated after traversal by accumulating
descendant file sizes and updating folder rows in a controlled phase.

**Alternatives considered**: Updating folder sizes on every discovered file was
rejected because it creates excessive writes and contention.

## Decision: Use Scoped Query Contracts for Every View

**Rationale**: Tree children, largest files/folders, search, extension summary,
scan errors, previous scans, and treemap each have bounded query contracts. This
keeps UI memory stable and makes performance easier to test.

**Alternatives considered**: A single "get full scan" query was rejected because
it violates frontend memory safety and bounded-data rules.

## Decision: Throttle Scan Events

**Rationale**: Users need proof that scanning is active, but one event per file
would flood the frontend. Throttling progress to roughly every 250-500 ms meets
the constitution and the success criterion of visible updates within 1 second.

**Alternatives considered**: Per-file events were rejected for performance.
Only completion events were rejected because long scans would feel stuck.

## Decision: Keep Reveal in Explorer as the Only OS-Level Action

**Rationale**: Reveal/open-location behavior is allowed by the constitution and
helps users manually inspect items outside the app. It must handle missing paths
with friendly errors.

**Alternatives considered**: Delete, move, rename, cleanup recommendations, and
Recycle Bin actions were rejected as out of scope.

## Decision: Use Virtualized Tables and Lazy Tree Rendering

**Rationale**: Large folders and top-result views must remain responsive. Tree
View loads children on demand, tables use bounded result sets plus
virtualization/pagination, and Treemap uses a capped top-N response.

**Alternatives considered**: Rendering all children or all search matches was
rejected because it risks freezing the UI.

## Decision: Preserve Partial Results on Cancellation

**Rationale**: The spec assumes cancelled scans are marked cancelled and partial
results may remain clearly labeled. This gives users feedback and avoids
ambiguous session state.

**Alternatives considered**: Deleting partial results was rejected because it
adds destructive database behavior and removes useful diagnostic information.
