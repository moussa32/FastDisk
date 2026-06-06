# Data Model: FastDisk Viewer MVP

## ScanSession

Represents one scan run for a selected root path.

**Fields**

- `id`: Stable numeric identifier.
- `rootPath`: Selected folder or drive path.
- `status`: `idle`, `scanning`, `completed`, `failed`, or `cancelled`.
- `startedAt`: Timestamp when the scan session began.
- `completedAt`: Optional timestamp when terminal state was reached.
- `totalFiles`: Number of files discovered.
- `totalFolders`: Number of folders discovered.
- `totalSize`: Total descendant file size for the root.
- `skippedItems`: Number of skipped or inaccessible paths.
- `elapsedMs`: Scan duration in milliseconds.

**Relationships**

- Has many `FileEntry` records.
- Has many `ScanIssue` records.

**Validation Rules**

- `rootPath` must be non-empty and valid before scan start.
- `status` must be one of the allowed lifecycle values.
- Totals must be non-negative.

**State Transitions**

```text
idle -> scanning -> completed
idle -> scanning -> failed
idle -> scanning -> cancelled
```

## FileEntry

Represents one scanned file or folder.

**Fields**

- `id`: Stable numeric identifier within persisted data.
- `scanSessionId`: Owning scan session.
- `parentId`: Parent folder entry, or null for root.
- `name`: Display name.
- `path`: Full local path.
- `size`: File size for files; aggregated descendant size for folders.
- `isDirectory`: Whether the entry is a folder.
- `extension`: File extension when available.
- `depth`: Depth from scan root.
- `modifiedAt`: Optional modified timestamp.
- `createdAt`: Optional created timestamp.
- `isSymlink`: Whether the entry is link-like.
- `childCount`: Direct child count.
- `descendantCount`: Total nested descendant count.

**Relationships**

- Belongs to one `ScanSession`.
- May belong to one parent `FileEntry`.
- May have many child `FileEntry` records.

**Validation Rules**

- `scanSessionId`, `name`, `path`, `size`, `isDirectory`, and `depth` are
  required.
- `size`, `depth`, `childCount`, and `descendantCount` must be non-negative.
- Link-like folders are recorded but not followed in MVP.

## ScanIssue

Represents a path that could not be scanned or revealed.

**Fields**

- `id`: Stable numeric identifier.
- `scanSessionId`: Owning scan session.
- `path`: Affected path.
- `errorKind`: User-understandable category such as access denied, locked file,
  broken link, missing path, or unknown.
- `errorMessage`: Friendly error detail.
- `createdAt`: Timestamp when the issue was recorded.

**Relationships**

- Belongs to one `ScanSession`.

**Validation Rules**

- `scanSessionId`, `path`, `errorKind`, and `createdAt` are required.
- Error messages must not expose raw panic text.

## ExtensionSummary

Represents aggregated disk usage by extension for a scan.

**Fields**

- `extension`: Extension label; empty or extensionless items use a stable
  display label.
- `totalSize`: Sum of matching file sizes.
- `fileCount`: Number of matching files.
- `percentageOfScan`: Total size divided by scan total.

**Relationships**

- Derived from `FileEntry` records in one `ScanSession`.

**Validation Rules**

- `totalSize` and `fileCount` must be non-negative.
- `percentageOfScan` must be between 0 and 100.

## TreemapNode

Represents a visual segment for a selected folder.

**Fields**

- `id`: File entry identifier, or null for a grouped "Other" segment.
- `name`: Display name.
- `path`: Full path or selected-folder context for grouped segments.
- `size`: Segment size.
- `isDirectory`: Whether the segment represents a folder.
- `percentageOfParent`: Segment size divided by selected folder size.
- `children`: Optional child segments for drilldown response.

**Relationships**

- Derived from top child `FileEntry` records for a selected folder.

**Validation Rules**

- Child count must be capped by request limit.
- Grouped "Other" may combine small items when needed for readability.

## FilterSet

Represents user-selected constraints for search and result views.

**Fields**

- `query`: Optional partial text.
- `entryType`: `all`, `file`, or `folder`.
- `extension`: Optional extension or predefined group.
- `minSize`: Optional minimum size.
- `maxSize`: Optional maximum size.
- `modifiedFrom`: Optional lower date bound for later MVP stage.
- `modifiedTo`: Optional upper date bound for later MVP stage.
- `limit`: Maximum returned rows.
- `offset`: Pagination offset.

**Validation Rules**

- `limit` must be bounded by the specific view contract.
- `offset` must be non-negative.
- `minSize` must not exceed `maxSize` when both are present.

## Predefined Extension Groups

- Videos: `.mp4`, `.mkv`, `.mov`, `.avi`, `.wmv`
- Archives: `.zip`, `.rar`, `.7z`, `.tar`, `.gz`
- Disk images: `.iso`, `.img`
- Installers: `.exe`, `.msi`
- Documents: `.pdf`, `.docx`, `.xlsx`, `.pptx`
