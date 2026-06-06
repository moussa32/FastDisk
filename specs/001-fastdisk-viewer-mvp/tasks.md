# Tasks: FastDisk Viewer MVP

**Input**: Design documents from `/specs/001-fastdisk-viewer-mvp/`

**Prerequisites**: plan.md (required), spec.md (required for user stories), research.md, data-model.md, contracts/

**Tests**: Backend tests are required for scanner, SQLite, aggregation, command, and query logic by the constitution. Frontend tests are included for meaningful UI state and interaction coverage.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Establish the project structure, dependencies, and testing baseline needed for the MVP.

- [ ] T001 Add Rust dependencies for SQLite, time handling, filesystem traversal helpers, cancellation flags, and serialization in src-tauri/Cargo.toml
- [ ] T002 Add frontend dependencies for async query state, table rendering, virtualization, treemap rendering, and test tooling in package.json
- [ ] T003 Create backend module directories scanner, db, repository, aggregator, commands, events, and models under src-tauri/src/
- [ ] T004 Create frontend feature directories scan, tree, largest-files, largest-folders, treemap, search, extensions, errors, and shared directories under src/
- [ ] T005 [P] Add Rust test module structure for unit and integration coverage in src-tauri/src/lib.rs and src-tauri/src/test_support.rs
- [ ] T006 [P] Add frontend test setup and shared render helpers in src/shared/test/render.tsx and src/setupTests.ts
- [ ] T007 Replace starter Tauri greeting wiring with FastDisk module registration skeleton in src-tauri/src/lib.rs
- [ ] T008 Replace starter React welcome screen with FastDisk app shell placeholder in src/App.tsx and src/App.css

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Build shared data, database, command, event, and API foundations that all user stories depend on.

**CRITICAL**: No user story work can begin until this phase is complete.

- [ ] T009 Define ScanSession, FileEntry, ScanIssue, ExtensionSummary, TreemapNode, FilterSet, and shared status enums in src-tauri/src/models/mod.rs
- [ ] T010 Define frontend DTOs and shared command input types matching the contract in src/shared/api/types.ts
- [ ] T011 Create SQLite connection manager with WAL pragmas and migration runner in src-tauri/src/db/mod.rs
- [ ] T012 Add scan_sessions, file_entries, scan_errors schema and required indexes in src-tauri/src/db/migrations.rs
- [ ] T013 [P] Add database migration tests for required tables and indexes in src-tauri/src/db/tests.rs
- [ ] T014 Create repository layer for scan sessions, file entries, scan issues, and bounded queries in src-tauri/src/repository/mod.rs
- [ ] T015 [P] Add repository tests for insert, query, pagination, and bounded limit validation in src-tauri/src/repository/tests.rs
- [ ] T016 Create structured backend error types and frontend-safe error mapping in src-tauri/src/models/errors.rs
- [ ] T017 Create scan lifecycle event payloads and throttled emitter helper in src-tauri/src/events/mod.rs
- [ ] T018 [P] Add throttled event tests that prevent per-file progress emission in src-tauri/src/events/tests.rs
- [ ] T019 Create typed Tauri invoke wrappers for all planned commands in src/shared/api/commands.ts
- [ ] T020 Create shared formatting utilities for sizes, percentages, dates, and paths in src/shared/utils/format.ts
- [ ] T021 [P] Add frontend utility tests for size, percentage, date, and path formatting in src/shared/utils/format.test.ts
- [ ] T022 Create shared EmptyState, ErrorState, LoadingState, SizeBadge, PercentageBar, and Breadcrumb components in src/shared/components/
- [ ] T023 Create top-level layout with scan selection area, results tabs, and details region in src/App.tsx

**Checkpoint**: Foundation ready. User story implementation can begin.

---

## Phase 3: User Story 1 - Scan a Folder or Drive (Priority: P1)

**Goal**: User can select a folder or drive, start one read-only scan, see progress, cancel if needed, and receive a completed summary without UI freezes.

**Independent Test**: Select a valid folder, start a scan, observe responsive progress, cancel a long scan, and complete a small scan with summary totals and skipped-item counts.

