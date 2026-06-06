import { SizeBadge } from "../../shared/components";
import type { ScanSessionDto } from "../../shared/api/types";

type ScanSummaryCardProps = {
  session?: ScanSessionDto;
};

export function ScanSummaryCard({ session }: ScanSummaryCardProps) {
  return (
    <aside className="summary-panel">
      <h2>Scan Summary</h2>
      <dl>
        <div>
          <dt>Status</dt>
          <dd>{session?.status ?? "idle"}</dd>
        </div>
        <div>
          <dt>Total size</dt>
          <dd>
            <SizeBadge bytes={session?.totalSize ?? 0} />
          </dd>
        </div>
        <div>
          <dt>Files</dt>
          <dd>{session?.totalFiles ?? 0}</dd>
        </div>
        <div>
          <dt>Folders</dt>
          <dd>{session?.totalFolders ?? 0}</dd>
        </div>
        <div>
          <dt>Skipped items</dt>
          <dd>{session?.skippedItems ?? 0}</dd>
        </div>
      </dl>
    </aside>
  );
}
