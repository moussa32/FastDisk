# Implementation Plan: FastDisk Viewer MVP

**Branch**: `001-fastdisk-viewer-mvp` | **Date**: 2026-06-07 | **Spec**: [spec.md](./spec.md)

**Input**: Feature specification from `/specs/001-fastdisk-viewer-mvp/spec.md`

## Summary

Build the read-only FastDisk Viewer MVP as a Windows desktop disk usage analyzer.
The app scans one selected folder or drive at a time, persists scan sessions and
entries locally, aggregates folder sizes, and exposes bounded views for Tree
View, Largest Files, Largest Folders, Treemap, Search, Extension Summary, Scan
Errors, previous scans, cancellation, and Reveal in Explorer.

The implementation uses the existing Tauri desktop shell with Rust owning
filesystem scanning, persistence, aggregation, and query behavior. React remains
the presentation layer and requests scoped data through typed command wrappers.

## Technical Context

**Language/Version**: TypeScript 5.8.3, React 19.1.0, Tauri 2, Rust 2021 edition
per `src-tauri/Cargo.toml`; local `rustc`/`cargo` are not currently on PATH in
this shell.

**Primary Dependencies**: Tauri 2, React, `@tauri-apps/api`, `@tauri-apps/plugin-opener`,
SQLite Rust crate to be added, async/blocking worker support in Rust, table and
virtualization libraries to be added for large result views, treemap library to
be added for visualization.

**Storage**: Local SQLite database containing `scan_sessions`, `file_entries`,
and `scan_errors`, with indexes for session, parent, size, extension, path, and
name. WAL mode is preferred.

**Testing**: Rust unit/integration tests for scanner, aggregation, repository,
and command query behavior; TypeScript component tests for scan UI, tree/table
states, search debounce, empty/error states, and treemap rendering; manual
quickstart validation for the full desktop workflow.

**Target Platform**: Windows desktop MVP.

**Project Type**: Tauri desktop application with Rust backend and React
frontend.

**Performance Goals**: UI remains responsive during scans; progress updates
visible at least every 1 second while active; scan progress emission throttled
to roughly 250-500 ms; Largest Files/Folders, search, and treemap initial views
return within 2 seconds for completed large scans; scan browsing supports
100,000+ item scans without loading the full tree into React.

**Constraints**: Read-only only; no delete/move/rename/compress/upload/cleanup
behavior; no AI or cloud features; one active scan at a time; no NTFS MFT
scanner in MVP; do not follow link-like folders that can create cycles; all scan
data remains local.

**Scale/Scope**: MVP covers scan selection, scanning, progress, local scan
storage, folder aggregation, summary, previous scans, Tree View, Largest Files,
Largest Folders, Treemap, Search, Filters, Extension Summary, Scan Errors,
Cancel Scan, and Reveal in Explorer.

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

- **Read-only scope**: PASS. MVP exposes only metadata scan, browsing, search,
  filtering, previous-scan loading, cancellation, and Reveal in Explorer.
- **Rust-owned scanning/querying**: PASS. Scanner, persistence, aggregation,
  and production queries are backend responsibilities exposed via Tauri
  commands.
- **SQLite-backed bounded data**: PASS. Scan sessions, entries, and errors are
  persisted locally; tree, table, search, and treemap views use limits, offsets,
  and parent-scoped queries.
- **Frontend memory safety**: PASS. React stores UI state only and never receives
  the full scan tree.
- **Performance**: PASS. Plan requires background scanning, throttled progress,
  batched writes, indexed queries, virtualization, and capped treemap data.
- **Robust scanning**: PASS. Scanner records errors and continues past
  inaccessible paths where possible; link-like paths are not followed in MVP.
- **MVP scope**: PASS. NTFS/MFT scanner, AI, cleanup, deletion, cloud sync,
  duplicate removal, and other out-of-scope items are excluded.
- **Testing**: PASS. Backend tests are required for scanner, persistence,
  aggregation, and query behavior; frontend tests cover meaningful UI states.

## Project Structure

### Documentation (this feature)

```text
specs/001-fastdisk-viewer-mvp/
|-- plan.md
|-- research.md
|-- data-model.md
|-- quickstart.md
|-- contracts/
|   `-- tauri-commands.md
|-- checklists/
|   `-- requirements.md
`-- spec.md
```

### Source Code (repository root)

```text
src/
|-- features/
|   |-- scan/
|   |-- tree/
|   |-- largest-files/
|   |-- largest-folders/
|   |-- treemap/
|   |-- search/
|   |-- extensions/
|   `-- errors/
|-- shared/
|   |-- api/
|   |-- components/
|   `-- utils/
|-- App.tsx
`-- main.tsx

src-tauri/
|-- Cargo.toml
|-- src/
|   |-- aggregator/
|   |-- commands/
|   |-- db/
|   |-- events/
|   |-- models/
|   |-- repository/
|   |-- scanner/
|   |-- lib.rs
|   `-- main.rs
`-- capabilities/

tests/
|-- integration/
`-- unit/
```

**Structure Decision**: Use the constitution-defined Tauri structure: Rust
modules under `src-tauri/src/` own scanning, persistence, aggregation, events,
commands, and DTO models; React feature areas under `src/features/` own UI
composition and query state. Shared typed Tauri wrappers live in
`src/shared/api/`.

## Complexity Tracking

No constitution violations.
