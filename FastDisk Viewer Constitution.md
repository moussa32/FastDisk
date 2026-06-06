# FastDisk Viewer Constitution

## Project Identity

**Project name:** FastDisk Viewer  
**Project type:** Windows desktop disk usage analyzer  
**Primary stack:** Tauri + React + TypeScript + Rust + SQLite  
**Primary purpose:** Scan selected folders/drives, persist file metadata in SQLite, and visualize disk usage through Tree View, largest files/folders tables, and Treemap.

This project is a **read-only disk analyzer**. The application must help users understand disk usage. It must not delete, move, rename, compress, upload, or modify files in the MVP.

---

# Core Principles

## I. Read-Only Safety First

The application must be read-only in the current product phase.

### Rules

- The app must not delete files.
- The app must not move files.
- The app must not rename files.
- The app must not modify file contents.
- The app must not compress files.
- The app must not automatically clean anything.
- The app may only read filesystem metadata and display it.
- The only allowed OS-level action is revealing/opening a file or folder location in Windows Explorer.

### Rationale

The first version is focused on visibility and analysis, not cleanup. This reduces risk, avoids accidental data loss, and keeps the product scope focused.

---

## II. Rust Owns Filesystem, Persistence, and Query Logic

Rust is the source of truth for all scanning, persistence, aggregation, and querying.

### Rules

- Filesystem scanning must be implemented in Rust.
- SQLite access must be implemented in Rust.
- Folder size aggregation must be implemented in Rust.
- Tauri commands must expose controlled query APIs to React.
- React must not access SQLite directly.
- React must not perform filesystem scanning.
- React must not calculate final folder sizes for production behavior.

### Rationale

Rust is better suited for performance-sensitive filesystem operations, large data processing, and safe concurrency. React should remain a UI layer.

---

## III. SQLite Is the Primary Data Layer

SQLite is required for scan storage and must not be replaced by frontend in-memory storage.

### Rules

- Every scan must create a `scan_session`.
- Every discovered file/folder must be persisted as a `file_entry`.
- Scan errors must be persisted as `scan_errors`.
- Folder sizes must be persisted after aggregation.
- React must request scoped data through Tauri commands.
- The app must not send the full file tree to React in one response.
- Large result sets must use limits, offsets, or scoped queries.
- SQLite writes during scanning must use transactions and batching.

### Rationale

Large disks may contain hundreds of thousands or millions of entries. SQLite allows scalable querying, lazy loading, caching, and stable performance.

---

## IV. Never Load the Full Tree in the Frontend

The frontend must render scoped views, not the complete filesystem tree.

### Rules

- Tree View must lazy-load folder children.
- Tables must use pagination, limits, or virtualization.
- Treemap must request only top N entries for the selected folder.
- Search must be performed through SQLite-backed Rust commands.
- React state must not contain the entire scan result.
- React may store UI state such as selected folder, expanded folder IDs, active tab, filters, and sort options.

### Rationale

Rendering or storing a full disk tree in React can freeze the UI and consume excessive memory.

---

## V. Performance Is a Product Requirement

Performance is not optional. The app must remain responsive during scans and while browsing large results.

### Rules

- Scanning must run outside the UI thread.
- Scan progress events must be throttled.
- Do not emit one frontend event per file.
- Prefer progress events every 250–500ms.
- SQLite inserts must be batched.
- Expensive queries must be indexed.
- Tree and table rendering must be virtualized where needed.
- Treemap rendering must cap the number of visible nodes.
- Long-running Rust operations must not block the Tauri frontend.

### Rationale

The product competes on fast disk analysis. A slow or freezing UI breaks the core value proposition.

---

## VI. Scan Robustness Over Perfect Completeness

The scanner must continue when it encounters inaccessible or problematic paths.

### Rules

- Permission errors must not crash the scan.
- Locked files must not crash the scan.
- Broken symlinks must not crash the scan.
- Very long paths must be handled where possible.
- Unicode and Arabic file names must be supported.
- Skipped paths must be counted.
- Scan errors must be recorded in SQLite.
- Users must be able to inspect skipped/error paths after the scan.

### Rationale

Real filesystems are messy. The app must produce useful results even when some paths cannot be scanned.

---

## VII. MVP Scanner Before Advanced Scanner

The MVP must use a stable recursive Rust scanner before introducing NTFS MFT scanning.

### Rules

