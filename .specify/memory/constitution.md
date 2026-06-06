<!--
Sync Impact Report
Version change: template -> 1.0.0
Modified principles:
- Placeholder principles -> I. Read-Only Safety First
- Placeholder principles -> II. Rust Owns Filesystem, Persistence, and Query Logic
- Placeholder principles -> III. SQLite Is the Primary Data Layer
- Placeholder principles -> IV. Never Load the Full Tree in the Frontend
- Placeholder principles -> V. Performance Is a Product Requirement
- Placeholder principles -> VI. Scan Robustness Over Perfect Completeness
- Placeholder principles -> VII. MVP Scanner Before Advanced Scanner
- Placeholder principles -> VIII. Clear Separation of Product Layers
- Placeholder principles -> IX. Specification-Driven Development
- Placeholder principles -> X. No AI Features in Product Scope
Added sections:
- Project Identity
- Technical Standards
- Required Tauri Commands
- Required Events
- Required Data Model
- Required Database Schema
- Testing Standards
- UX Rules
- Performance Budgets
- Privacy and Security Rules
- Out-of-Scope Features
- Development Workflow Rules
Removed sections:
- Placeholder SECTION_2 and SECTION_3
Templates requiring updates:
- .specify/templates/plan-template.md: updated
- .specify/templates/spec-template.md: updated
- .specify/templates/tasks-template.md: updated
- .specify/templates/checklist-template.md: reviewed, no change required
- .specify/templates/constitution-template.md: reviewed, no change required
- .specify/templates/commands/*.md: not present
Follow-up TODOs: none
-->

# FastDisk Viewer Constitution

## Project Identity

FastDisk Viewer is a Windows desktop disk usage analyzer built with Tauri,
React, TypeScript, Rust, and SQLite. Its primary purpose is to scan selected
folders or drives, persist file metadata in SQLite, and visualize disk usage
through Tree View, largest files and folders tables, extension summaries, scan
errors, and Treemap.

This project is a read-only disk analyzer. The application MUST help users
understand disk usage. It MUST NOT delete, move, rename, compress, upload, or
modify files in the MVP.

## Core Principles

### I. Read-Only Safety First

The application MUST remain read-only in the current product phase. It MUST NOT
delete files, move files, rename files, modify file contents, compress files,
automatically clean anything, or expose UI controls that imply those actions.
The app MAY read filesystem metadata and display it. The only allowed OS-level
action is revealing or opening a file or folder location in Windows Explorer.

Rationale: The first version is focused on visibility and analysis, not cleanup.
This reduces risk, avoids accidental data loss, and keeps product scope focused.

### II. Rust Owns Filesystem, Persistence, and Query Logic

Rust MUST be the source of truth for filesystem scanning, SQLite access, folder
size aggregation, persistence, and production query behavior. Tauri commands
MUST expose controlled query APIs to React. React MUST NOT access SQLite
directly, perform filesystem scanning, or calculate final folder sizes for
production behavior.

Rationale: Rust is better suited for performance-sensitive filesystem
operations, large data processing, and safe concurrency. React remains the UI
layer.

### III. SQLite Is the Primary Data Layer

SQLite MUST be used for scan storage and MUST NOT be replaced by frontend
in-memory storage. Every scan MUST create a `scan_session`; every discovered
file or folder MUST be persisted as a `file_entry`; scan errors MUST be
persisted as `scan_errors`; and folder sizes MUST be persisted after
aggregation. React MUST request scoped data through Tauri commands. The app
MUST NOT send the full file tree to React in one response. Large result sets
MUST use limits, offsets, or scoped queries. SQLite writes during scanning MUST
use transactions and batching.

Rationale: Large disks may contain hundreds of thousands or millions of
entries. SQLite enables scalable querying, lazy loading, caching, and stable
performance.

### IV. Never Load the Full Tree in the Frontend

The frontend MUST render scoped views, not the complete filesystem tree. Tree
View MUST lazy-load folder children. Tables MUST use pagination, limits, or
virtualization. Treemap MUST request only top entries for the selected folder.
Search MUST be performed through SQLite-backed Rust commands. React state MUST
NOT contain the entire scan result. React MAY store UI state such as selected
folder, expanded folder IDs, active tab, filters, and sort options.

Rationale: Rendering or storing a full disk tree in React can freeze the UI and
consume excessive memory.

### V. Performance Is a Product Requirement

Performance MUST be treated as product behavior. Scanning MUST run outside the
UI thread. Scan progress events MUST be throttled and MUST NOT emit once per
file; progress events SHOULD be emitted roughly every 250-500 ms during active
scans. SQLite inserts MUST be batched. Expensive queries MUST be indexed. Tree
and table rendering MUST be virtualized where needed. Treemap rendering MUST cap
the number of visible nodes. Long-running Rust operations MUST NOT block the
Tauri frontend.

Rationale: The product competes on fast disk analysis. A slow or freezing UI
breaks the core value proposition.

### VI. Scan Robustness Over Perfect Completeness

The scanner MUST continue when it encounters inaccessible or problematic paths.
Permission errors, locked files, broken symlinks, very long paths where
practical, and Unicode or Arabic file names MUST NOT crash the scan. Skipped
paths MUST be counted. Scan errors MUST be recorded in SQLite. Users MUST be
able to inspect skipped or error paths after the scan.

Rationale: Real filesystems are messy. The app must produce useful results even
when some paths cannot be scanned.

### VII. MVP Scanner Before Advanced Scanner

The MVP MUST use a stable recursive Rust scanner before introducing NTFS MFT
scanning. NTFS MFT scanning MUST NOT be implemented in the MVP unless the
recursive scanner, SQLite persistence, Tree View, Largest Files, Largest
Folders, and Treemap are already working. Advanced NTFS/MFT scanning belongs to
a future phase. Scanner implementations SHOULD expose a common interface where
practical.

Rationale: Building the full product experience first reduces risk. MFT
scanning is powerful but adds complexity and Windows-specific low-level
behavior.

### VIII. Clear Separation of Product Layers

The codebase MUST maintain clear separation between scanning, storage, commands,
events, models, and UI. The Rust backend SHOULD be organized around `scanner`,
`db`, `repository`, `aggregator`, `commands`, `events`, and `models`
responsibilities. The React frontend SHOULD be organized around
`features/scan`, `features/tree`, `features/largest-files`,
`features/largest-folders`, `features/treemap`, `features/extensions`,
`features/errors`, `shared/components`, `shared/utils`, and `shared/api`.

Rationale: Clear architecture keeps the app maintainable as scanner
performance, visualization, and query features grow.

### IX. Specification-Driven Development

Requirements MUST be documented before implementation. Implementation MUST NOT
add major features outside the active specification. If a requested
implementation conflicts with this constitution, the constitution wins. Product
behavior belongs in `spec.md`; technical decisions belong in `plan.md`;
implementation steps belong in `tasks.md`. Low-level implementation details
MUST NOT be mixed into product requirements unless required for acceptance
criteria. Destructive file operations MUST NOT be invented unless this
constitution is amended.

Rationale: The project should remain controlled, focused, and suitable for
AI-assisted implementation without scope drift.

### X. No AI Features in Product Scope

The MVP product MUST NOT include AI assistant behavior, cloud AI calls,
AI-powered cleanup recommendations, or upload of file metadata to AI services.
Development MAY use external AI coding tools, but the product itself MUST NOT
include AI features.

Rationale: The MVP is a local, read-only disk analyzer. AI product features
introduce privacy, complexity, and scope concerns.

## Technical Standards

Rust code MUST prefer explicit error handling, avoid panics in production
scanning paths, use structured errors for scanner, database, and command
failures, run long-running work off the main UI thread, and use DTOs for
frontend-facing responses. Internal database models SHOULD remain separate from
frontend DTOs where useful.

SQLite MUST include `scan_sessions`, `file_entries`, and `scan_errors` tables or
an equivalent compatible design. Required query behavior includes
`get_children`, `get_largest_files`, `get_largest_folders`, `search_entries`,
`get_extension_summary`, `get_treemap_data`, and `get_scan_session`. SQLite use
MUST rely on prepared statements, batch inserts inside transactions, indexes for
session, parent, size, extension, path, and name, and bounded Tauri command
queries. WAL mode SHOULD be used unless there is a strong reason not to.

React code MUST use TypeScript, typed Tauri command wrappers, and async command
state management such as React Query or an equivalent. Local/Zustand-style state
MAY be used for UI state only. The frontend MUST NOT store the full scan result,
MUST use virtualization for large lists, MUST debounce search inputs, and MUST
keep components focused and feature-oriented.

The UI MUST provide path selection, scan progress, scan summary, Tree View,
Largest Files table, Largest Folders table, Treemap, Extension Summary, Scan
Errors view, and Reveal in Explorer action. The UI MUST NOT provide Delete,
Move, Rename, Cleanup, Auto-fix, or AI assistant controls.

## Required Tauri Commands

The backend MUST expose these commands or equivalent scoped alternatives:

```ts
start_scan(path: string): Promise<{ scanSessionId: number }>;
cancel_scan(scanSessionId: number): Promise<void>;
get_scan_session(scanSessionId: number): Promise<ScanSessionDto>;
get_children(input: {
  scanSessionId: number;
  parentId: number | null;
  sortBy: "size" | "name" | "modified_at" | "type";
  sortDirection: "asc" | "desc";
  limit: number;
  offset: number;
}): Promise<FileEntryDto[]>;
get_largest_files(input: {
  scanSessionId: number;
  limit: number;
  offset: number;
}): Promise<FileEntryDto[]>;
get_largest_folders(input: {
  scanSessionId: number;
  limit: number;
  offset: number;
}): Promise<FileEntryDto[]>;
search_entries(input: {
  scanSessionId: number;
  query: string;
  entryType: "all" | "file" | "folder";
  limit: number;
  offset: number;
}): Promise<FileEntryDto[]>;
get_extension_summary(scanSessionId: number): Promise<ExtensionSummaryDto[]>;
get_treemap_data(input: {
  scanSessionId: number;
  parentId: number | null;
  limit: number;
}): Promise<TreemapNodeDto>;
reveal_in_explorer(path: string): Promise<void>;
```

## Required Events

The backend MUST emit scan lifecycle events named `scan-progress`,
`scan-completed`, `scan-failed`, and `scan-cancelled`. Event emission MUST be
throttled and MUST NOT happen once per file.

## Required Data Model

Frontend-facing DTOs MUST include these fields or equivalent compatible fields:

```ts
type ScanSessionDto = {
  id: number;
  rootPath: string;
  status: "idle" | "scanning" | "completed" | "failed" | "cancelled";
  startedAt: string;
  completedAt?: string;
  totalFiles: number;
  totalFolders: number;
  totalSize: number;
  skippedItems: number;
  elapsedMs: number;
};

type FileEntryDto = {
  id: number;
  scanSessionId: number;
  parentId: number | null;
  name: string;
  path: string;
  size: number;
  isDirectory: boolean;
  extension?: string;
  depth: number;
  modifiedAt?: string;
  createdAt?: string;
  isSymlink: boolean;
  childCount: number;
  descendantCount: number;
};

type ExtensionSummaryDto = {
  extension: string;
  totalSize: number;
  fileCount: number;
  percentageOfScan: number;
};

type TreemapNodeDto = {
  id: number | null;
  name: string;
  path: string;
  size: number;
  isDirectory: boolean;
  percentageOfParent: number;
  children?: TreemapNodeDto[];
};
```

## Required Database Schema

The implementation MUST start with this schema or an equivalent compatible
design:

```sql
CREATE TABLE scan_sessions (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  root_path TEXT NOT NULL,
  status TEXT NOT NULL,
  started_at TEXT NOT NULL,
  completed_at TEXT,
  total_files INTEGER DEFAULT 0,
  total_folders INTEGER DEFAULT 0,
  total_size INTEGER DEFAULT 0,
  skipped_items INTEGER DEFAULT 0,
  elapsed_ms INTEGER DEFAULT 0
);

CREATE TABLE file_entries (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  scan_session_id INTEGER NOT NULL,
  parent_id INTEGER,
  name TEXT NOT NULL,
  path TEXT NOT NULL,
  size INTEGER NOT NULL DEFAULT 0,
  is_directory INTEGER NOT NULL,
  extension TEXT,
  depth INTEGER NOT NULL DEFAULT 0,
  modified_at TEXT,
  created_at TEXT,
  is_symlink INTEGER DEFAULT 0,
  child_count INTEGER DEFAULT 0,
  descendant_count INTEGER DEFAULT 0,
  FOREIGN KEY(scan_session_id) REFERENCES scan_sessions(id),
  FOREIGN KEY(parent_id) REFERENCES file_entries(id)
);

CREATE TABLE scan_errors (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  scan_session_id INTEGER NOT NULL,
  path TEXT NOT NULL,
  error_kind TEXT NOT NULL,
  error_message TEXT,
  created_at TEXT NOT NULL,
  FOREIGN KEY(scan_session_id) REFERENCES scan_sessions(id)
);

CREATE INDEX idx_file_entries_session ON file_entries(scan_session_id);
CREATE INDEX idx_file_entries_parent ON file_entries(parent_id);
CREATE INDEX idx_file_entries_size ON file_entries(size DESC);
CREATE INDEX idx_file_entries_extension ON file_entries(extension);
CREATE INDEX idx_file_entries_path ON file_entries(path);
CREATE INDEX idx_file_entries_name ON file_entries(name);
```

## Testing Standards

Backend tests MUST cover non-trivial scanner, database, aggregation, and query
logic. Coverage MUST include normal folders, empty folders, permission or access
errors where practical, skipped path recording, folder size aggregation,
SQLite insert/query operations, direct-child queries, largest file/folder
filtering, search, and extension summary calculations.

Frontend tests SHOULD cover scan screen rendering, progress updates from event
data, Tree View rendering of returned children, largest files table rows, empty
states, error states, debounced search behavior, and Treemap handling of empty
and non-empty data.

## UX Rules

Sizes MUST be formatted as human-readable values. Percentages SHOULD be shown
relative to parent folder or scan total depending on context. Long paths MUST be
visually truncated while remaining available through tooltip or copy behavior.
Empty states MUST explain the next user action. Errors MUST be understandable
and MUST NOT expose raw Rust panic messages.

Selecting a folder in Tree View SHOULD update the Treemap context. Treemap
SHOULD support drilldown into folders and breadcrumb navigation. Tables SHOULD
support sorting. Search results SHOULD make clear whether an item is a file or
folder.

## Performance Budgets

The UI MUST remain responsive during scanning. Progress updates SHOULD appear
during long scans. Opening a folder in Tree View MUST NOT require loading the
entire scan. Treemap SHOULD render a capped subset with a default maximum of 200
children. Largest Files SHOULD default to top 100. Search results SHOULD be
limited and debounced. No frontend view SHOULD intentionally request all rows
from a large scan.

## Privacy and Security Rules

The app MUST operate locally. It MUST NOT upload paths, filenames, metadata, or
scan results. It MUST NOT require internet access for core functionality. It
MUST NOT include analytics in the MVP. SQLite scan data MUST remain local. Any
future telemetry requires an explicit product specification and constitution
amendment.

## Out-of-Scope Features

The following MUST NOT be implemented unless this constitution is amended:
delete file, move file, rename file, move to Recycle Bin, cleanup
recommendations, AI assistant, cloud sync, online account, file upload,
background service, scheduled scans, duplicate removal, registry cleaner, and
system optimizer.

## Development Workflow Rules

Before coding a major feature, the team MUST confirm the requirement exists in
the spec, does not violate read-only scope, identifies required backend commands
or frontend components, documents database impact, and documents performance
impact.

During coding, commits SHOULD stay focused. Unrelated features MUST NOT be
mixed. Deletion or mutation behavior MUST NOT be introduced. DTOs between Rust
and TypeScript SHOULD be typed. Tests MUST be added for non-trivial logic.
Indexes MUST be added before relying on large queries.

A feature is complete only when it satisfies acceptance criteria, has basic
tests where practical, handles empty and error states, avoids loading unbounded
data into React, does not violate read-only safety, and aligns with this
constitution.

## Governance

This constitution overrides agent suggestions, convenience shortcuts,
unspecified implementation ideas, out-of-scope feature requests, and destructive
file operation suggestions.

Amendments MUST be intentional and MUST document what changed, why it changed,
which specs, plans, or tasks are affected, whether MVP scope changed, and the
required semantic version bump. MAJOR versions represent backward-incompatible
governance changes or permission for previously prohibited product behavior.
MINOR versions add principles, sections, or materially expanded guidance. PATCH
versions clarify wording without changing governance intent.

Every feature plan MUST include a Constitution Check. Every task list MUST
include work needed to satisfy applicable read-only, SQLite, bounded-query,
performance, privacy, and testing requirements. Reviews MUST reject work that
violates this constitution unless an amendment has already been ratified.

**Version**: 1.0.0 | **Ratified**: 2026-06-07 | **Last Amended**: 2026-06-07
