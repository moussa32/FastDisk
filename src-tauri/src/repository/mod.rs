#[cfg(test)]
mod tests;

use rusqlite::{params, Connection};

use crate::models::errors::{FastDiskError, FastDiskResult};
use crate::models::ScanStatus;

pub const DEFAULT_LIMIT: i64 = 100;
pub const MAX_LIMIT: i64 = 1_000;

pub fn bounded_limit(limit: i64) -> FastDiskResult<i64> {
    if limit <= 0 {
        return Err(FastDiskError::Other("Limit must be greater than zero.".into()));
    }
    Ok(limit.min(MAX_LIMIT))
}

pub fn bounded_offset(offset: i64) -> FastDiskResult<i64> {
    if offset < 0 {
        return Err(FastDiskError::Other("Offset must be zero or greater.".into()));
    }
    Ok(offset)
}

pub fn create_scan_session(connection: &Connection, root_path: &str, started_at: &str) -> FastDiskResult<i64> {
    connection.execute(
        "INSERT INTO scan_sessions (root_path, status, started_at) VALUES (?1, ?2, ?3)",
        params![root_path, status_to_db(ScanStatus::Scanning), started_at],
    )?;
    Ok(connection.last_insert_rowid())
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
    let mut statement = if parent_id.is_some() {
        connection.prepare(
            "SELECT id FROM file_entries
             WHERE scan_session_id = ?1 AND parent_id = ?2
             ORDER BY size DESC, name ASC
             LIMIT ?3 OFFSET ?4",
        )?
    } else {
        connection.prepare(
            "SELECT id FROM file_entries
             WHERE scan_session_id = ?1 AND parent_id IS NULL
             ORDER BY size DESC, name ASC
             LIMIT ?2 OFFSET ?3",
        )?
    };

    let rows = if let Some(parent_id) = parent_id {
        statement.query_map(params![scan_session_id, parent_id, limit, offset], |row| row.get(0))?
    } else {
        statement.query_map(params![scan_session_id, limit, offset], |row| row.get(0))?
    };

    let mut ids = Vec::new();
    for id in rows {
        ids.push(id?);
    }
    Ok(ids)
}

fn status_to_db(status: ScanStatus) -> &'static str {
    match status {
        ScanStatus::Idle => "idle",
        ScanStatus::Scanning => "scanning",
        ScanStatus::Completed => "completed",
        ScanStatus::Failed => "failed",
        ScanStatus::Cancelled => "cancelled",
    }
}
