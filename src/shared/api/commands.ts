import { invoke } from "@tauri-apps/api/core";
import type {
  CancelScanInput,
  FileEntryDto,
  GetChildrenInput,
  GetEntriesResponse,
  GetExtensionSummaryInput,
  GetExtensionSummaryResponse,
  GetLargestFilesInput,
  GetLargestFoldersInput,
  GetScanErrorsInput,
  GetScanErrorsResponse,
  GetScanSessionInput,
  GetTreemapDataInput,
  ListScanSessionsInput,
  ListScanSessionsResponse,
  RevealInExplorerInput,
  ScanSessionDto,
  SearchEntriesInput,
  StartScanInput,
  StartScanResponse,
  TreemapNodeDto,
} from "./types";

export const commandNames = {
  cancelScan: "cancel_scan",
  getChildren: "get_children",
  getExtensionSummary: "get_extension_summary",
  getLargestFiles: "get_largest_files",
  getLargestFolders: "get_largest_folders",
  getScanErrors: "get_scan_errors",
  getScanSession: "get_scan_session",
  getTreemapData: "get_treemap_data",
  listScanSessions: "list_scan_sessions",
  revealInExplorer: "reveal_in_explorer",
  searchEntries: "search_entries",
  startScan: "start_scan",
} as const;

export function startScan(input: StartScanInput) {
  return invoke<StartScanResponse>(commandNames.startScan, input);
}

export function cancelScan(input: CancelScanInput) {
  return invoke<void>(commandNames.cancelScan, input);
}

export function getScanSession(input: GetScanSessionInput) {
  return invoke<ScanSessionDto>(commandNames.getScanSession, input);
}

export function listScanSessions(input: ListScanSessionsInput) {
  return invoke<ListScanSessionsResponse>(commandNames.listScanSessions, input);
}

export function getChildren(input: GetChildrenInput) {
  return invoke<FileEntryDto[]>(commandNames.getChildren, { input });
}

export function getLargestFiles(input: GetLargestFilesInput) {
  return invoke<GetEntriesResponse>(commandNames.getLargestFiles, { input });
}

export function getLargestFolders(input: GetLargestFoldersInput) {
  return invoke<GetEntriesResponse>(commandNames.getLargestFolders, { input });
}

export function searchEntries(input: SearchEntriesInput) {
  return invoke<GetEntriesResponse>(commandNames.searchEntries, { input });
}

export function getExtensionSummary(input: GetExtensionSummaryInput) {
  return invoke<GetExtensionSummaryResponse>(commandNames.getExtensionSummary, input);
}

export function getTreemapData(input: GetTreemapDataInput) {
  return invoke<TreemapNodeDto>(commandNames.getTreemapData, { input });
}

export function getScanErrors(input: GetScanErrorsInput) {
  return invoke<GetScanErrorsResponse>(commandNames.getScanErrors, { input });
}

export function revealInExplorer(input: RevealInExplorerInput) {
  return invoke<void>(commandNames.revealInExplorer, input);
}
