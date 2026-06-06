use std::collections::HashMap;
use std::path::PathBuf;

use rusqlite::{params, Connection};

use crate::models::errors::FastDiskResult;
use crate::scanner::{ScanIssueRecord, ScanOutput, ScannedEntry};

pub fn persist_scan_output(
    connection: &mut Connection,
    scan_session_id: i64,
    output: &ScanOutput,
) -> FastDiskResult<()> {
    let transaction = connection.transaction()?;
    let mut path_to_id = HashMap::<PathBuf, i64>::new();
    let mut entries = output.entries.clone();
    entries.sort_by_key(|entry| entry.depth);

    for entry in &entries {
        let parent_id = entry
            .parent_path
            .as_ref()
            .and_then(|parent_path| path_to_id.get(parent_path))
            .copied();
        let id = insert_entry(&transaction, scan_session_id, parent_id, entry)?;
        path_to_id.insert(entry.path.clone(), id);
    }

    for issue in &output.issues {
        insert_scan_issue(&transaction, scan_session_id, issue)?;
    }

    transaction.commit()?;
    Ok(())
}

fn insert_entry(
    connection: &Connection,
    scan_session_id: i64,
    parent_id: Option<i64>,
    entry: &ScannedEntry,
) -> FastDiskResult<i64> {
    connection.execute(
        "INSERT INTO file_entries
        (scan_session_id, parent_id, name, path, size, is_directory, extension,
         depth, modified_at, created_at, is_symlink)
        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
        params![
            scan_session_id,
            parent_id,
            &entry.name,
            entry.path.to_string_lossy(),
            entry.size,
            entry.is_directory,
            entry.extension.as_deref(),
            entry.depth,
            entry.modified_at.as_deref(),
            entry.created_at.as_deref(),
            entry.is_symlink,
        ],
    )?;
    Ok(connection.last_insert_rowid())
}

fn insert_scan_issue(
    connection: &Connection,
    scan_session_id: i64,
    issue: &ScanIssueRecord,
) -> FastDiskResult<i64> {
    connection.execute(
        "INSERT INTO scan_errors
        (scan_session_id, path, error_kind, error_message, created_at)
        VALUES (?1, ?2, ?3, ?4, ?5)",
        params![
            scan_session_id,
            issue.path.to_string_lossy(),
            &issue.error_kind,
            issue.error_message.as_deref(),
            &issue.created_at,
        ],
    )?;
    Ok(connection.last_insert_rowid())
}
