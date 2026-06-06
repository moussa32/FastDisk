import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import {
  cancelScan,
  getScanSession,
  startScan,
} from "./commands";
import type {
  CancelScanInput,
  GetScanSessionInput,
  ScanSessionDto,
  StartScanInput,
  StartScanResponse,
} from "./types";

export type ScanProgressEvent = {
  scanSessionId: number;
  currentPath: string;
  filesScanned: number;
  foldersScanned: number;
  bytesScanned: number;
  skippedItems: number;
  elapsedMs: number;
};

export type ScanCompletedEvent = {
  scanSessionId: number;
  totalFiles: number;
  totalFolders: number;
  totalSize: number;
  skippedItems: number;
  elapsedMs: number;
};

export function startScanSession(input: StartScanInput): Promise<StartScanResponse> {
  return startScan(input);
}

export function cancelScanSession(input: CancelScanInput): Promise<void> {
  return cancelScan(input);
}

export function loadScanSession(input: GetScanSessionInput): Promise<ScanSessionDto> {
  return getScanSession(input);
}

export function onScanProgress(callback: (event: ScanProgressEvent) => void): Promise<UnlistenFn> {
  return listen<ScanProgressEvent>("scan-progress", (event) => callback(event.payload));
}

export function onScanCompleted(callback: (event: ScanCompletedEvent) => void): Promise<UnlistenFn> {
  return listen<ScanCompletedEvent>("scan-completed", (event) => callback(event.payload));
}
