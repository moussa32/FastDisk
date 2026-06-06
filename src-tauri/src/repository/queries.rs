use rusqlite::{params, Connection, Row};

use crate::models::errors::FastDiskResult;
use crate::models::FileEntry;
use crate::repository::{bounded_limit, bounded_offset};

#[derive(Debug, Clone, Copy)]
pub enum ChildSortBy {
    Size,
    Name,
    ModifiedAt,
    Type,
}

impl ChildSortBy {
    pub fn from_input(value: &str) -> Self {
        match value {
            "name" => Self::Name,
            "modified_at" => Self::ModifiedAt,
            "type" => Self::Type,
            _ => Self::Size,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum SortDirection {
    Asc,
    Desc,
}

impl SortDirection {
    pub fn from_input(value: &str) -> Self {
        match value {
            "asc" => Self::Asc,
            _ => Self::Desc,
        }
    }
}

pub fn get_children(
    connection: &Connection,
    scan_session_id: i64,
    parent_id: Option<i64>,
    sort_by: ChildSortBy,
    sort_direction: SortDirection,
    limit: i64,
    offset: i64,
) -> FastDiskResult<Vec<FileEntry>> {
    let limit = bounded_limit(limit)?;
    let offset = bounded_offset(offset)?;
    let order_by = child_order_by(sort_by, sort_direction);

    if let Some(parent_id) = parent_id {
        let sql = format!(
            "SELECT id, scan_session_id, parent_id, name, path, size, is_directory,
                    extension, depth, modified_at, created_at, is_symlink,
                    child_count, descendant_count
             FROM file_entries
             WHERE scan_session_id = ?1 AND parent_id = ?2
             ORDER BY {order_by}
             LIMIT ?3 OFFSET ?4"
        );
        let mut statement = connection.prepare(&sql)?;
        let rows = statement.query_map(params![scan_session_id, parent_id, limit, offset], map_file_entry)?;
        collect_rows(rows)
    } else {
        let sql = format!(
            "SELECT id, scan_session_id, parent_id, name, path, size, is_directory,
                    extension, depth, modified_at, created_at, is_symlink,
                    child_count, descendant_count
             FROM file_entries
             WHERE scan_session_id = ?1 AND parent_id IS NULL
             ORDER BY {order_by}
             LIMIT ?2 OFFSET ?3"
        );
        let mut statement = connection.prepare(&sql)?;
        let rows = statement.query_map(params![scan_session_id, limit, offset], map_file_entry)?;
        collect_rows(rows)
    }
}

fn collect_rows<F>(rows: rusqlite::MappedRows<'_, F>) -> FastDiskResult<Vec<FileEntry>>
where
    F: FnMut(&Row<'_>) -> rusqlite::Result<FileEntry>,
{
    let mut entries = Vec::new();
    for row in rows {
        entries.push(row?);
    }
    Ok(entries)
}

fn child_order_by(sort_by: ChildSortBy, direction: SortDirection) -> &'static str {
    match (sort_by, direction) {
        (ChildSortBy::Name, SortDirection::Asc) => "name ASC, size DESC",
        (ChildSortBy::Name, SortDirection::Desc) => "name DESC, size DESC",
        (ChildSortBy::ModifiedAt, SortDirection::Asc) => "modified_at ASC, size DESC",
        (ChildSortBy::ModifiedAt, SortDirection::Desc) => "modified_at DESC, size DESC",
        (ChildSortBy::Type, SortDirection::Asc) => "is_directory ASC, size DESC, name ASC",
        (ChildSortBy::Type, SortDirection::Desc) => "is_directory DESC, size DESC, name ASC",
        (ChildSortBy::Size, SortDirection::Asc) => "size ASC, name ASC",
        (ChildSortBy::Size, SortDirection::Desc) => "size DESC, name ASC",
    }
}

fn map_file_entry(row: &Row<'_>) -> rusqlite::Result<FileEntry> {
    Ok(FileEntry {
        id: row.get(0)?,
        scan_session_id: row.get(1)?,
        parent_id: row.get(2)?,
        name: row.get(3)?,
        path: row.get(4)?,
        size: row.get(5)?,
        is_directory: row.get::<_, i64>(6)? == 1,
        extension: row.get(7)?,
        depth: row.get(8)?,
        modified_at: row.get(9)?,
        created_at: row.get(10)?,
        is_symlink: row.get::<_, i64>(11)? == 1,
        child_count: row.get(12)?,
        descendant_count: row.get(13)?,
    })
}
