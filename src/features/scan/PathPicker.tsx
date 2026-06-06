import { open } from "@tauri-apps/plugin-dialog";

type PathPickerProps = {
  selectedPath: string;
  onPathSelected: (path: string) => void;
};

export function PathPicker({ selectedPath, onPathSelected }: PathPickerProps) {
  async function choosePath() {
    const selected = await open({
      directory: true,
      multiple: false,
      title: "Choose a folder or drive to scan",
    });
    if (typeof selected === "string") {
      onPathSelected(selected);
    }
  }

  return (
    <div className="scan-controls" aria-label="Scan setup">
      <div className="path-preview" title={selectedPath || "No path selected"}>
        {selectedPath || "No path selected"}
      </div>
      <button type="button" onClick={choosePath}>
        Choose Path
      </button>
    </div>
  );
}
