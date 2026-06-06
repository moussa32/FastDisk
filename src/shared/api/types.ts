export type ScanStatus = "idle" | "scanning" | "completed" | "failed" | "cancelled";
export type EntryType = "all" | "file" | "folder";

export type ScanSessionDto = {
  id: number;
  rootPath: string;
  status: ScanStatus;
  startedAt: string;
  completedAt?: string;
  totalFiles: number;
  totalFolders: number;
  totalSize: number;
  skippedItems: number;
  elapsedMs: number;
};

export type FileEntryDto = {
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

export type ScanIssueDto = {
  id: number;
  scanSessionId: number;
  path: string;
  errorKind: string;
  errorMessage?: string;
  createdAt: string;
};

export type ExtensionSummaryDto = {
  extension: string;
  totalSize: number;
  fileCount: number;
  percentageOfScan: number;
};

export type TreemapNodeDto = {
  id: number | null;
  name: string;
  path: string;
  size: number;
  isDirectory: boolean;
  percentageOfParent: number;
  children?: TreemapNodeDto[];
};

export type FilterSetDto = {
  query?: string;
  entryType?: EntryType;
  extension?: string;
  extensionGroup?: "videos" | "archives" | "disk_images" | "installers" | "documents";
  minSize?: number;
  maxSize?: number;
  modifiedFrom?: string;
  modifiedTo?: string;
  limit?: number;
  offset?: number;
};

export type StartScanInput = { path: string };
export type StartScanResponse = { scanSessionId: number };
export type CancelScanInput = { scanSessionId: number };
export type GetScanSessionInput = { scanSessionId: number };

export type ListScanSessionsInput = { limit: number; offset: number };
export type ListScanSessionsResponse = { items: ScanSessionDto[] };

export type GetChildrenInput = {
  scanSessionId: number;
  parentId: number | null;
  sortBy: "size" | "name" | "modified_at" | "type";
  sortDirection: "asc" | "desc";
  limit: number;
  offset: number;
};

export type GetEntriesResponse = { items: FileEntryDto[] };

export type GetLargestFilesInput = {
  scanSessionId: number;
  limit: 100 | 500 | 1000;
  offset: number;
  sortBy?: "size" | "name" | "extension" | "modified_at";
  sortDirection?: "asc" | "desc";
  filters?: FilterSetDto;
};

export type GetLargestFoldersInput = {
  scanSessionId: number;
  limit: number;
  offset: number;
  sortBy?: "size" | "name" | "modified_at";
  sortDirection?: "asc" | "desc";
};

export type SearchEntriesInput = {
  scanSessionId: number;
  query: string;
  entryType: EntryType;
  limit: number;
  offset: number;
  filters?: FilterSetDto;
};

export type GetExtensionSummaryInput = { scanSessionId: number };
export type GetExtensionSummaryResponse = { items: ExtensionSummaryDto[] };

export type GetTreemapDataInput = {
  scanSessionId: number;
  parentId: number | null;
  limit: number;
};

export type GetScanErrorsInput = { scanSessionId: number; limit: number; offset: number };
export type GetScanErrorsResponse = { items: ScanIssueDto[] };

export type RevealInExplorerInput = { path: string };
