import { useSyncExternalStore } from "react";

type TreeSelection = {
  selectedEntryId: number | null;
  selectedPath: string | null;
};

let selection: TreeSelection = {
  selectedEntryId: null,
  selectedPath: null,
};

const listeners = new Set<() => void>();

export function setSelectedFolder(selectedEntryId: number | null, selectedPath: string | null) {
  selection = { selectedEntryId, selectedPath };
  for (const listener of listeners) {
    listener();
  }
}

export function getSelectedFolderSnapshot() {
  return selection;
}

export function subscribeToSelectedFolder(listener: () => void) {
  listeners.add(listener);
  return () => listeners.delete(listener);
}

export function useSelectedFolder() {
  return useSyncExternalStore(subscribeToSelectedFolder, getSelectedFolderSnapshot);
}
