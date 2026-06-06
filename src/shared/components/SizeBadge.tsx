import { formatBytes } from "../utils/format";

type SizeBadgeProps = {
  bytes: number;
};

export function SizeBadge({ bytes }: SizeBadgeProps) {
  return <span className="size-badge">{formatBytes(bytes)}</span>;
}
