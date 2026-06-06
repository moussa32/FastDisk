import { useEffect, useState } from "react";
import { EmptyState } from "../../shared/components";
import {
  cancelScanSession,
  loadScanSession,
  onScanCompleted,
  onScanProgress,
  startScanSession,
  type ScanProgressEvent,
} from "../../shared/api/scan";
import type { ScanSessionDto, ScanStatus } from "../../shared/api/types";
import { PathPicker } from "./PathPicker";
import { ScanButton } from "./ScanButton";
import { ScanMessages } from "./ScanMessages";
import { ScanProgressCard } from "./ScanProgressCard";
import { ScanSummaryCard } from "./ScanSummaryCard";

const resultTabs = [
  "Tree View",
  "Largest Files",
  "Largest Folders",
  "Treemap",
  "Extensions",
  "Scan Errors",
];

export function ScanScreen() {
  const [selectedPath, setSelectedPath] = useState("");
  const [status, setStatus] = useState<ScanStatus>("idle");
  const [activeScanSessionId, setActiveScanSessionId] = useState<number | null>(null);
  const [progress, setProgress] = useState<ScanProgressEvent>();
  const [session, setSession] = useState<ScanSessionDto>();
  const [errorMessage, setErrorMessage] = useState<string>();

  useEffect(() => {
    const unlisteners: Array<() => void> = [];
    onScanProgress((event) => {
      setProgress(event);
      setStatus("scanning");
    }).then((unlisten) => unlisteners.push(unlisten));
    onScanCompleted((event) => {
      setStatus("completed");
      setActiveScanSessionId(event.scanSessionId);
      void refreshSession(event.scanSessionId);
    }).then((unlisten) => unlisteners.push(unlisten));

    return () => {
      for (const unlisten of unlisteners) {
        unlisten();
      }
    };
  }, []);

  async function refreshSession(scanSessionId: number) {
    const nextSession = await loadScanSession({ scanSessionId });
    setSession(nextSession);
    setStatus(nextSession.status);
  }

  async function startScan() {
    if (!selectedPath) {
      setErrorMessage("Choose a folder or drive before starting a scan.");
      return;
    }

    setErrorMessage(undefined);
    setStatus("scanning");
    try {
      const response = await startScanSession({ path: selectedPath });
      setActiveScanSessionId(response.scanSessionId);
      await refreshSession(response.scanSessionId);
    } catch (error) {
      setStatus("failed");
      setErrorMessage(error instanceof Error ? error.message : "The scan could not be started.");
    }
  }

  async function cancelScan() {
    if (activeScanSessionId === null) {
      return;
    }
    await cancelScanSession({ scanSessionId: activeScanSessionId });
    setStatus("cancelled");
  }

  return (
    <main className="app-shell">
      <section className="scan-panel" aria-labelledby="scan-heading">
        <div>
          <p className="eyebrow">FastDisk Viewer</p>
          <h1 id="scan-heading">Read-only disk usage analysis</h1>
          <p className="lede">
            Select a folder or drive, scan it locally, and inspect disk usage
            without deleting, moving, renaming, uploading, or modifying files.
          </p>
        </div>

        <div className="scan-actions">
          <PathPicker selectedPath={selectedPath} onPathSelected={setSelectedPath} />
          <ScanButton
            disabled={!selectedPath}
            onCancel={cancelScan}
            onStart={startScan}
            status={status}
          />
        </div>
      </section>

      <section className="results-shell" aria-label="Scan results">
        <ScanSummaryCard session={session} />

        <section className="results-panel">
          <div className="tabs" role="tablist" aria-label="Result views">
            {resultTabs.map((tab, index) => (
              <button
                aria-selected={index === 0}
                className="tab"
                key={tab}
                role="tab"
                type="button"
              >
                {tab}
              </button>
            ))}
          </div>

          {status === "scanning" ? (
            <ScanProgressCard progress={progress} />
          ) : (
            <EmptyState
              title="No scan results yet"
              description="Complete a scan to browse folders, largest items, treemap data, file types, and scan errors."
            />
          )}
        </section>
      </section>

      <ScanMessages
        errorMessage={errorMessage}
        skippedItems={session?.skippedItems ?? progress?.skippedItems ?? 0}
        status={status}
      />
    </main>
  );
}
