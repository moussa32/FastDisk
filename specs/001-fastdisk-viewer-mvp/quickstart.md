# Quickstart: FastDisk Viewer MVP

## Prerequisites

- Node.js and pnpm available.
- Rust toolchain available for Tauri builds. In this shell, `rustc` and `cargo`
  were not on PATH, so install or expose Rust before running full Tauri builds.
- Windows desktop environment for Explorer reveal behavior.

## Install Dependencies

```powershell
pnpm install
```

## Run Frontend During Development

```powershell
pnpm dev
```

## Run Desktop App

```powershell
pnpm tauri dev
```

## Validate MVP Workflow

1. Open the desktop app.
2. Confirm no scan can start until a path is selected.
3. Select a small folder and start a scan.
4. Confirm progress shows current path, counts, discovered size, skipped count,
   elapsed time, and scan status.
5. Confirm the UI remains responsive while the scan is active.
6. Confirm scan completion shows a summary.
7. Open Tree View and expand folders; verify children load on demand.
8. Open Largest Files and Largest Folders; verify bounded, sorted rows.
9. Select a folder and open Treemap; verify drilldown, breadcrumb, tooltip, and
   capped child display.
10. Search for a partial name and apply extension or size filters.
11. Open Extension Summary and Scan Errors views.
12. Reveal an existing file or folder in Windows Explorer.
13. Try revealing a path that no longer exists and verify a friendly error.
14. Cancel a long scan and verify the session is marked cancelled with partial
    results clearly labeled when available.

## Constitution Checks During Manual QA

- No UI action deletes, moves, renames, compresses, uploads, cleans, or modifies
  user files.
- No view intentionally requests or renders the full scan tree.
- Large result views stay bounded, paginated, virtualized, or capped.
- Scan issues are recorded and understandable.
- All scan data remains local.
