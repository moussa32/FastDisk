import type { FileEntryDto } from "../api/types";
import { formatDate, truncatePath } from "../utils/format";
import { EmptyState } from "./EmptyState";
import { SizeBadge } from "./SizeBadge";

type FileTableProps = {
  entries: FileEntryDto[];
  emptyTitle: string;
  emptyDescription: string;
  showFolderCounts?: boolean;
  revealLabel?: string;
  revealingPath?: string | null;
  onReveal?: (entry: FileEntryDto) => void;
};

export function FileTable({
  entries,
  emptyTitle,
  emptyDescription,
  showFolderCounts = false,
  revealLabel = "Reveal",
  revealingPath = null,
  onReveal,
}: FileTableProps) {
  if (entries.length === 0) {
    return <EmptyState title={emptyTitle} description={emptyDescription} />;
  }

  return (
    <div className="file-table-viewport">
      <table className="file-table">
        <thead>
          <tr>
            <th>Name</th>
            <th>Type</th>
            <th>Size</th>
            {showFolderCounts ? <th>Contents</th> : <th>Extension</th>}
            <th>Modified</th>
            <th>Path</th>
            {onReveal ? <th>Action</th> : null}
          </tr>
        </thead>
        <tbody>
          {entries.map((entry) => (
            <tr key={entry.id}>
              <td className="file-table-name" title={entry.name}>
                {entry.name}
              </td>
              <td>{entry.isDirectory ? "Folder" : "File"}</td>
              <td>
                <SizeBadge bytes={entry.size} />
              </td>
              {showFolderCounts ? (
                <td>
                  {entry.childCount} direct, {entry.descendantCount} total
                </td>
              ) : (
                <td>{entry.extension || "-"}</td>
              )}
              <td>{formatDate(entry.modifiedAt)}</td>
              <td className="file-table-path" title={entry.path}>
                {truncatePath(entry.path, 72)}
              </td>
              {onReveal ? (
                <td>
                  <button
                    disabled={revealingPath === entry.path}
                    onClick={() => onReveal(entry)}
                    type="button"
                  >
                    {revealingPath === entry.path ? "Opening" : revealLabel}
                  </button>
                </td>
              ) : null}
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
}
