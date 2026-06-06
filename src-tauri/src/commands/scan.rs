use std::path::PathBuf;
use std::sync::Mutex;
use std::time::Instant;

use chrono::Utc;
use rusqlite::Connection;
use serde::Deserialize;
use tauri::State;

use crate::aggregator::aggregate_folder_sizes;
use crate::db::open_database;
use crate::models::errors::{FastDiskError, FastDiskResult, FrontendError};
use crate::models::{ScanSession, ScanStatus};
use crate::repository::scan_writer::persist_scan_output;
use crate::repository::{
    create_scan_session, get_scan_session as load_scan_session, update_scan_session_status,
    update_scan_session_totals,
};
use crate::scanner::scan_path;
use crate::scanner::session::ScanManager;

pub struct AppState {
    pub database_path: PathBuf,
    pub scan_manager: ScanManager,
    pub connection_lock: Mutex<()>,
}

impl AppState {
    pub fn new(database_path: PathBuf) -> Self {
        Self {
            database_path,
            scan_manager: ScanManager::default(),
            connection_lock: Mutex::new(()),
        }
    }

    pub fn open_connection(&self) -> FastDiskResult<Connection> {
        open_database(&self.database_path)
    }
}

#[derive(Debug, Deserialize)]
pub struct StartScanInput {
    pub path: String,
}

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StartScanResponse {
    pub scan_session_id: i64,
}

#[tauri::command]
pub async fn start_scan(input: StartScanInput, state: State<'_, AppState>) -> Result<StartScanResponse, FrontendError> {
    start_scan_inner(input, &state).map_err(|error| error.to_frontend_error())
}

#[tauri::command]
pub async fn cancel_scan(scan_session_id: i64, state: State<'_, AppState>) -> Result<(), FrontendError> {
    cancel_scan_inner(scan_session_id, &state).map_err(|error| error.to_frontend_error())
}

#[tauri::command]
pub async fn get_scan_session(scan_session_id: i64, state: State<'_, AppState>) -> Result<ScanSession, FrontendError> {
    get_scan_session_inner(scan_session_id, &state).map_err(|error| error.to_frontend_error())
}

pub fn start_scan_inner(input: StartScanInput, state: &AppState) -> FastDiskResult<StartScanResponse> {
    let path = PathBuf::from(input.path);
    if !path.exists() {
        return Err(FastDiskError::InvalidPath);
    }

    let _guard = state
        .connection_lock
        .lock()
        .map_err(|_| FastDiskError::Other("Database lock poisoned.".into()))?;
    let mut connection = state.open_connection()?;
    let started_at = Utc::now().to_rfc3339();
    let scan_session_id = create_scan_session(&connection, &path.to_string_lossy(), &started_at)?;
    let cancel = state.scan_manager.begin_scan(scan_session_id)?;
    let started = Instant::now();

    let output = scan_path(&path, cancel.clone());
    match output {
        Ok(output) => {
            persist_scan_output(&mut connection, scan_session_id, &output)?;
            let total_size = aggregate_folder_sizes(&connection, scan_session_id)?;
            let elapsed_ms = started.elapsed().as_millis() as i64;
            update_scan_session_totals(
                &connection,
                scan_session_id,
                output.total_files,
                output.total_folders,
                total_size.max(output.total_size),
                output.issues.len() as i64,
                elapsed_ms,
            )?;
            let status = if cancel.load(std::sync::atomic::Ordering::Relaxed) {
                ScanStatus::Cancelled
            } else {
                ScanStatus::Completed
            };
            update_scan_session_status(
                &connection,
                scan_session_id,
                status,
                Some(&Utc::now().to_rfc3339()),
                elapsed_ms,
            )?;
        }
        Err(error) => {
            let elapsed_ms = started.elapsed().as_millis() as i64;
            update_scan_session_status(
                &connection,
                scan_session_id,
                ScanStatus::Failed,
                Some(&Utc::now().to_rfc3339()),
                elapsed_ms,
            )?;
            let _ = state.scan_manager.finish_scan(scan_session_id);
            return Err(error);
        }
    }

    let _ = state.scan_manager.finish_scan(scan_session_id)?;
    Ok(StartScanResponse { scan_session_id })
}

pub fn cancel_scan_inner(scan_session_id: i64, state: &AppState) -> FastDiskResult<()> {
    state.scan_manager.cancel_scan(scan_session_id)?;
    Ok(())
}

pub fn get_scan_session_inner(scan_session_id: i64, state: &AppState) -> FastDiskResult<ScanSession> {
    let _guard = state
        .connection_lock
        .lock()
        .map_err(|_| FastDiskError::Other("Database lock poisoned.".into()))?;
    let connection = state.open_connection()?;
    load_scan_session(&connection, scan_session_id)
}