### Tests for User Story 1

- [ ] T024 [P] [US1] Add scanner tests for normal folders, empty folders, zero-byte files, Unicode names, and link-like paths in src-tauri/src/scanner/tests.rs
- [ ] T025 [P] [US1] Add scanner robustness tests for inaccessible or locked paths where practical in src-tauri/src/scanner/tests.rs
- [ ] T026 [P] [US1] Add scan session command tests for start_scan, cancel_scan, and get_scan_session in src-tauri/src/commands/tests.rs
- [ ] T027 [P] [US1] Add scan screen component tests for disabled scan, selected path, progress, cancellation, and summary states in src/features/scan/ScanScreen.test.tsx

### Implementation for User Story 1

- [ ] T028 [US1] Implement read-only recursive scanner that records metadata and does not follow symlink or junction directories in src-tauri/src/scanner/mod.rs
- [ ] T029 [US1] Implement scan worker orchestration, one-active-scan guard, cancellation flag, and progress collection in src-tauri/src/scanner/session.rs
- [ ] T030 [US1] Implement batched file_entry and scan_error writes during scans in src-tauri/src/repository/scan_writer.rs
- [ ] T031 [US1] Implement folder size aggregation and child/descendant count updates after traversal in src-tauri/src/aggregator/mod.rs
- [ ] T032 [US1] Implement start_scan, cancel_scan, and get_scan_session Tauri commands in src-tauri/src/commands/scan.rs
- [ ] T033 [US1] Register scan commands and scan lifecycle events in src-tauri/src/lib.rs
- [ ] T034 [US1] Implement typed frontend scan command wrappers and event listeners in src/shared/api/scan.ts
- [ ] T035 [US1] Implement PathPicker, ScanButton, ScanProgressCard, and ScanSummaryCard in src/features/scan/
- [ ] T036 [US1] Implement ScanScreen orchestration for selecting paths, starting scans, progress events, cancellation, and summary loading in src/features/scan/ScanScreen.tsx
- [ ] T037 [US1] Integrate ScanScreen into the app shell and remove starter placeholder content in src/App.tsx
- [ ] T038 [US1] Add friendly empty, invalid path, scan failed, cancelled, and skipped-item messages in src/features/scan/ScanMessages.tsx

**Checkpoint**: User Story 1 is independently functional and testable.

---

## Phase 4: User Story 2 - Browse Disk Usage Hierarchically (Priority: P1)

**Goal**: User can browse a completed scan through a lazy-loaded tree without loading the full scan into React.

**Independent Test**: Complete a scan, open results, expand folders, verify direct children load on demand, and confirm large folders remain usable.

### Tests for User Story 2

- [ ] T039 [P] [US2] Add get_children repository and command tests for direct-child-only queries, sorting, limits, and offsets in src-tauri/src/commands/tests.rs
- [ ] T040 [P] [US2] Add FileTree component tests for root rendering, expand/collapse, lazy loading, selected folder state, and empty children in src/features/tree/FileTree.test.tsx

### Implementation for User Story 2

- [ ] T041 [US2] Implement get_children bounded repository query with parent, sort, limit, and offset support in src-tauri/src/repository/queries.rs
- [ ] T042 [US2] Implement get_children Tauri command and validation in src-tauri/src/commands/tree.rs
- [ ] T043 [US2] Add getChildren frontend API wrapper and query keys in src/shared/api/tree.ts
- [ ] T044 [US2] Implement virtualized lazy FileTree rows with name, type, size, percentage, counts, and modified date in src/features/tree/FileTree.tsx
- [ ] T045 [US2] Implement selected folder state shared between Tree View and result context in src/features/tree/treeState.ts
- [ ] T046 [US2] Add Tree View tab and scan result layout integration in src/App.tsx

**Checkpoint**: User Stories 1 and 2 work independently after the foundation.

---

## Phase 5: User Story 3 - Identify Largest Files and Folders (Priority: P1)

**Goal**: User can view bounded largest file and folder tables and reveal item locations in Windows Explorer without modifying files.

