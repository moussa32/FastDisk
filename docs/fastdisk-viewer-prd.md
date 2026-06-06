# PRD — FastDisk Viewer

## 1. Product Overview

### Product Name

**FastDisk Viewer**

### Product Type

Windows desktop disk usage analyzer.

### Core Purpose

FastDisk Viewer helps users understand where disk space is being used by scanning a selected drive or folder, storing scan results in SQLite, and presenting disk usage through:

- Hierarchical Tree View
- Largest files table
- Largest folders table
- Treemap visualization
- Search and filtering
- File/folder metadata views

The product is **read-only** in the current phase. It does not delete, move, clean, rename, compress, or modify files.

---

## 2. Problem Statement

Users often do not know which files or folders are consuming the most disk space. Windows File Explorer does not provide a fast, visual, and hierarchical way to analyze disk usage across large directories or full drives.

Existing tools like WizTree solve this problem well, but the goal of this project is to build a custom desktop application with:

- A fast Rust-based scanner
- Modern Tauri + React UI
- SQLite-backed scan storage
- Tree and Treemap visualization
- Extensible architecture for future advanced scanning

---

## 3. Goals

### Primary Goals

1. Allow the user to scan a selected drive or folder.
2. Collect file and folder metadata using Rust.
3. Store scan results in SQLite.
4. Calculate folder sizes based on child files and folders.
5. Display scan results in a performant Tree View.
6. Display the largest files.
7. Display the largest folders.
8. Visualize disk usage using a Treemap.
9. Provide search and filtering capabilities.
10. Keep the app read-only and safe in the first version.

### Secondary Goals

1. Support large file systems without freezing the UI.
2. Provide scan progress feedback.
3. Handle permission errors gracefully.
4. Allow users to reveal file/folder locations in Windows Explorer.
5. Cache previous scan results in SQLite.
6. Provide useful summary statistics after scanning.

---

## 4. Non-Goals

The following are explicitly out of scope for the current version:

1. AI workflow.
2. AI assistant inside the app.
3. File deletion.
4. Move to Recycle Bin.
5. File movement.
6. File renaming.
7. File cleanup automation.
8. Duplicate file removal.
9. Cloud sync.
10. Cross-device scanning.
11. macOS/Linux support in the first version.
12. NTFS MFT scanner in the MVP.
13. Antivirus-like behavior.
14. File modification or compression.
15. Automatic cleanup recommendations.

---

## 5. Target Users

### Primary User

A Windows user who wants to understand what is taking disk space and manually decide what to do outside the app.

### Secondary User

A technical user or power user who wants a fast visual disk usage analyzer with a clean UI and reliable data.

---

## 6. Platform

### Initial Platform

Windows desktop.

### Technology Stack

#### Frontend

- React
- TypeScript
- Tauri frontend runtime
- TanStack Table
- TanStack Virtual
- D3 Treemap or Nivo Treemap

#### Backend

- Rust
- Tauri commands
- SQLite
- Rust filesystem APIs
- Optional: Rayon or async worker threads for scanning

#### Database

- SQLite

---

## 7. Product Scope

### 7.1 MVP Scope

The MVP should include:

1. Select folder or drive.
2. Start scan.
3. Show scan progress.
4. Store scan results in SQLite.
5. Calculate file and folder sizes.
6. Show scan summary.
7. Show Tree View.
8. Show Largest Files table.
9. Show Largest Folders table.
10. Show Treemap for selected folder.
11. Search files and folders.
12. Filter by file extension and minimum size.
13. Reveal file/folder location in Windows Explorer.
14. Handle access-denied errors without crashing.

---

## 8. User Stories

### 8.1 Scan Selection

#### User Story

As a user, I want to choose a folder or drive to scan, so I can analyze disk usage in that location.

#### Acceptance Criteria

