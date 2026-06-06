import { formatPercentage } from "../utils/format";

type PercentageBarProps = {
  value: number;
};

export function PercentageBar({ value }: PercentageBarProps) {
  const safeValue = Math.max(0, Math.min(100, value));

  return (
    <div className="percentage-bar" aria-label={formatPercentage(safeValue)}>
      <span style={{ width: `${safeValue}%` }} />
    </div>
  );
}
