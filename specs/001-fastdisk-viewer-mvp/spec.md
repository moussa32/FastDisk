# Feature Specification: FastDisk Viewer MVP

**Feature Branch**: `001-fastdisk-viewer-mvp`

**Created**: 2026-06-07

**Status**: Draft

**Input**: User description: "Create the FastDisk Viewer MVP specification from fastdisk-viewer-prd.md"

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Scan a Folder or Drive (Priority: P1)

As a Windows user, I want to choose a folder or drive and start a scan, so I can
analyze where disk space is being used without changing any files.

**Why this priority**: Scanning is the entry point for every other capability.
Without a safe, responsive scan flow, the product has no usable disk usage data.

**Independent Test**: Can be tested by selecting a valid folder, starting a
scan, observing progress, and confirming a completed scan summary appears while
the app remains responsive.

**Acceptance Scenarios**:

1. **Given** no path is selected, **When** the user views the scan screen,
   **Then** the app prevents starting a scan and indicates that a path is needed.
2. **Given** the user selects an existing folder or drive path, **When** the user
   starts a scan, **Then** the app shows the selected path, scan state, progress
   details, and eventually a completed scan summary.
3. **Given** a scan is running, **When** the user interacts with the app,
   **Then** the interface remains responsive and continues showing progress.
4. **Given** some files or folders cannot be accessed, **When** the scan reaches
   them, **Then** the scan continues and counts skipped items.

---

### User Story 2 - Browse Disk Usage Hierarchically (Priority: P1)

As a user, I want to browse scanned folders in a tree, so I can understand which
areas of the selected location consume the most space.

**Why this priority**: Hierarchical browsing is the core way users understand
disk usage and decide where to inspect next.

**Independent Test**: Can be tested after any completed scan by opening the scan
results, expanding folders, and verifying only requested levels are shown with
size and percentage information.

**Acceptance Scenarios**:

1. **Given** a completed scan, **When** the user opens results, **Then** the root
   scan path appears as the top-level item with total size.
2. **Given** a folder is visible, **When** the user expands it, **Then** direct
   children load on demand and appear sorted by size descending by default.
3. **Given** a large folder, **When** its children are displayed, **Then** the app
   remains usable and does not attempt to display the entire scan at once.

---

### User Story 3 - Identify Largest Files and Folders (Priority: P1)

As a user, I want lists of the largest files and folders, so I can quickly find
the biggest contributors to disk usage.

**Why this priority**: Largest-item views provide immediate value even before
the user explores the full tree or visualizations.

**Independent Test**: Can be tested by completing a scan and confirming the
largest files and folders views show bounded, sortable results with useful
metadata and reveal actions.

**Acceptance Scenarios**:

1. **Given** a completed scan, **When** the user opens Largest Files, **Then** the
   app shows the top files by size with name, path, size, extension, and modified
   date when available.
2. **Given** a completed scan, **When** the user opens Largest Folders, **Then**
   the app shows folders ordered by calculated total size with path and child
   counts when available.
3. **Given** a largest-item row, **When** the user chooses reveal, **Then** the
   item location opens in Windows Explorer without modifying the item.

---

### User Story 4 - Visualize Usage with a Treemap (Priority: P2)

As a user, I want a treemap for the selected folder, so I can visually understand
which files and folders take the most space.

**Why this priority**: The treemap makes patterns obvious and helps users
quickly choose which folder to inspect further.

**Independent Test**: Can be tested by selecting a folder after a scan,
confirming a treemap appears, drilling into a folder, using breadcrumbs to go
back, and checking tooltip details.

**Acceptance Scenarios**:

1. **Given** a completed scan and selected folder, **When** the user opens the
   treemap, **Then** the app shows a capped set of the largest child items where
   rectangle size reflects item size.
2. **Given** a folder rectangle, **When** the user selects it, **Then** the
   treemap drills into that folder and updates navigation context.
3. **Given** many small items, **When** the treemap would otherwise be crowded,
   **Then** small items may be grouped into an "Other" segment.

---

### User Story 5 - Search and Filter Scan Results (Priority: P3)

As a user, I want to search and filter scanned items, so I can narrow results to
specific names, file types, sizes, or item categories.

