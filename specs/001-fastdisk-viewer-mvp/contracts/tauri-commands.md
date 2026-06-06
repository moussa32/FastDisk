# Contract: Tauri Commands and Events

All commands are read-only except for local scan-session persistence. No command
may delete, move, rename, compress, upload, clean, or modify user files.

## Commands

### `start_scan`

Starts a scan for a validated local path. Only one scan may be active at a time.

**Request**

```ts
type StartScanInput = {
  path: string;
};
```

**Response**

```ts
type StartScanResponse = {
  scanSessionId: number;
};
```

**Errors**

- Invalid or missing path.
- Another scan is already active.
- Scan session could not be created.

### `cancel_scan`

Requests cancellation of an active scan.

```ts
type CancelScanInput = {
  scanSessionId: number;
};
```

Returns no body on success. Cancelled sessions are marked `cancelled` and may
preserve partial results.

### `get_scan_session`

Returns summary for one scan session.

```ts
type GetScanSessionInput = {
  scanSessionId: number;
};
```

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

### `list_scan_sessions`

Returns previous scan sessions in descending start time.

```ts
type ListScanSessionsInput = {
  limit: number;
  offset: number;
};
```

```ts
type ListScanSessionsResponse = {
  items: ScanSessionDto[];
};
```

### `get_children`

Returns direct children for a folder, never descendants beyond one level.

```ts
type GetChildrenInput = {
  scanSessionId: number;
  parentId: number | null;
  sortBy: "size" | "name" | "modified_at" | "type";
  sortDirection: "asc" | "desc";
  limit: number;
  offset: number;
};
```

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

### `get_largest_files`

Returns bounded file rows ordered by size descending unless another allowed sort
is requested.

```ts
type GetLargestFilesInput = {
  scanSessionId: number;
  limit: 100 | 500 | 1000;
  offset: number;
  sortBy?: "size" | "name" | "extension" | "modified_at";
  sortDirection?: "asc" | "desc";
  filters?: FilterSetDto;
};
```

```ts
type GetLargestFilesResponse = {
  items: FileEntryDto[];
};
```

### `get_largest_folders`

Returns bounded folder rows ordered by calculated total size descending unless
another allowed sort is requested.

```ts
type GetLargestFoldersInput = {
  scanSessionId: number;
  limit: number;
  offset: number;
  sortBy?: "size" | "name" | "modified_at";
  sortDirection?: "asc" | "desc";
};
```

```ts
type GetLargestFoldersResponse = {
  items: FileEntryDto[];
};
```

### `search_entries`

Searches files and folders by partial text with bounded results.

```ts
type SearchEntriesInput = {
  scanSessionId: number;
  query: string;
  entryType: "all" | "file" | "folder";
  limit: number;
  offset: number;
  filters?: FilterSetDto;
};
```

```ts
type SearchEntriesResponse = {
  items: FileEntryDto[];
};
```

### `get_extension_summary`

Returns aggregated extension usage for one scan session.

```ts
type ExtensionSummaryDto = {
  extension: string;
  totalSize: number;
  fileCount: number;
  percentageOfScan: number;
};
```

```ts
type GetExtensionSummaryInput = {
  scanSessionId: number;
};

type GetExtensionSummaryResponse = {
  items: ExtensionSummaryDto[];
};
```

### `get_treemap_data`

Returns a capped treemap payload for the selected folder.

```ts
type GetTreemapDataInput = {
  scanSessionId: number;
  parentId: number | null;
  limit: number;
};
```

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

### `get_scan_errors`

Returns bounded scan issues for a scan session.

```ts
type ScanIssueDto = {
  id: number;
  scanSessionId: number;
  path: string;
  errorKind: string;
  errorMessage?: string;
  createdAt: string;
};

type GetScanErrorsInput = {
  scanSessionId: number;
  limit: number;
  offset: number;
};

type GetScanErrorsResponse = {
  items: ScanIssueDto[];
};
```

### `reveal_in_explorer`

Reveals an existing file or folder in Windows Explorer.

```ts
type RevealInExplorerInput = {
  path: string;
};
```

Returns no body on success. If the path no longer exists, returns a friendly
missing-path error.

## Shared Filter DTO

```ts
type FilterSetDto = {
  extension?: string;
  extensionGroup?: "videos" | "archives" | "disk_images" | "installers" | "documents";
  minSize?: number;
  maxSize?: number;
  entryType?: "all" | "file" | "folder";
};
```

## Events

### `scan-progress`

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

Emission must be throttled and must not occur once per file.

### `scan-completed`

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

### `scan-failed`

```ts
type ScanFailedEvent = {
  scanSessionId: number;
  errorMessage: string;
};
```

### `scan-cancelled`

```ts
type ScanCancelledEvent = {
  scanSessionId: number;
};
```
