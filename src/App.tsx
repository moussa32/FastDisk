import "./App.css";
import { EmptyState, PercentageBar, SizeBadge } from "./shared/components";

const resultTabs = [
  "Tree View",
  "Largest Files",
  "Largest Folders",
  "Treemap",
  "Extensions",
  "Scan Errors",
];

function App() {
  return (
    <main className="app-shell">
      <section className="scan-panel" aria-labelledby="scan-heading">
        <div>
          <p className="eyebrow">FastDisk Viewer</p>
          <h1 id="scan-heading">Read-only disk usage analysis</h1>
          <p className="lede">
            Select a folder or drive, scan it locally, and inspect disk usage
            without deleting, moving, renaming, uploading, or modifying files.
          </p>
        </div>

        <div className="scan-controls" aria-label="Scan setup">
          <div className="path-preview">No path selected</div>
          <div className="button-row">
            <button type="button">Choose Path</button>
            <button type="button" disabled>
              Start Scan
            </button>
          </div>
        </div>
      </section>

      <section className="results-shell" aria-label="Scan results">
        <aside className="summary-panel">
          <h2>Scan Summary</h2>
          <dl>
            <div>
              <dt>Status</dt>
              <dd>Idle</dd>
            </div>
            <div>
              <dt>Total size</dt>
              <dd>
                <SizeBadge bytes={0} />
              </dd>
            </div>
            <div>
              <dt>Skipped items</dt>
              <dd>0</dd>
            </div>
          </dl>
          <PercentageBar value={0} />
        </aside>

        <section className="results-panel">
          <div className="tabs" role="tablist" aria-label="Result views">
            {resultTabs.map((tab, index) => (
              <button
                aria-selected={index === 0}
                className="tab"
                key={tab}
                role="tab"
                type="button"
              >
                {tab}
              </button>
            ))}
          </div>
          <EmptyState
            title="No scan results yet"
            description="Complete a scan to browse folders, largest items, treemap data, file types, and scan errors."
          />
        </section>
      </section>
    </main>
  );
}

export default App;