**Why this priority**: Search and filtering make completed scans practical for
targeted investigation, especially when scans contain many files.

**Independent Test**: Can be tested by searching for partial text, applying file
type and size filters, and confirming bounded results show matching files and
folders.

**Acceptance Scenarios**:

1. **Given** a completed scan, **When** the user searches by partial name,
   **Then** matching files and folders appear with name, path, size, type, and
   extension when available.
2. **Given** a completed scan, **When** the user filters by extension group or
   size range, **Then** results update to show only matching items.
3. **Given** a large scan, **When** search results are returned, **Then** results
   are limited and ordered by size descending by default.

---

### User Story 6 - Review Scan Issues and Previous Results (Priority: P3)

As a user, I want to review skipped paths, scan errors, and previous scans, so I
can trust the results and continue analysis later.

**Why this priority**: Error visibility and previous scan access make the app
feel reliable and useful after the first scan.

**Independent Test**: Can be tested by scanning a location with inaccessible
items, confirming skipped items are summarized, opening the scan errors view,
and reopening a previous scan from local history.

**Acceptance Scenarios**:

1. **Given** a scan has skipped items, **When** the user opens the scan errors
   view, **Then** the app shows skipped paths with understandable error details.
2. **Given** previous scans exist, **When** the user opens the app, **Then** the
   user can choose a previous scan and browse its results.
3. **Given** a scan is cancelled, **When** the user views scan history, **Then**
   the cancelled scan is marked as cancelled and any preserved partial results
   are clearly indicated.

### Edge Cases

- Scanning a missing or invalid path must fail gracefully before scan start.
- Scanning a protected folder must continue past inaccessible items when
  possible and count skipped paths.
- Locked files, broken links, long paths, hidden files, system files, zero-byte
  files, special characters, and Unicode or Arabic file names must not crash the
  app.
- Link-like folders that could create cycles must not cause infinite scanning.
- Files deleted, moved, or disconnected after a scan must show a friendly error
  when the user tries to reveal them.
- Very large folders must not require loading or rendering every scanned item at
  once.
- Treemap views with many small items must remain readable by limiting or
  grouping displayed items.
- Search and filtered views must remain bounded even for very large scans.
- The feature must preserve read-only behavior: no delete, move, rename,
  cleanup, compression, upload, or file-content mutation actions are available.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST allow the user to select an existing folder or drive
  path before starting a scan.
- **FR-002**: System MUST prevent scan start when no valid path is selected.
- **FR-003**: System MUST show scan states including idle, scanning, completed,
  failed, and cancelled.
- **FR-004**: System MUST show scan progress with current path, files scanned,
  folders scanned, discovered size, skipped item count, elapsed time, and status.
- **FR-005**: System MUST complete scans without freezing the user interface.
- **FR-006**: System MUST preserve read-only scope: no delete, move, rename,
  compress, cleanup, upload, automatic recommendation-to-clean, or file-content
  mutation behavior.
- **FR-007**: System MUST keep scan results local on the user's device.
- **FR-008**: System MUST create a distinct scan session for every new scan.
- **FR-009**: System MUST record discovered files and folders with parent-child
  relationships.
- **FR-010**: System MUST record item metadata including name, full path, size,
  item type, extension when available, depth, and modified date when available.
- **FR-011**: System MUST calculate folder sizes as the sum of descendant file
  sizes.
- **FR-012**: System MUST persist calculated folder sizes and scan summary
  totals.
- **FR-013**: System MUST continue scanning when individual paths are
  inaccessible where possible.
- **FR-014**: System MUST count skipped items and make scan issues inspectable
  after scan completion.
- **FR-015**: System MUST support loading previous scan results from local
  history.
- **FR-016**: System MUST expose scan results through scoped or bounded result
  requests rather than delivering the full scan tree at once.
- **FR-017**: System MUST display a Tree View with root path, expandable folders,
  on-demand child loading, and default size-descending ordering.
- **FR-018**: Tree View rows MUST display name, type, size, percentage of parent
  size, file or folder count when available, and modified date when available.
