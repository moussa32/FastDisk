import { getLargestFiles, getLargestFolders } from "./commands";
import type {
  FileEntryDto,
  GetLargestFilesInput,
  GetLargestFoldersInput,
} from "./types";

export const largestQueryKeys = {
  files: (input: GetLargestFilesInput) => [
    "largest-files",
    input.scanSessionId,
    input.limit,
    input.offset,
    input.sortBy ?? "size",
    input.sortDirection ?? "desc",
    input.filters?.extension ?? "",
    input.filters?.extensionGroup ?? "",
    input.filters?.minSize ?? "",
    input.filters?.maxSize ?? "",
  ] as const,
  folders: (input: GetLargestFoldersInput) => [
    "largest-folders",
    input.scanSessionId,
    input.limit,
    input.offset,
    input.sortBy ?? "size",
    input.sortDirection ?? "desc",
  ] as const,
};

export function loadLargestFiles(input: GetLargestFilesInput): Promise<FileEntryDto[]> {
  return getLargestFiles(input).then((response) => response.items);
}

export function loadLargestFolders(input: GetLargestFoldersInput): Promise<FileEntryDto[]> {
  return getLargestFolders(input).then((response) => response.items);
}