**Independent Test**: Complete a scan, open Largest Files and Largest Folders, sort rows, change file limits, and reveal an existing scanned item.

### Tests for User Story 3

- [ ] T047 [P] [US3] Add repository and command tests for get_largest_files, get_largest_folders, filters, sorting, and file/folder exclusion in src-tauri/src/commands/tests.rs
- [ ] T048 [P] [US3] Add reveal_in_explorer command tests for missing path and read-only behavior in src-tauri/src/commands/tests.rs
- [ ] T049 [P] [US3] Add largest table component tests for rows, sorting, limits, filters, and reveal action states in src/features/largest-files/LargestFilesTable.test.tsx and src/features/largest-folders/LargestFoldersTable.test.tsx

### Implementation for User Story 3

- [ ] T050 [US3] Implement get_largest_files and get_largest_folders bounded repository queries in src-tauri/src/repository/queries.rs
- [ ] T051 [US3] Implement get_largest_files and get_largest_folders Tauri commands in src-tauri/src/commands/largest.rs
- [ ] T052 [US3] Implement reveal_in_explorer command with missing-path friendly errors and no file mutation in src-tauri/src/commands/reveal.rs
- [ ] T053 [US3] Register largest and reveal commands in src-tauri/src/lib.rs
- [ ] T054 [US3] Add largest files, largest folders, and reveal frontend API wrappers in src/shared/api/largest.ts and src/shared/api/reveal.ts
- [ ] T055 [US3] Implement reusable virtualized FileTable component for bounded tabular results in src/shared/components/FileTable.tsx
- [ ] T056 [US3] Implement LargestFilesTable with default 100 and selectable 500/1000 limits in src/features/largest-files/LargestFilesTable.tsx
- [ ] T057 [US3] Implement LargestFoldersTable with calculated size and child/descendant counts in src/features/largest-folders/LargestFoldersTable.tsx
- [ ] T058 [US3] Add Largest Files and Largest Folders tabs to scan results layout in src/App.tsx

**Checkpoint**: User Stories 1, 2, and 3 provide the P1 MVP browsing experience.

---

## Phase 6: User Story 4 - Visualize Usage with a Treemap (Priority: P2)

**Goal**: User can view a capped treemap for the selected folder, drill into folders, navigate back, and inspect tooltips.

**Independent Test**: Select a folder after a scan, render a capped treemap, drill down into a folder, navigate back with breadcrumbs, and confirm small items group into Other when needed.

### Tests for User Story 4

- [ ] T059 [P] [US4] Add get_treemap_data repository and command tests for top-N limiting, percentages, and Other grouping in src-tauri/src/commands/tests.rs
- [ ] T060 [P] [US4] Add TreemapView component tests for empty data, non-empty data, drilldown, breadcrumb, and tooltip content in src/features/treemap/TreemapView.test.tsx

### Implementation for User Story 4

- [ ] T061 [US4] Implement get_treemap_data bounded repository query and Other grouping in src-tauri/src/repository/treemap.rs
- [ ] T062 [US4] Implement get_treemap_data Tauri command in src-tauri/src/commands/treemap.rs
- [ ] T063 [US4] Add treemap frontend API wrapper in src/shared/api/treemap.ts
- [ ] T064 [US4] Implement TreemapView with capped rendering, tooltip, drilldown, and breadcrumb navigation in src/features/treemap/TreemapView.tsx
- [ ] T065 [US4] Connect Tree View selected folder to Treemap context in src/features/treemap/treemapState.ts
- [ ] T066 [US4] Add Treemap tab to scan results layout in src/App.tsx

**Checkpoint**: Treemap visualization is independently usable for completed scans.

---

## Phase 7: User Story 5 - Search and Filter Scan Results (Priority: P3)

**Goal**: User can search by partial text and filter result views by extension, size, and file/folder type with bounded results.

**Independent Test**: Complete a scan, search partial text, apply file type and size filters, use predefined extension groups, and confirm results remain bounded.

### Tests for User Story 5