- User can choose a folder.
- User can choose a drive path such as `C:\`.
- App shows selected path before scan starts.
- App validates that the selected path exists.
- App prevents starting a scan if no path is selected.

---

### 8.2 Start Scan

#### User Story

As a user, I want to start scanning the selected location, so the app can analyze disk usage.

#### Acceptance Criteria

- User can click a `Scan` button.
- App starts scanning in the Rust backend.
- UI remains responsive during scan.
- App shows scan state: idle, scanning, completed, failed, cancelled.
- App does not block React rendering during scan.

---

### 8.3 Scan Progress

#### User Story

As a user, I want to see scan progress, so I know the app is working.

#### Acceptance Criteria

The UI should show:

- Current scanned path
- Number of files scanned
- Number of folders scanned
- Total discovered size
- Skipped items count
- Elapsed time
- Scan status

Progress does not need to show exact percentage in MVP because total file count is unknown before scan completion.

---

### 8.4 Store Scan Results in SQLite

#### User Story

As a user, I want scan results to be stored locally, so I can browse results efficiently without loading everything into memory.

#### Acceptance Criteria

- Every scan creates a scan session.
- File and folder records are stored in SQLite.
- React does not receive the full file tree at once.
- UI queries data through paginated or scoped commands.
- Previous scan can be loaded from SQLite.
- New scan can replace or create a new session.

---

### 8.5 Tree View

#### User Story

As a user, I want to browse folders in a tree structure, so I can understand where space is used.

#### Acceptance Criteria

- Root scan path appears as the top-level node.
- User can expand and collapse folders.
- Children are loaded on demand from SQLite.
- Items are sorted by size descending by default.
- Tree View supports large folders using virtualization.
- Each row displays:
  - Name
  - Type
  - Size
  - Percentage of parent size
  - File/folder count if available
  - Modified date if available

---

### 8.6 Largest Files Table

#### User Story

As a user, I want to see the largest files, so I can quickly identify files that consume the most space.

#### Acceptance Criteria

- App displays top largest files.
- Default limit: top 100.
- User can increase limit to 500 or 1000.
- Table supports sorting by:
  - Size
  - Name
  - Extension
  - Modified date
- Table supports search and filtering.
- Each row displays:
  - File name
  - Full path
  - Size
  - Extension
  - Modified date

---

### 8.7 Largest Folders Table

#### User Story

As a user, I want to see the largest folders, so I can quickly identify high-impact directories.

#### Acceptance Criteria

- App displays largest folders from the current scan.
- Folders are ordered by calculated total size.
- Each row displays:
  - Folder name
  - Full path
  - Total size
  - Direct child count
  - Total descendant count if available

---

### 8.8 Treemap Visualization

#### User Story

As a user, I want to see a visual map of disk usage, so I can quickly understand which files and folders take the most space.

#### Acceptance Criteria

- App shows Treemap for the selected folder.
- Rectangle size represents file/folder size.
- User can click a folder rectangle to drill down.
- User can go back to parent folder.
- Tooltip shows:
  - Name
  - Full path
  - Size
  - Percentage of current folder
- Treemap should not attempt to render millions of items.
- Treemap should load top N children only.
- Default Treemap limit: top 200 children.
- Small items can be grouped into `Other`.

---

### 8.9 Search

#### User Story

As a user, I want to search by file or folder name, so I can quickly find specific items.

#### Acceptance Criteria

- Search works against SQLite.
- Search supports partial text.
- Search returns files and folders.
- Search results show:
  - Name
  - Path
  - Size
  - Type
  - Extension
- Search results are ordered by size descending by default.
- Search should support a result limit.

---

### 8.10 Filters

#### User Story

As a user, I want to filter files by type or size, so I can narrow down disk usage.

#### Acceptance Criteria

Supported filters:

- Extension
- Minimum size
- Maximum size
- File only
- Folder only
- Modified date range, optional in later MVP stage

Useful predefined groups:

- Videos: `.mp4`, `.mkv`, `.mov`, `.avi`, `.wmv`
- Archives: `.zip`, `.rar`, `.7z`, `.tar`, `.gz`
- Disk images: `.iso`, `.img`
- Installers: `.exe`, `.msi`
- Documents: `.pdf`, `.docx`, `.xlsx`, `.pptx`

---

### 8.11 Reveal in Explorer

#### User Story

As a user, I want to open a file or folder location in Windows Explorer, so I can manually inspect it.

#### Acceptance Criteria

- User can open folder in Explorer.
- User can reveal file in Explorer.
- App does not modify the file.
- If path no longer exists, app shows a friendly error.

---

### 8.12 Error Handling

#### User Story

As a user, I want the app to continue scanning even if some files are inaccessible.

#### Acceptance Criteria

- Access denied errors do not stop the scan.
- Locked files do not stop the scan.
- Broken symlinks do not crash the app.
- Skipped paths are counted.
- App shows total skipped items after scan.
- Error details can be viewed in a scan report.

---

## 9. Functional Requirements

### 9.1 Scanner Requirements

The scanner must:

1. Accept a root path.
2. Traverse directories recursively.
3. Read metadata for files and folders.
4. Capture file size.
5. Capture modified timestamp when available.
6. Detect whether item is file or directory.
7. Extract extension for files.
8. Generate stable internal IDs.
9. Store parent-child relationships.
10. Continue on permission errors.
11. Emit progress events.
12. Persist results to SQLite.

---

### 9.2 Scanner Output

Each scanned item should produce a record with:

- ID
- Scan session ID
- Parent ID
- Name
- Full path
- Size
- Is directory
- Extension
- Modified timestamp
- Created timestamp, optional
- Depth
- Error flag, optional
- Symlink flag, optional

---

### 9.3 Folder Size Aggregation

The app must calculate folder sizes after scanning.

#### Rule

File size is read from filesystem metadata.

Folder size is calculated as:

```txt
folder_size = sum of all descendant file sizes
```

#### Acceptance Criteria

- Root folder size equals total scanned size.
- Parent folder size includes nested children.
- Folder sizes are persisted in SQLite.
- Aggregation should be efficient for large scans.

---

### 9.4 SQLite Requirements

SQLite is required and should be used as the main local data store.

SQLite should store:

1. Scan sessions.
2. File/folder records.
3. Aggregated folder sizes.
4. Scan errors.
5. Optional summary data.

React should request data from Rust/Tauri commands, not directly from SQLite.

---

## 10. Database Design

### 10.1 Table: scan_sessions

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
```

