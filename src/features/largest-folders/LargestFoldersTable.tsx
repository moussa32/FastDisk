import { useEffect, useState } from "react";
import { ErrorState, FileTable, LoadingState } from "../../shared/components";
import { loadLargestFolders } from "../../shared/api/largest";
import { revealPath } from "../../shared/api/reveal";
import type { FileEntryDto, GetLargestFoldersInput } from "../../shared/api/types";

type LargestFoldersTableProps = {
  scanSessionId: number | null;
  loadRows?: (input: GetLargestFoldersInput) => Promise<FileEntryDto[]>;
  revealEntry?: (path: string) => Promise<void>;
};

export function LargestFoldersTable({
  scanSessionId,
  loadRows = loadLargestFolders,
  revealEntry = (path) => revealPath({ path }),
}: LargestFoldersTableProps) {
  const [entries, setEntries] = useState<FileEntryDto[]>([]);
  const [limit, setLimit] = useState(100);
  const [sortBy, setSortBy] = useState<GetLargestFoldersInput["sortBy"]>("size");
  const [sortDirection, setSortDirection] = useState<"asc" | "desc">("desc");
  const [isLoading, setIsLoading] = useState(false);
  const [errorMessage, setErrorMessage] = useState<string>();
  const [revealingPath, setRevealingPath] = useState<string | null>(null);

  useEffect(() => {
    if (scanSessionId === null) {
      setEntries([]);
      return;
    }

    let isCurrent = true;
    setIsLoading(true);
    setErrorMessage(undefined);
    loadRows({
      scanSessionId,
      limit,
      offset: 0,
      sortBy,
      sortDirection,
    })
      .then((rows) => {
        if (isCurrent) {
          setEntries(rows);
        }
      })
      .catch(() => {
        if (isCurrent) {
          setErrorMessage("Largest folders could not be loaded.");
        }
      })
      .finally(() => {
        if (isCurrent) {
          setIsLoading(false);
        }
      });

    return () => {
      isCurrent = false;
    };
  }, [limit, loadRows, scanSessionId, sortBy, sortDirection]);

  async function reveal(entry: FileEntryDto) {
    setRevealingPath(entry.path);
    setErrorMessage(undefined);
    try {
      await revealEntry(entry.path);
    } catch {
      setErrorMessage("That folder no longer exists, so Explorer could not reveal it.");
    } finally {
      setRevealingPath(null);
    }
  }

  return (
    <section className="result-view" aria-label="Largest folders">
      <div className="table-toolbar">
        <label>
          Rows
          <input
            aria-label="Folder row limit"
            min="1"
            max="1000"
            onChange={(event) => setLimit(Number(event.target.value))}
            type="number"
            value={limit}
          />
        </label>
        <label>
          Sort
          <select value={sortBy} onChange={(event) => setSortBy(event.target.value as typeof sortBy)}>
            <option value="size">Size</option>
            <option value="name">Name</option>
            <option value="modified_at">Modified</option>
          </select>
        </label>
        <label>
          Direction
          <select value={sortDirection} onChange={(event) => setSortDirection(event.target.value as "asc" | "desc")}>
            <option value="desc">Desc</option>
            <option value="asc">Asc</option>
          </select>
        </label>
      </div>

      {errorMessage ? <ErrorState title="Largest folders unavailable" message={errorMessage} /> : null}
      {isLoading ? <LoadingState label="Loading largest folders" /> : null}
      {!isLoading ? (
        <FileTable
          emptyTitle="No folders returned"
          emptyDescription="Complete a scan with folders before opening this view."
          entries={entries}
          onReveal={reveal}
          revealingPath={revealingPath}
          showFolderCounts
        />
      ) : null}
    </section>
  );
}