- [ ] T067 [P] [US5] Add search_entries repository and command tests for partial text, entry type, size filters, extension filters, limits, and offsets in src-tauri/src/commands/tests.rs
- [ ] T068 [P] [US5] Add frontend tests for debounced search input, filter controls, and bounded search result rendering in src/features/search/SearchPanel.test.tsx

### Implementation for User Story 5

- [ ] T069 [US5] Implement FilterSet validation and predefined extension groups in src-tauri/src/models/filters.rs
- [ ] T070 [US5] Implement search_entries bounded repository query with filters and size-descending default order in src-tauri/src/repository/search.rs
- [ ] T071 [US5] Implement search_entries Tauri command in src-tauri/src/commands/search.rs
- [ ] T072 [US5] Add search frontend API wrapper in src/shared/api/search.ts
- [ ] T073 [US5] Implement SearchPanel with debounced query, entry type, extension group, and size filters in src/features/search/SearchPanel.tsx
- [ ] T074 [US5] Integrate search and filters into Tree, Largest Files, and Largest Folders result views in src/App.tsx

**Checkpoint**: Search and filtering are independently usable on completed scans.

---

## Phase 8: User Story 6 - Review Scan Issues and Previous Results (Priority: P3)

**Goal**: User can review skipped paths, inspect scan errors, reopen previous scans, and understand cancelled scan state.

**Independent Test**: Scan a location with skipped items, open Scan Errors, reopen a previous scan from history, and verify cancelled scans are clearly labeled.

### Tests for User Story 6

- [ ] T075 [P] [US6] Add list_scan_sessions and get_scan_errors repository and command tests for bounded history and error rows in src-tauri/src/commands/tests.rs
- [ ] T076 [P] [US6] Add RecentScans and ScanErrorsTable component tests for loading, empty, error, cancelled, and selected scan states in src/features/scan/RecentScans.test.tsx and src/features/errors/ScanErrorsTable.test.tsx

### Implementation for User Story 6

- [ ] T077 [US6] Implement list_scan_sessions and get_scan_errors bounded repository queries in src-tauri/src/repository/queries.rs
- [ ] T078 [US6] Implement list_scan_sessions and get_scan_errors Tauri commands in src-tauri/src/commands/history.rs and src-tauri/src/commands/errors.rs
- [ ] T079 [US6] Add previous scans and scan errors frontend API wrappers in src/shared/api/history.ts and src/shared/api/errors.ts
- [ ] T080 [US6] Implement RecentScans list with completed, failed, and cancelled status labels in src/features/scan/RecentScans.tsx
- [ ] T081 [US6] Implement ScanErrorsTable with path, error kind, and friendly message columns in src/features/errors/ScanErrorsTable.tsx
- [ ] T082 [US6] Add Extensions tab query and table for extension summaries in src/features/extensions/ExtensionSummaryTable.tsx
- [ ] T083 [US6] Add Scan Errors, Extensions, and Recent Scans integration to the app shell in src/App.tsx

**Checkpoint**: Scan history, errors, and extension summaries are independently usable.

---

## Phase 9: Polish & Cross-Cutting Concerns

**Purpose**: Validate the complete MVP, tighten performance, and ensure constitutional compliance.

- [ ] T084 [P] Add end-to-end manual QA checklist results for quickstart workflow in specs/001-fastdisk-viewer-mvp/quickstart.md
- [ ] T085 Run backend test suite and record any Rust toolchain/setup gaps in specs/001-fastdisk-viewer-mvp/quickstart.md
- [ ] T086 Run frontend test suite and record any dependency/setup gaps in specs/001-fastdisk-viewer-mvp/quickstart.md
- [ ] T087 Run pnpm build and Tauri build when Rust is available, then document outcomes in specs/001-fastdisk-viewer-mvp/quickstart.md
- [ ] T088 Audit UI for forbidden destructive actions and document read-only compliance in specs/001-fastdisk-viewer-mvp/quickstart.md
- [ ] T089 Audit large-result views for bounded queries, virtualization, and capped treemap behavior in src/features/
- [ ] T090 Review friendly error messages for raw panic leakage and missing path handling in src-tauri/src/models/errors.rs and src/shared/components/ErrorState.tsx

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies.
- **Foundational (Phase 2)**: Depends on Setup completion and blocks all user stories.
- **User Stories (Phase 3+)**: Depend on Foundational completion.
- **Polish (Phase 9)**: Depends on all desired user stories being complete.

