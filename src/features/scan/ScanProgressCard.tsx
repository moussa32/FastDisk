import { formatBytes } from "../../shared/utils/format";
import type { ScanProgressEvent } from "../../shared/api/scan";

type ScanProgressCardProps = {
  progress?: ScanProgressEvent;
};

export function ScanProgressCard({ progress }: ScanProgressCardProps) {
  return (
    <section className="progress-card" aria-label="Scan progress">
      <h2>Progress</h2>
      <dl>
        <div>
          <dt>Current path</dt>
          <dd>{progress?.currentPath || "-"}</dd>
        </div>
        <div>
          <dt>Files</dt>
          <dd>{progress?.filesScanned ?? 0}</dd>
        </div>
        <div>
          <dt>Folders</dt>
          <dd>{progress?.foldersScanned ?? 0}</dd>
        </div>
        <div>
          <dt>Discovered size</dt>
          <dd>{formatBytes(progress?.bytesScanned ?? 0)}</dd>
        </div>
        <div>
          <dt>Skipped</dt>
          <dd>{progress?.skippedItems ?? 0}</dd>
        </div>
      </dl>
    </section>
  );
}
