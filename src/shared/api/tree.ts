import { getChildren } from "./commands";
import type { FileEntryDto, GetChildrenInput } from "./types";

export const treeQueryKeys = {
  children: (input: GetChildrenInput) => [
    "children",
    input.scanSessionId,
    input.parentId,
    input.sortBy,
    input.sortDirection,
    input.limit,
    input.offset,
  ] as const,
};

export function loadChildren(input: GetChildrenInput): Promise<FileEntryDto[]> {
  return getChildren(input);
}
