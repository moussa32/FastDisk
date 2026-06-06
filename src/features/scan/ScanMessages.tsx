import { EmptyState, ErrorState } from "../../shared/components";
import type { ScanStatus } from "../../shared/api/types";

type ScanMessagesProps = {
  status: ScanStatus;
  errorMessage?: string;
  skippedItems: number;
};

export function ScanMessages({ status, errorMessage, skippedItems }: ScanMessagesProps) {
  if (errorMessage) {
    return <ErrorState message={errorMessage} title="Scan failed" />;
  }

  if (status === "cancelled") {
    return (
      <EmptyState
        title="Scan cancelled"
        description="Partial results may be available after cancellation."
      />
    );
  }

  if (skippedItems > 0) {
    return (
      <EmptyState
        title="Some items were skipped"
        description={`${skippedItems} paths could not be scanned because of permissions, locks, or missing files.`}
      />
    );
  }

  if (status === "idle") {
    return (
      <EmptyState
        title="Choose a path"
        description="Select a folder or drive to begin read-only disk analysis."
      />
    );
  }

  return null;
}