### 10.2 Table: file_entries

```sql
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
```

### 10.3 Table: scan_errors

```sql
CREATE TABLE scan_errors (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  scan_session_id INTEGER NOT NULL,
  path TEXT NOT NULL,
  error_kind TEXT NOT NULL,
  error_message TEXT,
  created_at TEXT NOT NULL,

  FOREIGN KEY(scan_session_id) REFERENCES scan_sessions(id)
);
```

### 10.4 Indexes

```sql
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

Optional full-text search later:

```sql
CREATE VIRTUAL TABLE file_entries_fts
USING fts5(name, path);
```

---

## 11. Tauri Commands

### 11.1 start_scan

Starts a new scan.

```ts
invoke("start_scan", {
  path: "C:\\Users\\MI"
});
```

Returns:

```ts
type StartScanResponse = {
  scanSessionId: number;
};
```

---

### 11.2 cancel_scan

Cancels active scan.

```ts
invoke("cancel_scan", {
  scanSessionId: 1
});
```

---

### 11.3 get_scan_session

Gets scan summary.

```ts
invoke("get_scan_session", {
  scanSessionId: 1
});
```

---

### 11.4 get_children

Gets direct children for a folder.

```ts
invoke("get_children", {
  scanSessionId: 1,
  parentId: 50,
  sortBy: "size",
  sortDirection: "desc",
  limit: 500,
  offset: 0
});
```

---

### 11.5 get_largest_files

Gets largest files.

```ts
invoke("get_largest_files", {
  scanSessionId: 1,
  limit: 100,
  offset: 0
});
```

---

### 11.6 get_largest_folders

Gets largest folders.

```ts
invoke("get_largest_folders", {
  scanSessionId: 1,
  limit: 100,
  offset: 0
});
```

---

### 11.7 search_entries

Searches files and folders.

```ts
invoke("search_entries", {
  scanSessionId: 1,
  query: "video",
  entryType: "all",
  limit: 100,
  offset: 0
});
```

---

### 11.8 get_extension_summary

Returns size grouped by extension.

```ts
invoke("get_extension_summary", {
  scanSessionId: 1
});
```

---

### 11.9 get_treemap_data

Returns Treemap data for selected folder.

```ts
invoke("get_treemap_data", {
  scanSessionId: 1,
  parentId: 50,
  limit: 200
});
```

---

### 11.10 reveal_in_explorer

Opens item location in Windows Explorer.

```ts
invoke("reveal_in_explorer", {
  path: "C:\\Users\\MI\\Downloads\\file.zip"
});
```

---

## 12. Tauri Events

### 12.1 scan-progress

Emitted during scan.

```ts
type ScanProgressEvent = {
  scanSessionId: number;
  currentPath: string;
  filesScanned: number;
  foldersScanned: number;
  bytesScanned: number;
  skippedItems: number;
  elapsedMs: number;
};
```

### 12.2 scan-completed

Emitted when scan completes.

```ts
type ScanCompletedEvent = {
  scanSessionId: number;
  totalFiles: number;
  totalFolders: number;
  totalSize: number;
  skippedItems: number;
  elapsedMs: number;
};
```

### 12.3 scan-failed

Emitted if scan fails.

```ts
type ScanFailedEvent = {
  scanSessionId: number;
  errorMessage: string;
};
```

### 12.4 scan-cancelled

Emitted if user cancels scan.

```ts
type ScanCancelledEvent = {
  scanSessionId: number;
};
```

---

## 13. Frontend Screens

### 13.1 Home / Scan Screen

Purpose:

Allow the user to choose a path and start scanning.

Main components:

- Path selector
- Recent scans list
- Scan button
- Scan progress card
- Error summary

---

### 13.2 Scan Results Screen

Main layout:

```txt
--------------------------------------------------
Top Bar:
Selected scan | Root path | Total size | Search
--------------------------------------------------
Left/Main:
Tree View
--------------------------------------------------
Right/Bottom:
Details panel / Treemap / Largest files tabs
--------------------------------------------------
```

Suggested tabs:

1. Tree View
2. Largest Files
3. Largest Folders
4. Treemap
5. Extensions
6. Scan Errors

---

### 13.3 Tree View Tab

Features:

- Expand/collapse folders
- Lazy load children
- Sort by size
- Display percentage of parent
- Reveal in Explorer
- Select folder to update Treemap

---

### 13.4 Largest Files Tab

Features:

- Top 100 by default
- Pagination or infinite scrolling
- Search within results
- Extension filter
- Minimum size filter
- Reveal in Explorer

---

### 13.5 Treemap Tab

Features:

- Visualize current selected folder
- Drill down by clicking folder
- Breadcrumb navigation
- Tooltip with metadata
- Group tiny items into Other

---

### 13.6 Extensions Tab

Features:

Shows extension-based disk usage summary.

Columns:

- Extension
- Total size
- File count
- Percentage of scan size

Example:

```txt
.mp4    24.5 GB    142 files    18%
.zip    10.2 GB     51 files     7%
.iso     8.8 GB      3 files     6%
```

---

### 13.7 Scan Errors Tab

Features:

Shows paths that could not be scanned.

Columns:

- Path
- Error kind
- Error message

---

## 14. UI Components

### 14.1 Core Components

- `PathPicker`
- `ScanButton`
- `ScanProgressCard`
- `ScanSummaryCard`
- `FileTree`
- `FileTable`
- `LargestFilesTable`
- `LargestFoldersTable`
- `TreemapView`
- `ExtensionSummaryTable`
- `ScanErrorsTable`
- `Breadcrumb`
- `SizeBadge`
- `PercentageBar`
- `EmptyState`
- `ErrorState`

---

## 15. Frontend State Management

Recommended approach:

- React Query for async Tauri commands.
- Zustand or local state for UI-only state.
- Avoid storing complete scan tree in React state.
- Store selected scan session ID.
- Store selected folder ID.
- Store expanded folder IDs.
- Store sorting/filter state.

Example state:

```ts
type AppState = {
  activeScanSessionId: number | null;
  selectedEntryId: number | null;
  expandedFolderIds: Set<number>;
  activeTab:
    | "tree"
    | "largest-files"
    | "largest-folders"
    | "treemap"
    | "extensions"
    | "errors";
};
```

---

## 16. Performance Requirements

### 16.1 Scanning

The scanner should:

- Run outside the UI thread.
- Avoid blocking Tauri frontend.
- Batch SQLite inserts.
- Use transactions for insert performance.
- Avoid excessive event emissions.
- Emit progress every 250ms to 500ms, not per file.

### 16.2 SQLite

Required optimizations:

- Use transactions for batch inserts.
- Insert records in batches.
- Add indexes after bulk insert if needed.
- Use prepared statements.
- Avoid querying huge unbounded result sets.
- Use pagination or limits.

Recommended pragmas during scan:

```sql
PRAGMA journal_mode = WAL;
PRAGMA synchronous = NORMAL;
PRAGMA temp_store = MEMORY;
```

### 16.3 React UI

The UI should:

- Use virtualized lists for large tables.
- Lazy-load tree children.
- Avoid rendering the full tree.
- Limit Treemap data size.
- Debounce search input.
- Avoid storing millions of rows in frontend memory.

---

## 17. Data Flow

### 17.1 Scan Flow

```txt
User selects path
        ↓
