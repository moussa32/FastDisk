use serde::Deserialize;
use tauri::State;

use crate::commands::scan::AppState;
use crate::models::errors::{FastDiskResult, FrontendError};
use crate::models::FileEntry;
use crate::repository::queries::{get_children as query_children, ChildSortBy, SortDirection};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetChildrenInput {
    pub scan_session_id: i64,
    pub parent_id: Option<i64>,
    pub sort_by: String,
    pub sort_direction: String,
    pub limit: i64,
    pub offset: i64,
}

#[tauri::command]
pub async fn get_children(
    input: GetChildrenInput,
    state: State<'_, AppState>,
) -> Result<Vec<FileEntry>, FrontendError> {
    get_children_inner(input, &state).map_err(|error| error.to_frontend_error())
}

pub fn get_children_inner(
    input: GetChildrenInput,
    state: &AppState,
) -> FastDiskResult<Vec<FileEntry>> {
    let _guard = state.connection_lock.lock().map_err(|_| {
        crate::models::errors::FastDiskError::Other("Database lock poisoned.".into())
    })?;
    let connection = state.open_connection()?;
    query_children(
        &connection,
        input.scan_session_id,
        input.parent_id,
        ChildSortBy::from_input(&input.sort_by),
        SortDirection::from_input(&input.sort_direction),
        input.limit,
        input.offset,
    )
}