- Do not implement NTFS MFT scanning in the MVP unless the recursive scanner, SQLite persistence, Tree View, Largest Files, Largest Folders, and Treemap are already working.
- Advanced NTFS/MFT scanning belongs to a future phase.
- The scanner architecture should allow future replacement or addition of an MFT-based scanner.
- Scanner implementations should expose a common interface where practical.

### Rationale

Building the full product experience first reduces risk. MFT scanning is powerful but adds complexity and Windows-specific low-level behavior.

---

## VIII. Clear Separation of Product Layers

The codebase must maintain clean separation between scanning, storage, commands, and UI.

### Required Backend Modules

The Rust backend should be organized around these responsibilities:

- `scanner`: filesystem traversal and metadata extraction
- `db`: SQLite connection, schema, migrations, and low-level queries
- `repository`: scan session, file entry, and scan error data operations
- `aggregator`: folder size and child/descendant count aggregation
- `commands`: Tauri command handlers
- `events`: scan progress/completion/failure events
- `models`: shared Rust data structures and DTOs

### Required Frontend Areas

The React frontend should be organized around:

- `features/scan`
- `features/tree`
- `features/largest-files`
- `features/largest-folders`
- `features/treemap`
- `features/extensions`
- `features/errors`
- `shared/components`
- `shared/utils`
- `shared/api`

### Rationale

A clear architecture keeps the app maintainable as scanner performance, visualization, and query features grow.

---

## IX. Specification-Driven Development

All implementation must follow the product specification and this constitution.

### Rules

- Requirements must be documented before implementation.
- Implementation must not add major features outside the active specification.
- If a requested implementation conflicts with this constitution, the constitution wins.
- Product behavior belongs in `spec.md`.
- Technical decisions belong in `plan.md`.
- Implementation steps belong in `tasks.md`.
- Do not mix low-level implementation details into product requirements unless required for acceptance criteria.
- Do not invent destructive file operations unless the constitution is amended.

### Rationale

The project should remain controlled, focused, and suitable for AI-assisted implementation without scope drift.

---

## X. No AI Features in Product Scope

The product must not include AI assistant behavior in the MVP.

### Rules

- No AI assistant inside the app.
- No “what should I delete?” assistant.
- No cloud AI calls.
- No file metadata upload to AI services.
- No automated cleanup recommendations powered by AI.
- Development may use external AI coding tools, but the product itself must not include AI features.

### Rationale

The MVP is a local, read-only disk analyzer. AI product features introduce privacy, complexity, and scope concerns.

---

# Technical Standards

## Backend Standards

### Rust

- Prefer explicit error handling.
- Avoid panics in production scanning paths.
- Use structured errors for scanner, database, and command failures.
- Long-running work must run off the main UI thread.
- Use DTOs for frontend-facing responses.
- Keep internal database models separate from frontend DTOs where useful.

### SQLite

Required tables:

- `scan_sessions`
- `file_entries`
- `scan_errors`

Required query behavior:

- `get_children`
- `get_largest_files`
- `get_largest_folders`
- `search_entries`
- `get_extension_summary`
- `get_treemap_data`
- `get_scan_session`

Required SQLite practices:

- Use prepared statements.
- Use transactions for batch inserts.
- Use indexes for session, parent, size, extension, path, and name.
- Avoid unbounded queries in Tauri commands.
- Use WAL mode unless there is a strong reason not to.

Recommended pragmas:

```sql
PRAGMA journal_mode = WAL;
PRAGMA synchronous = NORMAL;
PRAGMA temp_store = MEMORY;
```

---

## Frontend Standards

### React

- Use TypeScript.
- Use typed Tauri command wrappers.
- Use React Query or equivalent for async command state.
- Use local/Zustand state only for UI state.
- Do not store the full scan result in React state.
- Use virtualization for large lists.
- Debounce search inputs.
- Keep components focused and feature-oriented.

### UI Requirements

The UI must provide:

- Path selection
- Scan progress
- Scan summary
- Tree View
- Largest Files table
- Largest Folders table
- Treemap
- Extension summary
- Scan errors view
- Reveal in Explorer action

The UI must not provide:

- Delete button
- Move button
- Rename button
- Cleanup button
- Auto-fix button
- AI assistant panel

---

# Required Tauri Commands

The backend must expose these commands or equivalent scoped alternatives:

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

---

# Required Events

The backend must emit scan lifecycle events:

```txt
scan-progress
scan-completed
scan-failed
scan-cancelled
```

Event emission must be throttled and must not happen per file.

---

# Required Data Model

