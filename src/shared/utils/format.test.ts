import { describe, expect, it } from "vitest";
import { formatBytes, formatDate, formatPercentage, truncatePath } from "./format";

describe("formatBytes", () => {
  it("formats bytes using binary units", () => {
    expect(formatBytes(0)).toBe("0 B");
    expect(formatBytes(1024)).toBe("1 KB");
    expect(formatBytes(1536 * 1024)).toBe("1.5 MB");
  });
});

describe("formatPercentage", () => {
  it("clamps and formats percentages", () => {
    expect(formatPercentage(12.345)).toBe("12.3%");
    expect(formatPercentage(130)).toBe("100.0%");
    expect(formatPercentage(-1)).toBe("0.0%");
  });
});

describe("formatDate", () => {
  it("returns a placeholder for missing or invalid dates", () => {
    expect(formatDate()).toBe("-");
    expect(formatDate("not-a-date")).toBe("-");
  });
});

describe("truncatePath", () => {
  it("keeps short paths and truncates long paths", () => {
    expect(truncatePath("C:/Users")).toBe("C:/Users");
    expect(truncatePath("C:/Users/FastDisk/Downloads/Very/Large/File.iso", 24)).toContain("...");
  });
});
