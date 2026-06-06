use tempfile::tempdir;

use super::scan::{cancel_scan_inner, get_scan_session_inner, start_scan_inner, AppState, StartScanInput};
use super::tree::{get_children_inner, GetChildrenInput};
use crate::models::ScanStatus;
use crate::repository::{create_scan_session, insert_file_entry};

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

#[test]
fn get_children_returns_direct_children_only_sorted_by_size() {
    let temp = tempdir().unwrap();
    let state = AppState::new(temp.path().join("fastdisk.sqlite"));
    let connection = state.open_connection().unwrap();
    let session_id = create_scan_session(&connection, "C:/scan", "2026-06-07T00:00:00Z").unwrap();
    let root_id = insert_file_entry(&connection, session_id, None, "scan", "C:/scan", 0, true).unwrap();
    let small_id = insert_file_entry(&connection, session_id, Some(root_id), "small.txt", "C:/scan/small.txt", 10, false).unwrap();
    let large_id = insert_file_entry(&connection, session_id, Some(root_id), "large.txt", "C:/scan/large.txt", 20, false).unwrap();
    let _grandchild_id = insert_file_entry(&connection, session_id, Some(small_id), "nested.txt", "C:/scan/nested.txt", 99, false).unwrap();

    let children = get_children_inner(
        GetChildrenInput {
            scan_session_id: session_id,
            parent_id: Some(root_id),
            sort_by: "size".into(),
            sort_direction: "desc".into(),
            limit: 10,
            offset: 0,
        },
        &state,
    )
    .unwrap();

    assert_eq!(children.iter().map(|entry| entry.id).collect::<Vec<_>>(), vec![large_id, small_id]);
}

#[test]
fn get_children_applies_limit_and_offset() {
    let temp = tempdir().unwrap();
    let state = AppState::new(temp.path().join("fastdisk.sqlite"));
    let connection = state.open_connection().unwrap();
    let session_id = create_scan_session(&connection, "C:/scan", "2026-06-07T00:00:00Z").unwrap();
    let root_id = insert_file_entry(&connection, session_id, None, "scan", "C:/scan", 0, true).unwrap();
    let _first_id = insert_file_entry(&connection, session_id, Some(root_id), "a.txt", "C:/scan/a.txt", 30, false).unwrap();
    let second_id = insert_file_entry(&connection, session_id, Some(root_id), "b.txt", "C:/scan/b.txt", 20, false).unwrap();
    let _third_id = insert_file_entry(&connection, session_id, Some(root_id), "c.txt", "C:/scan/c.txt", 10, false).unwrap();

    let children = get_children_inner(
        GetChildrenInput {
            scan_session_id: session_id,
            parent_id: Some(root_id),
            sort_by: "size".into(),
            sort_direction: "desc".into(),
            limit: 1,
            offset: 1,
        },
        &state,
    )
    .unwrap();

    assert_eq!(children.iter().map(|entry| entry.id).collect::<Vec<_>>(), vec![second_id]);
}
