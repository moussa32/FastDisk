import "@testing-library/jest-dom/vitest";
import { describe, expect, it, vi } from "vitest";
import { screen } from "@testing-library/react";
import { renderApp } from "../../shared/test/render";
import { ScanButton } from "./ScanButton";
import { ScanProgressCard } from "./ScanProgressCard";
import { ScanSummaryCard } from "./ScanSummaryCard";

describe("ScanButton", () => {
  it("disables start when no path is selected", () => {
    renderApp(<ScanButton disabled onCancel={vi.fn()} onStart={vi.fn()} status="idle" />);
    expect(screen.getByRole("button", { name: "Start Scan" })).toBeDisabled();
  });

  it("shows cancel while scanning", () => {
    renderApp(<ScanButton disabled={false} onCancel={vi.fn()} onStart={vi.fn()} status="scanning" />);
    expect(screen.getByRole("button", { name: "Cancel Scan" })).toBeInTheDocument();
  });
});

describe("ScanProgressCard", () => {
  it("renders progress values", () => {
    renderApp(
      <ScanProgressCard
        progress={{
          scanSessionId: 1,
          currentPath: "C:/tmp",
          filesScanned: 2,
          foldersScanned: 1,
          bytesScanned: 1024,
          skippedItems: 0,
          elapsedMs: 50,
        }}
      />,
    );
    expect(screen.getByText("C:/tmp")).toBeInTheDocument();
    expect(screen.getByText("1 KB")).toBeInTheDocument();
  });
});

describe("ScanSummaryCard", () => {
  it("renders completed summary values", () => {
    renderApp(
      <ScanSummaryCard
        session={{
          id: 1,
          rootPath: "C:/tmp",
          status: "completed",
          startedAt: "2026-06-07T00:00:00Z",
          totalFiles: 3,
          totalFolders: 1,
          totalSize: 2048,
          skippedItems: 1,
          elapsedMs: 100,
        }}
      />,
    );
    expect(screen.getByText("completed")).toBeInTheDocument();
    expect(screen.getByText("2 KB")).toBeInTheDocument();
  });
});
