use rusqlite::types::Value;
use rusqlite::{params, params_from_iter, Connection, Row};

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

#[derive(Debug, Clone, Copy)]
pub enum LargestFileSortBy {
    Size,
    Name,
    Extension,
    ModifiedAt,
}

impl LargestFileSortBy {
    pub fn from_input(value: Option<&str>) -> Self {
        match value {
            Some("name") => Self::Name,
            Some("extension") => Self::Extension,
            Some("modified_at") => Self::ModifiedAt,
            _ => Self::Size,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum LargestFolderSortBy {
    Size,
    Name,
    ModifiedAt,
}

impl LargestFolderSortBy {
    pub fn from_input(value: Option<&str>) -> Self {
        match value {
            Some("name") => Self::Name,
            Some("modified_at") => Self::ModifiedAt,
            _ => Self::Size,
        }
    }
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

#[derive(Debug, Default, Clone)]
pub struct EntryFilters {
    pub extension: Option<String>,
    pub extension_group: Option<String>,
    pub min_size: Option<i64>,
    pub max_size: Option<i64>,
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
        let rows = statement.query_map(
            params![scan_session_id, parent_id, limit, offset],
            map_file_entry,
        )?;
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

pub fn get_largest_files(
    connection: &Connection,
    scan_session_id: i64,
    sort_by: LargestFileSortBy,
    sort_direction: SortDirection,
    limit: i64,
    offset: i64,
    filters: EntryFilters,
) -> FastDiskResult<Vec<FileEntry>> {
    let limit = bounded_limit(limit)?;
    let offset = bounded_offset(offset)?;
    validate_size_filters(filters.min_size, filters.max_size)?;

    let mut where_parts = vec![
        "scan_session_id = ?".to_string(),
        "is_directory = 0".to_string(),
    ];
    let mut values = vec![Value::Integer(scan_session_id)];
    push_file_filters(&mut where_parts, &mut values, filters)?;
    values.push(Value::Integer(limit));
    values.push(Value::Integer(offset));

    let sql = format!(
        "SELECT id, scan_session_id, parent_id, name, path, size, is_directory,
                extension, depth, modified_at, created_at, is_symlink,
                child_count, descendant_count
         FROM file_entries
         WHERE {}
         ORDER BY {}
         LIMIT ? OFFSET ?",
        where_parts.join(" AND "),
        largest_file_order_by(sort_by, sort_direction)
    );
    let mut statement = connection.prepare(&sql)?;
    let rows = statement.query_map(params_from_iter(values), map_file_entry)?;
    collect_rows(rows)
}

pub fn get_largest_folders(
    connection: &Connection,
    scan_session_id: i64,
    sort_by: LargestFolderSortBy,
    sort_direction: SortDirection,
    limit: i64,
    offset: i64,
) -> FastDiskResult<Vec<FileEntry>> {
    let limit = bounded_limit(limit)?;
    let offset = bounded_offset(offset)?;
    let sql = format!(
        "SELECT id, scan_session_id, parent_id, name, path, size, is_directory,
                extension, depth, modified_at, created_at, is_symlink,
                child_count, descendant_count
         FROM file_entries
         WHERE scan_session_id = ?1 AND is_directory = 1
         ORDER BY {}
         LIMIT ?2 OFFSET ?3",
        largest_folder_order_by(sort_by, sort_direction)
    );
    let mut statement = connection.prepare(&sql)?;
    let rows = statement.query_map(params![scan_session_id, limit, offset], map_file_entry)?;
    collect_rows(rows)
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

fn largest_file_order_by(sort_by: LargestFileSortBy, direction: SortDirection) -> &'static str {
    match (sort_by, direction) {
        (LargestFileSortBy::Name, SortDirection::Asc) => "name ASC, size DESC",
        (LargestFileSortBy::Name, SortDirection::Desc) => "name DESC, size DESC",
        (LargestFileSortBy::Extension, SortDirection::Asc) => "extension ASC, size DESC, name ASC",
        (LargestFileSortBy::Extension, SortDirection::Desc) => {
            "extension DESC, size DESC, name ASC"
        }
        (LargestFileSortBy::ModifiedAt, SortDirection::Asc) => "modified_at ASC, size DESC",
        (LargestFileSortBy::ModifiedAt, SortDirection::Desc) => "modified_at DESC, size DESC",
        (LargestFileSortBy::Size, SortDirection::Asc) => "size ASC, name ASC",
        (LargestFileSortBy::Size, SortDirection::Desc) => "size DESC, name ASC",
    }
}

fn largest_folder_order_by(sort_by: LargestFolderSortBy, direction: SortDirection) -> &'static str {
    match (sort_by, direction) {
        (LargestFolderSortBy::Name, SortDirection::Asc) => "name ASC, size DESC",
        (LargestFolderSortBy::Name, SortDirection::Desc) => "name DESC, size DESC",
        (LargestFolderSortBy::ModifiedAt, SortDirection::Asc) => "modified_at ASC, size DESC",
        (LargestFolderSortBy::ModifiedAt, SortDirection::Desc) => "modified_at DESC, size DESC",
        (LargestFolderSortBy::Size, SortDirection::Asc) => "size ASC, name ASC",
        (LargestFolderSortBy::Size, SortDirection::Desc) => "size DESC, name ASC",
    }
}

fn push_file_filters(
    where_parts: &mut Vec<String>,
    values: &mut Vec<Value>,
    filters: EntryFilters,
) -> FastDiskResult<()> {
    if let Some(extension) = normalize_extension(filters.extension) {
        where_parts.push("extension = ?".to_string());
        values.push(Value::Text(extension));
    }

    if let Some(group) = filters.extension_group {
        let extensions = extension_group_values(&group).ok_or_else(|| {
            crate::models::errors::FastDiskError::Other("Unknown extension group.".into())
        })?;
        let placeholders = vec!["?"; extensions.len()].join(", ");
        where_parts.push(format!("extension IN ({placeholders})"));
        values.extend(
            extensions
                .into_iter()
                .map(|extension| Value::Text(extension.to_string())),
        );
    }

    if let Some(min_size) = filters.min_size {
        where_parts.push("size >= ?".to_string());
        values.push(Value::Integer(min_size.max(0)));
    }

    if let Some(max_size) = filters.max_size {
        where_parts.push("size <= ?".to_string());
        values.push(Value::Integer(max_size.max(0)));
    }

    Ok(())
}

fn validate_size_filters(min_size: Option<i64>, max_size: Option<i64>) -> FastDiskResult<()> {
    if let (Some(min_size), Some(max_size)) = (min_size, max_size) {
        if min_size > max_size {
            return Err(crate::models::errors::FastDiskError::Other(
                "Minimum size cannot be greater than maximum size.".into(),
            ));
        }
    }
    Ok(())
}

fn normalize_extension(extension: Option<String>) -> Option<String> {
    extension.and_then(|value| {
        let trimmed = value.trim();
        if trimmed.is_empty() {
            None
        } else if trimmed.starts_with('.') {
            Some(trimmed.to_lowercase())
        } else {
            Some(format!(".{}", trimmed.to_lowercase()))
        }
    })
}

fn extension_group_values(group: &str) -> Option<Vec<&'static str>> {
    match group {
        "videos" => Some(vec![".mp4", ".mkv", ".mov", ".avi", ".wmv"]),
        "archives" => Some(vec![".zip", ".rar", ".7z", ".tar", ".gz"]),
        "disk_images" => Some(vec![".iso", ".img"]),
        "installers" => Some(vec![".exe", ".msi"]),
        "documents" => Some(vec![".pdf", ".docx", ".xlsx", ".pptx"]),
        _ => None,
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
