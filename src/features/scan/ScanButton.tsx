import type { ScanStatus } from "../../shared/api/types";

type ScanButtonProps = {
  disabled: boolean;
  status: ScanStatus;
  onCancel: () => void;
  onStart: () => void;
};

export function ScanButton({ disabled, status, onCancel, onStart }: ScanButtonProps) {
  if (status === "scanning") {
    return (
      <button type="button" onClick={onCancel}>
        Cancel Scan
      </button>
    );
  }

  return (
    <button type="button" disabled={disabled} onClick={onStart}>
      Start Scan
    </button>
  );
}