React calls start_scan(path)
        ↓
Rust creates scan_session in SQLite
        ↓
Rust scanner traverses filesystem
        ↓
Rust writes file_entries in batches
        ↓
Rust emits scan-progress events
        ↓
Rust aggregates folder sizes
        ↓
Rust updates scan_session summary
        ↓
Rust emits scan-completed
        ↓
React loads summary and initial root children
```

---

### 17.2 Browse Flow

```txt
User expands folder
        ↓
React calls get_children(parent_id)
        ↓
Rust queries SQLite
        ↓
React renders returned children only
```

---

### 17.3 Treemap Flow

```txt
User selects folder
        ↓
React calls get_treemap_data(parent_id)
        ↓
Rust queries top children by size
        ↓
Rust groups small items into Other
        ↓
React renders Treemap
```

---

## 18. Sorting Rules

Default sorting:

- Folders and files sorted by size descending.

Optional sorting:

- Name ascending/descending
- Size ascending/descending
- Modified date ascending/descending
- Type

Recommended Tree View default:

```txt
Directories and files mixed, ordered by size descending
```

Alternative setting later:

```txt
Show folders first
```

---

## 19. Size Formatting

The UI should format sizes as:

```txt
Bytes
KB
MB
GB
TB
```

Example:

```txt
1,024 bytes → 1 KB
1,536 MB → 1.5 GB
```

Use binary units internally if preferred:

```txt
1 KB = 1024 bytes
1 MB = 1024 KB
```

---

## 20. Edge Cases

The app must handle:

1. Permission denied folders.
2. Locked files.
3. Very long paths.
4. Symlinks.
5. Junction points.
6. Empty folders.
7. Zero-byte files.
8. Files deleted after scan.
9. Drives disconnected after scan.
10. Huge directories with hundreds of thousands of files.
11. Unicode and Arabic file names.
12. Paths with special characters.
13. Hidden files.
14. System files.
15. Multiple scans over time.

---

## 21. Security and Safety

Since this version is read-only:

- App should not delete files.
- App should not modify files.
- App should not move files.
- App should not rename files.
- App should not compress files.
- App should not upload file metadata.
- All scan data stays local in SQLite.
- No internet connection is required.
- No AI or cloud service is used.

Privacy note:

The app scans file names and paths, which may contain sensitive information. This data must remain local.

---

## 22. Permissions

The app should work without admin mode for normal folders.

If scanning protected folders:

- Some paths may fail.
- App should count skipped items.
- App may show a message: “Some files could not be scanned due to permissions.”

Admin mode is not required for MVP.

---

## 23. Future Scope

The following can be considered after MVP:

### 23.1 Advanced Scanner

- NTFS MFT-based scanner
- Faster full-drive scan
- Admin mode detection
- Drive-level analysis

### 23.2 Advanced Visualization

- File type breakdown chart
- Timeline by modified date
- Folder heatmap
- Disk usage comparison between scans

### 23.3 Advanced Analysis

- Duplicate detection
- Old large files
- Recently created large files
- Extension intelligence
- Cached scans comparison

### 23.4 Read-Only Recommendations

Even without deleting files, the app can later suggest:

- Largest videos
- Largest archives
- Old downloads
- Large installers
- Large cache folders

The app should still not delete anything unless a later product version explicitly adds safe actions.

---

## 24. Success Metrics

### 24.1 Technical Metrics

- Scan does not freeze UI.
- Tree View handles large folders smoothly.
- Largest Files query returns quickly.
- Treemap renders within acceptable time.
- SQLite database does not become unnecessarily huge.
- App handles permission errors without crashing.

### 24.2 User Experience Metrics

- User can identify largest folders quickly.
- User can identify largest files quickly.
- User understands scan status.
- User can browse results without confusion.
- User can reveal items in Explorer.

---

## 25. MVP Priority Order

### P0 — Core Foundation

1. Tauri + React app setup.
2. Rust scanner.
3. SQLite schema.
4. Scan session creation.
5. File/folder metadata insertion.
6. Folder size aggregation.
7. Scan progress events.
8. Scan completed summary.

### P1 — Main User Experience

9. Path picker.
10. Scan progress UI.
11. Tree View.
12. Lazy-load children.
13. Largest Files table.
14. Largest Folders table.
15. Reveal in Explorer.

### P2 — Visualization

16. Treemap for selected folder.
17. Treemap drilldown.
18. Breadcrumb navigation.
19. Tooltip with size and percentage.
20. Group small items into Other.

### P3 — Search and Filtering

21. Search files/folders.
22. Filter by extension.
23. Filter by minimum size.
24. Extension summary tab.
25. Scan errors tab.

### P4 — Polish

26. Recent scans.
27. Cancel scan.
28. Better empty states.
29. Better error messages.
30. Export scan summary, optional.

---

## 26. Recommended MVP Build Plan

### Milestone 1 — Project Skeleton

Deliverables:

- Tauri app created.
- React frontend running.
- Rust commands callable from React.
- SQLite connection working.

Done when:

- React can call a test Tauri command.
- Rust can create and query SQLite database.

---

### Milestone 2 — Basic Scanner

Deliverables:

- Rust scanner accepts path.
- Recursively scans files and folders.
- Stores records in SQLite.
- Handles permission errors.
- Creates scan session.

Done when:

- User can scan a small folder.
- SQLite contains file/folder records.
- Scan does not crash on inaccessible files.

---

### Milestone 3 — Aggregation

Deliverables:

- Folder sizes calculated.
- Child count calculated.
- Scan session summary updated.

Done when:

- Root folder total size is correct.
- Largest folders can be queried correctly.

---

### Milestone 4 — Results UI

Deliverables:

- Scan summary screen.
- Tree View.
- Largest Files table.
- Largest Folders table.

Done when:

- User can browse scan results.
- User can identify largest files and folders.

---

### Milestone 5 — Treemap

Deliverables:

- Treemap tab.
- Folder drilldown.
- Breadcrumb.
- Tooltip.

Done when:

- User can visually inspect selected folder usage.

---

### Milestone 6 — Search and Filters

Deliverables:

- Search by name/path.
- Filter by extension.
- Filter by size.
- Extension summary.

Done when:

- User can narrow results to videos, archives, installers, and large files.

---

## 27. Architectural Decision Records

### ADR 1 — Use SQLite instead of in-memory state

#### Decision

Use SQLite as the primary data store for scan results.

#### Reason

Large scans may contain hundreds of thousands or millions of entries. Keeping everything in React state or frontend memory is risky and inefficient.

#### Consequences

Positive:

- Better scalability.
- Query-based UI.
- Previous scans can be cached.
- Tables can use pagination.
- Tree children can be loaded lazily.

Negative:

- More backend complexity.
- Need schema migrations.
- Need careful indexing.

---

### ADR 2 — Rust owns scan data

#### Decision

Rust backend owns scanning, aggregation, persistence, and query logic.

#### Reason

Rust is better suited for filesystem scanning and performance-sensitive processing.

#### Consequences

React remains simpler and only requests views from Rust.

---

### ADR 3 — React does not receive full tree

#### Decision

React must not load the full tree at once.

#### Reason

Large file systems can produce too many nodes for frontend memory and rendering.

#### Consequences

Tree View must lazy-load children and use virtualization.

---

### ADR 4 — Read-only MVP

#### Decision

The MVP is read-only.

#### Reason

The product’s first goal is visibility, not cleanup. Read-only behavior reduces risk and complexity.

#### Consequences

No delete, move, cleanup, or automated actions are included.

---

## 28. Open Questions

1. Should MVP support only Windows, or should the architecture remain portable for macOS/Linux later?
2. Should each scan create a new session, or should scanning the same path replace the previous session?
3. Should hidden/system files be included by default?
4. Should symlinks and junctions be followed or skipped?
5. Should scan cancellation preserve partial results or discard them?
6. Should the app support multiple active scans, or only one scan at a time?
7. Should Treemap show files and folders together or folders first?
8. Should the app allow exporting results to CSV in MVP or later?

---

## 29. Recommended Decisions for MVP

1. Support Windows only.
2. Create a new scan session for every scan.
3. Include hidden files by default.
4. Do not follow symlinks/junctions in MVP to avoid cycles.
5. On cancel, mark scan session as cancelled and keep partial results.
6. Allow only one active scan at a time.
7. Treemap shows files and folders together, sorted by size.
8. CSV export can be postponed.

---

## 30. Final MVP Definition

The MVP is complete when a user can:

1. Open the desktop app.
2. Select a folder or drive.
3. Start a scan.
4. See scan progress.
5. Wait for scan completion.
6. View total disk usage summary.
7. Browse files and folders in Tree View.
8. See the largest files.
9. See the largest folders.
10. Visualize selected folder usage in Treemap.
11. Search and filter results.
12. Reveal selected file or folder in Windows Explorer.

The app must not delete, move, rename, compress, or modify files in this version.
