import "@testing-library/jest-dom/vitest";
import { cleanup, fireEvent, screen, waitFor } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";
import type { FileEntryDto } from "../../shared/api/types";
import { renderApp } from "../../shared/test/render";
import { LargestFilesTable } from "./LargestFilesTable";

const fileRow: FileEntryDto = {
  id: 1,
  scanSessionId: 10,
  parentId: null,
  name: "archive.zip",
  path: "C:/scan/archive.zip",
  size: 2048,
  isDirectory: false,
  extension: ".zip",
  depth: 1,
  modifiedAt: "2026-06-07T00:00:00Z",
  isSymlink: false,
  childCount: 0,
  descendantCount: 0,
};

describe("LargestFilesTable", () => {
  afterEach(() => {
    cleanup();
  });

  it("renders bounded file rows", async () => {
    const loadRows = vi.fn().mockResolvedValue([fileRow]);
    renderApp(<LargestFilesTable scanSessionId={10} loadRows={loadRows} />);

    expect(await screen.findByText("archive.zip")).toBeInTheDocument();
    expect(screen.getByText("2 KB")).toBeInTheDocument();
    expect(screen.getByText(".zip")).toBeInTheDocument();
  });

  it("passes sorting, limit, and filters to the loader", async () => {
    const loadRows = vi.fn().mockResolvedValue([fileRow]);
    renderApp(<LargestFilesTable scanSessionId={10} loadRows={loadRows} />);
    await screen.findByText("archive.zip");

    fireEvent.change(screen.getByLabelText("Extension filter"), { target: { value: "zip" } });
    fireEvent.change(screen.getByLabelText("Minimum size"), { target: { value: "1000" } });
    fireEvent.change(screen.getByDisplayValue("100"), { target: { value: "500" } });
    fireEvent.change(screen.getByDisplayValue("Size"), { target: { value: "name" } });
    fireEvent.change(screen.getByDisplayValue("Desc"), { target: { value: "asc" } });

    await waitFor(() =>
      expect(loadRows).toHaveBeenLastCalledWith(
        expect.objectContaining({
          scanSessionId: 10,
          limit: 500,
          sortBy: "name",
          sortDirection: "asc",
          filters: expect.objectContaining({
            extension: "zip",
            minSize: 1000,
          }),
        }),
      ),
    );
  });

  it("shows reveal progress and missing-path errors", async () => {
    let rejectReveal: (error: Error) => void = () => {};
    const revealEntry = vi.fn(
      () =>
        new Promise<void>((_resolve, reject) => {
          rejectReveal = reject;
        }),
    );
    const loadRows = vi.fn().mockResolvedValue([fileRow]);
    renderApp(<LargestFilesTable scanSessionId={10} loadRows={loadRows} revealEntry={revealEntry} />);

    fireEvent.click(await screen.findByRole("button", { name: "Reveal" }));

    expect(screen.getByRole("button", { name: "Opening" })).toBeDisabled();
    rejectReveal(new Error("missing"));
    expect(await screen.findByText("That item no longer exists, so Explorer could not reveal it.")).toBeInTheDocument();
  });
});
