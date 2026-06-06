pub mod queries;
pub mod scan_writer;
#[cfg(test)]
mod tests;

use rusqlite::{params, Connection};

use crate::models::errors::{FastDiskError, FastDiskResult};
use crate::models::{ScanSession, ScanStatus};

pub const DEFAULT_LIMIT: i64 = 100;
pub const MAX_LIMIT: i64 = 1_000;

pub fn bounded_limit(limit: i64) -> FastDiskResult<i64> {
    if limit <= 0 {
        return Err(FastDiskError::Other(
            "Limit must be greater than zero.".into(),
        ));
    }
    Ok(limit.min(MAX_LIMIT))
}

pub fn bounded_offset(offset: i64) -> FastDiskResult<i64> {
    if offset < 0 {
        return Err(FastDiskError::Other(
            "Offset must be zero or greater.".into(),
        ));
    }
    Ok(offset)
}

pub fn create_scan_session(
    connection: &Connection,
    root_path: &str,
    started_at: &str,
) -> FastDiskResult<i64> {
    connection.execute(
        "INSERT INTO scan_sessions (root_path, status, started_at) VALUES (?1, ?2, ?3)",
        params![root_path, status_to_db(ScanStatus::Scanning), started_at],
    )?;
    Ok(connection.last_insert_rowid())
}

pub fn get_scan_session(
    connection: &Connection,
    scan_session_id: i64,
) -> FastDiskResult<ScanSession> {
    connection
        .query_row(
            "SELECT id, root_path, status, started_at, completed_at, total_files,
                    total_folders, total_size, skipped_items, elapsed_ms
             FROM scan_sessions
             WHERE id = ?1",
            params![scan_session_id],
            |row| {
                let status: String = row.get(2)?;
                Ok(ScanSession {
                    id: row.get(0)?,
                    root_path: row.get(1)?,
                    status: ScanStatus::from_db(&status),
                    started_at: row.get(3)?,
                    completed_at: row.get(4)?,
                    total_files: row.get(5)?,
                    total_folders: row.get(6)?,
                    total_size: row.get(7)?,
                    skipped_items: row.get(8)?,
                    elapsed_ms: row.get(9)?,
                })
            },
        )
        .map_err(FastDiskError::from)
}

pub fn update_scan_session_status(
    connection: &Connection,
    scan_session_id: i64,
    status: ScanStatus,
    completed_at: Option<&str>,
    elapsed_ms: i64,
) -> FastDiskResult<()> {
    connection.execute(
        "UPDATE scan_sessions
         SET status = ?1, completed_at = ?2, elapsed_ms = ?3
         WHERE id = ?4",
        params![
            status_to_db(status),
            completed_at,
            elapsed_ms,
            scan_session_id
        ],
    )?;
    Ok(())
}

pub fn update_scan_session_totals(
    connection: &Connection,
    scan_session_id: i64,
    total_files: i64,
    total_folders: i64,
    total_size: i64,
    skipped_items: i64,
    elapsed_ms: i64,
) -> FastDiskResult<()> {
    connection.execute(
        "UPDATE scan_sessions
         SET total_files = ?1, total_folders = ?2, total_size = ?3,
             skipped_items = ?4, elapsed_ms = ?5
         WHERE id = ?6",
        params![
            total_files,
            total_folders,
            total_size,
            skipped_items,
            elapsed_ms,
            scan_session_id
        ],
    )?;
    Ok(())
}

pub fn insert_file_entry(
    connection: &Connection,
    scan_session_id: i64,
    parent_id: Option<i64>,
    name: &str,
    path: &str,
    size: i64,
    is_directory: bool,
) -> FastDiskResult<i64> {
    connection.execute(
        "INSERT INTO file_entries
        (scan_session_id, parent_id, name, path, size, is_directory)
        VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        params![scan_session_id, parent_id, name, path, size, is_directory],
    )?;
    Ok(connection.last_insert_rowid())
}

pub fn list_children_ids(
    connection: &Connection,
    scan_session_id: i64,
    parent_id: Option<i64>,
    limit: i64,
    offset: i64,
) -> FastDiskResult<Vec<i64>> {
    let limit = bounded_limit(limit)?;
    let offset = bounded_offset(offset)?;
    if let Some(parent_id) = parent_id {
        let mut statement = connection.prepare(
            "SELECT id FROM file_entries
             WHERE scan_session_id = ?1 AND parent_id = ?2
             ORDER BY size DESC, name ASC
             LIMIT ?3 OFFSET ?4",
        )?;
        let rows = statement
            .query_map(params![scan_session_id, parent_id, limit, offset], |row| {
                row.get(0)
            })?;
        let mut ids = Vec::new();
        for id in rows {
            ids.push(id?);
        }
        Ok(ids)
    } else {
        let mut statement = connection.prepare(
            "SELECT id FROM file_entries
             WHERE scan_session_id = ?1 AND parent_id IS NULL
             ORDER BY size DESC, name ASC
             LIMIT ?2 OFFSET ?3",
        )?;
        let rows =
            statement.query_map(params![scan_session_id, limit, offset], |row| row.get(0))?;
        let mut ids = Vec::new();
        for id in rows {
            ids.push(id?);
        }
        Ok(ids)
    }
}

fn status_to_db(status: ScanStatus) -> &'static str {
    status.as_str()
}
