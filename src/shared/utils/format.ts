const units = ["B", "KB", "MB", "GB", "TB"] as const;

export function formatBytes(bytes: number): string {
  if (!Number.isFinite(bytes) || bytes <= 0) {
    return "0 B";
  }

  let value = bytes;
  let unitIndex = 0;

  while (value >= 1024 && unitIndex < units.length - 1) {
    value /= 1024;
    unitIndex += 1;
  }

  const precision = value % 1 === 0 || value >= 10 || unitIndex === 0 ? 0 : 1;
  return `${value.toFixed(precision)} ${units[unitIndex]}`;
}

export function formatPercentage(value: number): string {
  if (!Number.isFinite(value)) {
    return "0%";
  }
  return `${Math.max(0, Math.min(100, value)).toFixed(1)}%`;
}

export function formatDate(value?: string): string {
  if (!value) {
    return "-";
  }

  const date = new Date(value);
  if (Number.isNaN(date.getTime())) {
    return "-";
  }

  return new Intl.DateTimeFormat(undefined, {
    dateStyle: "medium",
    timeStyle: "short",
  }).format(date);
}

export function truncatePath(path: string, maxLength = 96): string {
  if (path.length <= maxLength) {
    return path;
  }

  const headLength = Math.max(12, Math.floor((maxLength - 3) / 2));
  const tailLength = Math.max(12, maxLength - headLength - 3);
  return `${path.slice(0, headLength)}...${path.slice(-tailLength)}`;
}
