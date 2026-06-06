use serde::{Deserialize, Serialize};
use tauri::State;

use crate::commands::scan::AppState;
use crate::models::errors::{FastDiskError, FastDiskResult, FrontendError};
use crate::models::FileEntry;
use crate::repository::queries::{
    get_largest_files as query_largest_files, get_largest_folders as query_largest_folders,
    EntryFilters, LargestFileSortBy, LargestFolderSortBy, SortDirection,
};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LargestFiltersInput {
    pub extension: Option<String>,
    pub extension_group: Option<String>,
    pub min_size: Option<i64>,
    pub max_size: Option<i64>,
}

impl From<Option<LargestFiltersInput>> for EntryFilters {
    fn from(value: Option<LargestFiltersInput>) -> Self {
        value
            .map(|filters| EntryFilters {
                extension: filters.extension,
                extension_group: filters.extension_group,
                min_size: filters.min_size,
                max_size: filters.max_size,
            })
            .unwrap_or_default()
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetLargestFilesInput {
    pub scan_session_id: i64,
    pub limit: i64,
    pub offset: i64,
    pub sort_by: Option<String>,
    pub sort_direction: Option<String>,
    pub filters: Option<LargestFiltersInput>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetLargestFoldersInput {
    pub scan_session_id: i64,
    pub limit: i64,
    pub offset: i64,
    pub sort_by: Option<String>,
    pub sort_direction: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EntriesResponse {
    pub items: Vec<FileEntry>,
}

#[tauri::command]
pub async fn get_largest_files(
    input: GetLargestFilesInput,
    state: State<'_, AppState>,
) -> Result<EntriesResponse, FrontendError> {
    get_largest_files_inner(input, &state).map_err(|error| error.to_frontend_error())
}

#[tauri::command]
pub async fn get_largest_folders(
    input: GetLargestFoldersInput,
    state: State<'_, AppState>,
) -> Result<EntriesResponse, FrontendError> {
    get_largest_folders_inner(input, &state).map_err(|error| error.to_frontend_error())
}

pub fn get_largest_files_inner(
    input: GetLargestFilesInput,
    state: &AppState,
) -> FastDiskResult<EntriesResponse> {
    let limit = validate_largest_files_limit(input.limit)?;
    let _guard = state
        .connection_lock
        .lock()
        .map_err(|_| FastDiskError::Other("Database lock poisoned.".into()))?;
    let connection = state.open_connection()?;
    let items = query_largest_files(
        &connection,
        input.scan_session_id,
        LargestFileSortBy::from_input(input.sort_by.as_deref()),
        SortDirection::from_input(input.sort_direction.as_deref().unwrap_or("desc")),
        limit,
        input.offset,
        EntryFilters::from(input.filters),
    )?;
    Ok(EntriesResponse { items })
}

pub fn get_largest_folders_inner(
    input: GetLargestFoldersInput,
    state: &AppState,
) -> FastDiskResult<EntriesResponse> {
    let _guard = state
        .connection_lock
        .lock()
        .map_err(|_| FastDiskError::Other("Database lock poisoned.".into()))?;
    let connection = state.open_connection()?;
    let items = query_largest_folders(
        &connection,
        input.scan_session_id,
        LargestFolderSortBy::from_input(input.sort_by.as_deref()),
        SortDirection::from_input(input.sort_direction.as_deref().unwrap_or("desc")),
        input.limit,
        input.offset,
    )?;
    Ok(EntriesResponse { items })
}

fn validate_largest_files_limit(limit: i64) -> FastDiskResult<i64> {
    match limit {
        100 | 500 | 1000 => Ok(limit),
        _ => Err(FastDiskError::Other(
            "Largest files limit must be 100, 500, or 1000.".into(),
        )),
    }
}
