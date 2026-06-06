use rusqlite::Connection;

use crate::models::errors::FastDiskResult;

pub const REQUIRED_TABLES: &[&str] = &["scan_sessions", "file_entries", "scan_errors"];

pub const REQUIRED_INDEXES: &[&str] = &[
    "idx_file_entries_session",
    "idx_file_entries_parent",
    "idx_file_entries_size",
    "idx_file_entries_extension",
    "idx_file_entries_path",
    "idx_file_entries_name",
];

pub fn run_migrations(connection: &Connection) -> FastDiskResult<()> {
    connection.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS scan_sessions (
          id INTEGER PRIMARY KEY AUTOINCREMENT,
          root_path TEXT NOT NULL,
          status TEXT NOT NULL,
          started_at TEXT NOT NULL,
          completed_at TEXT,
          total_files INTEGER DEFAULT 0,
          total_folders INTEGER DEFAULT 0,
          total_size INTEGER DEFAULT 0,
          skipped_items INTEGER DEFAULT 0,
          elapsed_ms INTEGER DEFAULT 0
        );

        CREATE TABLE IF NOT EXISTS file_entries (
          id INTEGER PRIMARY KEY AUTOINCREMENT,
          scan_session_id INTEGER NOT NULL,
          parent_id INTEGER,
          name TEXT NOT NULL,
          path TEXT NOT NULL,
          size INTEGER NOT NULL DEFAULT 0,
          is_directory INTEGER NOT NULL,
          extension TEXT,
          depth INTEGER NOT NULL DEFAULT 0,
          modified_at TEXT,
          created_at TEXT,
          is_symlink INTEGER DEFAULT 0,
          child_count INTEGER DEFAULT 0,
          descendant_count INTEGER DEFAULT 0,
          FOREIGN KEY(scan_session_id) REFERENCES scan_sessions(id),
          FOREIGN KEY(parent_id) REFERENCES file_entries(id)
        );

        CREATE TABLE IF NOT EXISTS scan_errors (
          id INTEGER PRIMARY KEY AUTOINCREMENT,
          scan_session_id INTEGER NOT NULL,
          path TEXT NOT NULL,
          error_kind TEXT NOT NULL,
          error_message TEXT,
          created_at TEXT NOT NULL,
          FOREIGN KEY(scan_session_id) REFERENCES scan_sessions(id)
        );

        CREATE INDEX IF NOT EXISTS idx_file_entries_session ON file_entries(scan_session_id);
        CREATE INDEX IF NOT EXISTS idx_file_entries_parent ON file_entries(parent_id);
        CREATE INDEX IF NOT EXISTS idx_file_entries_size ON file_entries(size DESC);
        CREATE INDEX IF NOT EXISTS idx_file_entries_extension ON file_entries(extension);
        CREATE INDEX IF NOT EXISTS idx_file_entries_path ON file_entries(path);
        CREATE INDEX IF NOT EXISTS idx_file_entries_name ON file_entries(name);
        ",
    )?;
    Ok(())
}