## ScanSession

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
```

## FileEntry

```ts
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
```

## ExtensionSummary

```ts
type ExtensionSummaryDto = {
  extension: string;
  totalSize: number;
  fileCount: number;
  percentageOfScan: number;
};
```

## TreemapNode

```ts
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

---

# Required Database Schema

The implementation must start with this schema or an equivalent compatible design.

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

CREATE INDEX idx_file_entries_session
ON file_entries(scan_session_id);

CREATE INDEX idx_file_entries_parent
ON file_entries(parent_id);

CREATE INDEX idx_file_entries_size
ON file_entries(size DESC);

CREATE INDEX idx_file_entries_extension
ON file_entries(extension);

CREATE INDEX idx_file_entries_path
ON file_entries(path);

CREATE INDEX idx_file_entries_name
ON file_entries(name);
```

---

# Testing Standards

## Backend Tests

Backend tests must cover:

- Scanner handles normal folders.
- Scanner handles empty folders.
- Scanner handles permission/access errors where practical.
- Scanner records skipped paths.
- Folder size aggregation is correct.
- SQLite insert/query operations work.
- `get_children` returns only direct children.
- `get_largest_files` excludes folders.
- `get_largest_folders` excludes files.
- Search returns expected files/folders.
- Extension summary calculates correct total size and counts.

## Frontend Tests

Frontend tests should cover:

- Scan screen renders.
- Progress card updates from event data.
- Tree View renders returned children.
- Largest Files table renders rows.
- Empty states render correctly.
- Error states render correctly.
- Search input debounces and calls query function.
- Treemap handles empty and non-empty data.

---

# UX Rules

## Display Rules

- Sizes must be formatted as human-readable values.
- Percentages should be shown relative to parent folder or scan total depending on context.
- Long paths must be truncated visually but available in tooltip/copy.
- Empty states must explain what the user should do next.
- Errors must be understandable and not expose raw Rust panic messages.

## Navigation Rules

- Selecting a folder in Tree View should update the Treemap context.
- Treemap should support drilldown into folders.
- Treemap should provide breadcrumb navigation.
- Tables should support sorting.
- Search results should make it clear whether an item is a file or folder.

---

# Performance Budgets

The app should target the following behavior:

- UI must remain responsive during scanning.
- Progress updates should appear during long scans.
- Opening a folder in Tree View should not require loading the entire scan.
- Treemap should render a capped subset, default max 200 children.
- Largest Files default query should return top 100.
- Search results should be limited and debounced.
- No frontend view should intentionally request all rows from a large scan.

---

# Privacy and Security Rules

- The app must operate locally.
- The app must not upload paths, filenames, metadata, or scan results.
- The app must not require internet access.
- The app must not include analytics in the MVP.
- SQLite scan data remains local.
- Any future telemetry requires explicit product specification and constitution amendment.

---

# Out-of-Scope Features

The following must not be implemented unless the constitution is amended:

- Delete file
- Move file
- Rename file
- Move to Recycle Bin
- Cleanup recommendations
- AI assistant
- Cloud sync
- Online account
- File upload
- Background service
- Scheduled scans
- Duplicate removal
- Registry cleaner
- System optimizer

---

# Development Workflow Rules

## Before Coding

For every major feature:

1. Confirm requirement in spec.
2. Confirm it does not violate read-only scope.
3. Confirm required backend command or frontend component.
4. Confirm database impact.
5. Confirm performance impact.

## During Coding

- Keep commits focused.
- Do not mix unrelated features.
- Do not introduce deletion/mutation behavior.
- Prefer typed DTOs between Rust and TypeScript.
- Add tests for non-trivial logic.
- Add indexes before relying on large queries.

## Before Marking Feature Complete

A feature is complete only when:

- It satisfies acceptance criteria.
- It has basic tests where practical.
- It handles empty/error states.
- It avoids loading unbounded data into React.
- It does not violate read-only safety.
- It is aligned with this constitution.

---

# Governance

## Constitution Priority

This constitution overrides:

- Agent suggestions
- Convenience shortcuts
- Unspecified implementation ideas
- Out-of-scope feature requests
- Destructive file operation suggestions

## Amendment Process

This constitution may be changed only intentionally.

Any amendment must include:

- What changed
- Why it changed
- Which specs/plans/tasks are affected
- Whether the MVP scope changed

---

# Version

**Constitution version:** 1.0.0  
**Created for:** FastDisk Viewer MVP  
**Scope:** Read-only Windows disk usage analyzer  
**Stack:** Tauri + React + TypeScript + Rust + SQLite  
**Last updated:** 2026-06-07
