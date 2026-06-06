import "@testing-library/jest-dom/vitest";
import { cleanup, fireEvent, screen, waitFor } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";
import type { FileEntryDto } from "../../shared/api/types";
import { renderApp } from "../../shared/test/render";
import { LargestFoldersTable } from "./LargestFoldersTable";

const folderRow: FileEntryDto = {
  id: 2,
  scanSessionId: 10,
  parentId: null,
  name: "Media",
  path: "C:/scan/Media",
  size: 4096,
  isDirectory: true,
  depth: 1,
  modifiedAt: "2026-06-07T00:00:00Z",
  isSymlink: false,
  childCount: 3,
  descendantCount: 9,
};

describe("LargestFoldersTable", () => {
  afterEach(() => {
    cleanup();
  });

  it("renders folder rows with child and descendant counts", async () => {
    const loadRows = vi.fn().mockResolvedValue([folderRow]);
    renderApp(<LargestFoldersTable scanSessionId={10} loadRows={loadRows} />);

    expect(await screen.findByText("Media")).toBeInTheDocument();
    expect(screen.getByText("4 KB")).toBeInTheDocument();
    expect(screen.getByText("3 direct, 9 total")).toBeInTheDocument();
  });

  it("passes sorting and limits to the loader", async () => {
    const loadRows = vi.fn().mockResolvedValue([folderRow]);
    renderApp(<LargestFoldersTable scanSessionId={10} loadRows={loadRows} />);
    await screen.findByText("Media");

    fireEvent.change(screen.getByLabelText("Folder row limit"), { target: { value: "50" } });
    fireEvent.change(screen.getByDisplayValue("Size"), { target: { value: "name" } });
    fireEvent.change(screen.getByDisplayValue("Desc"), { target: { value: "asc" } });

    await waitFor(() =>
      expect(loadRows).toHaveBeenLastCalledWith(
        expect.objectContaining({
          scanSessionId: 10,
          limit: 50,
          sortBy: "name",
          sortDirection: "asc",
        }),
      ),
    );
  });

  it("shows reveal progress and missing folder errors", async () => {
    let rejectReveal: (error: Error) => void = () => {};
    const revealEntry = vi.fn(
      () =>
        new Promise<void>((_resolve, reject) => {
          rejectReveal = reject;
        }),
    );
    const loadRows = vi.fn().mockResolvedValue([folderRow]);
    renderApp(<LargestFoldersTable scanSessionId={10} loadRows={loadRows} revealEntry={revealEntry} />);

    fireEvent.click(await screen.findByRole("button", { name: "Reveal" }));

    expect(screen.getByRole("button", { name: "Opening" })).toBeDisabled();
    rejectReveal(new Error("missing"));
    expect(await screen.findByText("That folder no longer exists, so Explorer could not reveal it.")).toBeInTheDocument();
  });
});
