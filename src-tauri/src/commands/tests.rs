use tempfile::tempdir;

use super::scan::{cancel_scan_inner, get_scan_session_inner, start_scan_inner, AppState, StartScanInput};
use crate::models::ScanStatus;

#[test]
fn start_scan_creates_completed_session() {
    let temp = tempdir().unwrap();
    std::fs::write(temp.path().join("file.txt"), b"hello").unwrap();
    let database = temp.path().join("fastdisk.sqlite");
    let state = AppState::new(database);

    let response = start_scan_inner(
        StartScanInput {
            path: temp.path().to_string_lossy().to_string(),
        },
        &state,
    )
    .unwrap();
    let session = get_scan_session_inner(response.scan_session_id, &state).unwrap();

    assert_eq!(session.status, ScanStatus::Completed);
    assert_eq!(session.total_files, 1);
}

#[test]
fn start_scan_rejects_missing_path() {
    let temp = tempdir().unwrap();
    let database = temp.path().join("fastdisk.sqlite");
    let state = AppState::new(database);

    let result = start_scan_inner(
        StartScanInput {
            path: temp.path().join("missing").to_string_lossy().to_string(),
        },
        &state,
    );

    assert!(result.is_err());
}

#[test]
fn cancel_scan_without_active_session_is_ok() {
    let temp = tempdir().unwrap();
    let state = AppState::new(temp.path().join("fastdisk.sqlite"));

    assert!(cancel_scan_inner(42, &state).is_ok());
}
