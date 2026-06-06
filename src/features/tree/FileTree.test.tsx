import "@testing-library/jest-dom/vitest";
import { cleanup, fireEvent, screen, waitFor } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";
import { renderApp } from "../../shared/test/render";
import type { FileEntryDto } from "../../shared/api/types";
import { FileTree } from "./FileTree";
import { getSelectedFolderSnapshot, setSelectedFolder } from "./treeState";

const root: FileEntryDto = {
  id: 1,
  scanSessionId: 10,
  parentId: null,
  name: "Root",
  path: "C:/Root",
  size: 2048,
  isDirectory: true,
  depth: 0,
  isSymlink: false,
  childCount: 2,
  descendantCount: 2,
};

const child: FileEntryDto = {
  id: 2,
  scanSessionId: 10,
  parentId: 1,
  name: "Child.txt",
  path: "C:/Root/Child.txt",
  size: 1024,
  isDirectory: false,
  extension: "txt",
  depth: 1,
  isSymlink: false,
  childCount: 0,
  descendantCount: 0,
};

describe("FileTree", () => {
  afterEach(() => {
    cleanup();
    setSelectedFolder(null, null);
  });

  it("renders root node", () => {
    renderApp(<FileTree root={root} scanSessionId={10} />);
    expect(screen.getByRole("button", { name: "Root" })).toBeInTheDocument();
    expect(screen.getByText("2 KB")).toBeInTheDocument();
  });

  it("lazy loads children on expand", async () => {
    const loadChildrenForNode = vi.fn().mockResolvedValue([child]);
    renderApp(<FileTree root={root} scanSessionId={10} loadChildrenForNode={loadChildrenForNode} />);

    fireEvent.click(screen.getByRole("button", { name: "Expand Root" }));

    await waitFor(() => expect(loadChildrenForNode).toHaveBeenCalledWith(1));
    expect(await screen.findByRole("button", { name: "Child.txt" })).toBeInTheDocument();
  });

  it("supports collapse after expanding", async () => {
    const loadChildrenForNode = vi.fn().mockResolvedValue([child]);
    renderApp(<FileTree root={root} scanSessionId={10} loadChildrenForNode={loadChildrenForNode} />);

    fireEvent.click(screen.getByRole("button", { name: "Expand Root" }));
    expect(await screen.findByRole("button", { name: "Child.txt" })).toBeInTheDocument();
    fireEvent.click(screen.getByRole("button", { name: "Collapse Root" }));

    await waitFor(() => expect(screen.queryByRole("button", { name: "Child.txt" })).not.toBeInTheDocument());
  });

  it("updates selected folder state", () => {
    renderApp(<FileTree root={root} scanSessionId={10} />);

    fireEvent.click(screen.getByRole("button", { name: "Root" }));

    expect(getSelectedFolderSnapshot()).toEqual({
      selectedEntryId: 1,
      selectedPath: "C:/Root",
    });
  });

  it("renders empty state without root", () => {
    renderApp(<FileTree scanSessionId={null} />);
    expect(screen.getByText("No tree data yet")).toBeInTheDocument();
  });
});
