# FastDisk Viewer

FastDisk Viewer is a Windows desktop disk usage analyzer built with Tauri,
React, TypeScript, Rust, and SQLite. It scans a selected folder or drive,
persists local metadata, aggregates folder sizes, and presents bounded views
that help users understand where disk space is being used.

## Read-Only Safety

FastDisk Viewer is read-only. The MVP must not delete, move, rename, compress,
clean up, upload, or modify user files. The only allowed OS-level action is
revealing an existing file or folder location in Windows Explorer.

The app operates locally and must not upload paths, filenames, metadata, or scan
results. AI assistant behavior, AI cleanup recommendations, cloud sync,
analytics, and online accounts are not part of the product scope.

## MVP Scope

The MVP focuses on local disk usage visibility:

- Select one folder or drive and run one active scan at a time.
- Persist scan sessions, file entries, and scan errors in SQLite.
- Continue scanning when individual paths cannot be accessed where practical.
- Aggregate folder sizes from descendant files.
- Show scan progress, completion summaries, skipped item counts, failed states,
  and cancellation state.
- Browse completed scans through scoped, bounded result queries.
- Reveal file or folder locations in Windows Explorer without modifying them.

## Planned Features

The active specification plans these user-facing features:

- Scan selection, scan progress, cancellation, and summary views.
- Lazy-loaded Tree View with direct-child queries.
- Largest Files and Largest Folders tables with bounded result sizes.
- Treemap visualization with capped child rendering and folder drilldown.
- Search and filtering by partial name, item type, extension, and size range.
- Extension Summary for aggregated usage by file extension.
- Scan Errors view for skipped or inaccessible paths.
- Previous scan history loaded from local SQLite data.
- Friendly empty, invalid path, missing item, failure, and cancelled messages.

## Explicitly Out of Scope

The MVP does not include:

- Delete, move, rename, recycle bin, cleanup, compression, or auto-fix actions.
- AI assistant features or AI-powered cleanup recommendations.
- Cloud sync, online accounts, file upload, analytics, or telemetry.
- Background services or scheduled scans.
- Duplicate removal, registry cleaner, or system optimizer behavior.
- NTFS MFT scanning. The MVP uses a recursive Rust scanner first.

## Tech Stack

- Desktop shell: Tauri 2
- Frontend: React 19, TypeScript 5, Vite
- Backend: Rust 2021
- Data layer: SQLite through Rust
- Frontend query/state support: React Query
- Table and large-list support: TanStack Table and TanStack Virtual
- Tests: Vitest and Testing Library for frontend; Rust tests for backend logic

## Architecture Overview

Rust owns filesystem scanning, SQLite persistence, folder aggregation, and
production query logic. React must not scan the filesystem, access SQLite
directly, calculate final folder sizes for production behavior, or load the full
scan tree into memory.

Backend responsibilities live under `src-tauri/src/`:

- `scanner/`: read-only recursive scanning, scan session orchestration, progress,
  and cancellation.
- `db/`: SQLite connection setup, WAL pragmas, migrations, and indexes.
- `repository/`: scan writes and bounded persistence queries.
- `aggregator/`: folder size, child count, and descendant count aggregation.
- `commands/`: Tauri command handlers exposed to the frontend.
- `events/`: throttled scan lifecycle events.
- `models/`: backend models, DTOs, statuses, and structured errors.

Frontend responsibilities live under `src/`:

- `features/scan/`: path selection, scan controls, progress, messages, and
  summary UI.
- `features/tree/`, `features/largest-files/`, `features/largest-folders/`,
  `features/treemap/`, `features/search/`, `features/extensions/`, and
  `features/errors/`: planned result views.
- `shared/api/`: typed Tauri command wrappers and DTO types.
- `shared/components/`: reusable empty, loading, error, size, percentage, and
  breadcrumb components.
- `shared/utils/`: formatting helpers.

SQLite is the primary data layer. Required persisted data includes
`scan_sessions`, `file_entries`, and `scan_errors`, with indexes for session,
parent, size, extension, path, and name. Result views must use limits, offsets,
parent-scoped queries, pagination, virtualization, or capped responses.

## Current Implementation Status

Based on `specs/001-fastdisk-viewer-mvp/tasks.md`, tasks `T001` through `T038`
are complete.

Completed:

- Project setup for Rust dependencies, frontend dependencies, backend modules,
  frontend feature directories, and test scaffolding.
- Foundational models, SQLite migrations, repository layer, structured errors,
  throttled scan events, typed command wrappers, formatting utilities, shared UI
  components, and top-level app layout.
- User Story 1: read-only recursive scanner, scan worker orchestration,
  one-active-scan guard, cancellation flag, batched scan writes, folder
  aggregation, scan commands, scan events, frontend scan API, path picker, scan
  button, progress card, summary card, scan screen integration, and friendly
  scan messages.

Not yet marked complete:

- User Story 2 Tree View lazy loading.
- User Story 3 Largest Files, Largest Folders, and Reveal in Explorer.
- User Story 4 Treemap.
- User Story 5 Search and filters.
- User Story 6 Previous scans, scan errors UI, and extension summary UI.
- Polish, manual QA, full build validation, and compliance audit tasks.

## Development Commands

Install dependencies:

```powershell
pnpm install
```

Run the Vite frontend:

```powershell
pnpm dev
```

Run the Tauri desktop app:

```powershell
pnpm tauri dev
```

Build the frontend:

```powershell
pnpm build
```

Build the Tauri app:

```powershell
pnpm tauri build
```

Preview the frontend build:

```powershell
pnpm preview
```

The Tauri configuration runs `pnpm dev` before desktop development and
`pnpm build` before desktop builds. The configured dev URL is
`http://localhost:1420`.

## Running Tests

No `test` script is currently defined in `package.json`.

Frontend test tooling is installed, so tests can be run directly with Vitest:

```powershell
pnpm exec vitest
```

Backend tests are standard Rust tests from `src-tauri/`:

```powershell
cd src-tauri
cargo test
```

The current plan notes that `rustc` and `cargo` were not on PATH in the shell
used to create the plan. Install or expose the Rust toolchain before running
Tauri builds or Rust tests.

## How to Run the App

1. Install Node.js, pnpm, and the Rust toolchain.
2. Install dependencies with `pnpm install`.
3. Start the desktop app with `pnpm tauri dev`.
4. Select a folder or drive path in the app.
5. Start a scan and review the progress and summary when complete.

Core functionality is local and must not require internet access after
dependencies are installed.