- **FR-019**: System MUST display a Largest Files view with a default limit of
  100 items and user-selectable higher limits of 500 and 1000.
- **FR-020**: Largest Files MUST support sorting by size, name, extension, and
  modified date.
- **FR-021**: System MUST display a Largest Folders view ordered by calculated
  total size.
- **FR-022**: Largest Folders rows MUST show folder name, full path, total size,
  direct child count, and descendant count when available.
- **FR-023**: System MUST display a Treemap for the selected folder with a
  default maximum of 200 child items.
- **FR-024**: Treemap MUST support folder drilldown, parent navigation, and
  tooltips with name, path, size, and percentage of current folder.
- **FR-025**: Treemap MAY group small items into an "Other" segment to preserve
  readability.
- **FR-026**: System MUST support searching files and folders by partial text.
- **FR-027**: Search results MUST include name, path, size, type, extension when
  available, and bounded result counts.
- **FR-028**: System MUST support filters for extension, minimum size, maximum
  size, files only, and folders only.
- **FR-029**: System SHOULD include predefined extension groups for videos,
  archives, disk images, installers, and documents.
- **FR-030**: System MUST provide an extension summary showing extension, total
  size, file count, and percentage of scan size.
- **FR-031**: System MUST provide a scan errors view with path, error kind, and
  understandable error message.
- **FR-032**: System MUST allow the user to reveal a file or folder location in
  Windows Explorer without modifying it.
- **FR-033**: System MUST show friendly messages for invalid paths, missing
  items, scan failures, empty folders, and empty search results.
- **FR-034**: System MUST allow only one active scan at a time in the MVP.
- **FR-035**: System MUST mark cancelled scans as cancelled and preserve partial
  results when available.

### Key Entities *(include if feature involves data)*

- **Scan Session**: A single scan run for a selected root path, including status,
  timing, total files, total folders, total size, skipped count, and completion
  state.
- **File Entry**: A scanned file or folder with identity, parent relationship,
  name, path, size, type, extension, depth, timestamps, link status, and child or
  descendant counts.
- **Scan Issue**: A path that could not be scanned or revealed, including error
  kind, readable message, and when it occurred.
- **Extension Summary**: Aggregated disk usage by file extension, including total
  size, count, and percentage of the scan.
- **Treemap Segment**: A visual item representing a file, folder, or grouped
  "Other" segment within the selected folder context.
- **Filter Set**: User-selected constraints such as extension, size bounds, item
  type, and optional date range.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: A user can select a folder or drive, start a scan, and reach a
  completed summary without needing instructions outside the app.
- **SC-002**: During long scans, the app continues to respond to user input and
  shows progress updates at least every 1 second while work is active.
- **SC-003**: After scanning a folder with at least 100,000 items, the user can
  expand a folder, switch result views, and search without the interface becoming
  unusable.
- **SC-004**: Largest Files and Largest Folders views show initial results within
  2 seconds after the user opens each view for a completed large scan.
- **SC-005**: Treemap for a selected folder renders a readable capped view within
  2 seconds for folders with many children.
- **SC-006**: 95% of tested scans that encounter inaccessible individual paths
  complete with skipped items recorded instead of failing the whole scan.
- **SC-007**: Users can identify the largest file and largest folder in a
  completed scan within 30 seconds of opening the results screen.
- **SC-008**: No user-facing workflow in the MVP modifies, deletes, moves,
  renames, compresses, uploads, or automatically cleans files.
- **SC-009**: Search returns bounded matching results within 2 seconds for common
  partial-name searches on completed scans.
- **SC-010**: Users can reveal an existing scanned item in Windows Explorer from
  tree, table, or search results in one direct action.

## Assumptions

- MVP targets Windows desktop users only.
- Every scan creates a new scan session rather than replacing an earlier scan.
- Hidden and system files are included when the app has permission to read their
  metadata.
- Link-like folders are not followed in MVP to avoid cycles.
- Cancelled scans are marked cancelled and may retain clearly labeled partial
  results.
- Only one scan runs at a time in MVP.
- Treemap displays files and folders together, sorted by size.
- Exporting scan summaries is optional and outside the initial MVP unless added
  by a later specification.
