import { useEffect, useState } from "react";
import { ErrorState, FileTable, LoadingState } from "../../shared/components";
import { loadLargestFiles } from "../../shared/api/largest";
import { revealPath } from "../../shared/api/reveal";
import type { FileEntryDto, FilterSetDto, GetLargestFilesInput } from "../../shared/api/types";

type LargestFilesTableProps = {
  scanSessionId: number | null;
  loadRows?: (input: GetLargestFilesInput) => Promise<FileEntryDto[]>;
  revealEntry?: (path: string) => Promise<void>;
};

const fileLimits = [100, 500, 1000] as const;

export function LargestFilesTable({
  scanSessionId,
  loadRows = loadLargestFiles,
  revealEntry = (path) => revealPath({ path }),
}: LargestFilesTableProps) {
  const [entries, setEntries] = useState<FileEntryDto[]>([]);
  const [limit, setLimit] = useState<(typeof fileLimits)[number]>(100);
  const [sortBy, setSortBy] = useState<GetLargestFilesInput["sortBy"]>("size");
  const [sortDirection, setSortDirection] = useState<"asc" | "desc">("desc");
  const [extension, setExtension] = useState("");
  const [minSize, setMinSize] = useState("");
  const [maxSize, setMaxSize] = useState("");
  const [isLoading, setIsLoading] = useState(false);
  const [errorMessage, setErrorMessage] = useState<string>();
  const [revealingPath, setRevealingPath] = useState<string | null>(null);

  useEffect(() => {
    if (scanSessionId === null) {
      setEntries([]);
      return;
    }

    const filters: FilterSetDto = {
      extension: extension || undefined,
      minSize: minSize ? Number(minSize) : undefined,
      maxSize: maxSize ? Number(maxSize) : undefined,
    };
    let isCurrent = true;
    setIsLoading(true);
    setErrorMessage(undefined);
    loadRows({
      scanSessionId,
      limit,
      offset: 0,
      sortBy,
      sortDirection,
      filters,
    })
      .then((rows) => {
        if (isCurrent) {
          setEntries(rows);
        }
      })
      .catch(() => {
        if (isCurrent) {
          setErrorMessage("Largest files could not be loaded.");
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
  }, [extension, limit, loadRows, maxSize, minSize, scanSessionId, sortBy, sortDirection]);

  async function reveal(entry: FileEntryDto) {
    setRevealingPath(entry.path);
    setErrorMessage(undefined);
    try {
      await revealEntry(entry.path);
    } catch {
      setErrorMessage("That item no longer exists, so Explorer could not reveal it.");
    } finally {
      setRevealingPath(null);
    }
  }

  return (
    <section className="result-view" aria-label="Largest files">
      <div className="table-toolbar">
        <label>
          Rows
          <select value={limit} onChange={(event) => setLimit(Number(event.target.value) as typeof limit)}>
            {fileLimits.map((value) => (
              <option key={value} value={value}>
                {value}
              </option>
            ))}
          </select>
        </label>
        <label>
          Sort
          <select value={sortBy} onChange={(event) => setSortBy(event.target.value as typeof sortBy)}>
            <option value="size">Size</option>
            <option value="name">Name</option>
            <option value="extension">Extension</option>
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
        <label>
          Extension
          <input
            aria-label="Extension filter"
            onChange={(event) => setExtension(event.target.value)}
            placeholder=".log"
            value={extension}
          />
        </label>
        <label>
          Min bytes
          <input
            aria-label="Minimum size"
            min="0"
            onChange={(event) => setMinSize(event.target.value)}
            type="number"
            value={minSize}
          />
        </label>
        <label>
          Max bytes
          <input
            aria-label="Maximum size"
            min="0"
            onChange={(event) => setMaxSize(event.target.value)}
            type="number"
            value={maxSize}
          />
        </label>
      </div>

      {errorMessage ? <ErrorState title="Largest files unavailable" message={errorMessage} /> : null}
      {isLoading ? <LoadingState label="Loading largest files" /> : null}
      {!isLoading ? (
        <FileTable
          emptyTitle="No files returned"
          emptyDescription="Try a broader filter or complete a scan with files."
          entries={entries}
          onReveal={reveal}
          revealingPath={revealingPath}
        />
      ) : null}
    </section>
  );
}