### User Story Dependencies

- **US1 Scan a Folder or Drive (P1)**: Starts after Foundation and is the first MVP slice.
- **US2 Browse Disk Usage Hierarchically (P1)**: Depends on scan sessions and entries from US1 but remains independently testable with a completed scan.
- **US3 Identify Largest Files and Folders (P1)**: Depends on scan sessions, entries, aggregation, and reveal command foundation; can proceed after US1 data exists.
- **US4 Visualize Usage with a Treemap (P2)**: Depends on selected-folder context from US2 and aggregated sizes.
- **US5 Search and Filter Scan Results (P3)**: Depends on persisted entries and shared FilterSet.
- **US6 Review Scan Issues and Previous Results (P3)**: Depends on persisted sessions and scan issues from US1.

### Within Each User Story

- Tests before implementation for non-trivial backend/frontend behavior.
- Backend query/command implementation before frontend API wrappers.
- Frontend API wrappers before UI components that call them.
- Story integration after story-specific components are complete.

---

## Parallel Opportunities

- Setup tasks T005 and T006 can run in parallel after dependencies are added.
- Foundational tests T013, T015, T018, and T021 can run in parallel after their target modules exist.
- Each user story has parallel backend and frontend test tasks before implementation.
- US2 and US3 can proceed in parallel after US1 produces completed scan data and Foundation is complete.
- US5 and US6 can proceed in parallel after persisted scan entries and sessions exist.

## Parallel Examples

### User Story 1

```text
Task: T024 scanner normal/edge tests in src-tauri/src/scanner/tests.rs
Task: T026 command tests in src-tauri/src/commands/tests.rs
Task: T027 scan screen tests in src/features/scan/ScanScreen.test.tsx
```

### User Story 2

```text
Task: T039 get_children command tests in src-tauri/src/commands/tests.rs
Task: T040 FileTree component tests in src/features/tree/FileTree.test.tsx
```

### User Story 3

```text
Task: T047 largest query tests in src-tauri/src/commands/tests.rs
Task: T049 largest table component tests in src/features/largest-files/LargestFilesTable.test.tsx and src/features/largest-folders/LargestFoldersTable.test.tsx
```

### User Story 4

```text
Task: T059 treemap command tests in src-tauri/src/commands/tests.rs
Task: T060 TreemapView component tests in src/features/treemap/TreemapView.test.tsx
```

### User Story 5

```text
Task: T067 search command tests in src-tauri/src/commands/tests.rs
Task: T068 SearchPanel component tests in src/features/search/SearchPanel.test.tsx
```

### User Story 6

```text
Task: T075 history/error command tests in src-tauri/src/commands/tests.rs
Task: T076 RecentScans and ScanErrorsTable component tests in src/features/scan/RecentScans.test.tsx and src/features/errors/ScanErrorsTable.test.tsx
```

---

## Implementation Strategy

### MVP First

1. Complete Phase 1 and Phase 2.
2. Complete US1 to produce persisted scan data and progress.
3. Complete US2 and US3 for the core P1 browsing MVP.
4. Stop and validate scan, tree, largest files, largest folders, and reveal flows.

### Incremental Delivery

1. US1 delivers scan selection, progress, storage, aggregation, cancellation, and summary.
2. US2 adds hierarchical browsing.
3. US3 adds largest files/folders and reveal.
4. US4 adds visual treemap exploration.
5. US5 adds search and filtering.
6. US6 adds scan history, errors, and extension summary.

### Suggested MVP Scope

The first demonstrable MVP should include Setup, Foundation, US1, US2, and US3.
US4-US6 can be layered afterward without changing the read-only data model.

---

## Notes

- Every task line follows `- [ ] T### [P?] [US?] Description with file path`.
- User story labels appear only in user story phases.
- Tests are included because the constitution requires coverage for scanner, database, aggregation, command, and query behavior.
- Read-only scope must be preserved throughout implementation.
