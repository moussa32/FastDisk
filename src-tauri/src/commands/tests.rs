use tempfile::tempdir;

use super::largest::{
    get_largest_files_inner, get_largest_folders_inner, GetLargestFilesInput,
    GetLargestFoldersInput, LargestFiltersInput,
};
use super::reveal::reveal_in_explorer_inner;
use super::scan::{
    cancel_scan_inner, get_scan_session_inner, start_scan_inner, AppState, StartScanInput,
};
use super::tree::{get_children_inner, GetChildrenInput};
use crate::models::ScanStatus;
use crate::repository::{create_scan_session, insert_file_entry};

#[test]
fn start_scan_creates_completed_session() {
    let temp = tempdir().unwrap();
    let scan_root = temp.path().join("scan-root");
    std::fs::create_dir(&scan_root).unwrap();
    std::fs::write(scan_root.join("file.txt"), b"hello").unwrap();
    let database = temp.path().join("fastdisk.sqlite");
    let state = AppState::new(database);

    let response = start_scan_inner(
        StartScanInput {
            path: scan_root.to_string_lossy().to_string(),
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
    let root_id =
        insert_file_entry(&connection, session_id, None, "scan", "C:/scan", 0, true).unwrap();
    let small_id = insert_file_entry(
        &connection,
        session_id,
        Some(root_id),
        "small.txt",
        "C:/scan/small.txt",
        10,
        false,
    )
    .unwrap();
    let large_id = insert_file_entry(
        &connection,
        session_id,
        Some(root_id),
        "large.txt",
        "C:/scan/large.txt",
        20,
        false,
    )
    .unwrap();
    let _grandchild_id = insert_file_entry(
        &connection,
        session_id,
        Some(small_id),
        "nested.txt",
        "C:/scan/nested.txt",
        99,
        false,
    )
    .unwrap();

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

    assert_eq!(
        children.iter().map(|entry| entry.id).collect::<Vec<_>>(),
        vec![large_id, small_id]
    );
}

#[test]
fn get_children_applies_limit_and_offset() {
    let temp = tempdir().unwrap();
    let state = AppState::new(temp.path().join("fastdisk.sqlite"));
    let connection = state.open_connection().unwrap();
    let session_id = create_scan_session(&connection, "C:/scan", "2026-06-07T00:00:00Z").unwrap();
    let root_id =
        insert_file_entry(&connection, session_id, None, "scan", "C:/scan", 0, true).unwrap();
    let _first_id = insert_file_entry(
        &connection,
        session_id,
        Some(root_id),
        "a.txt",
        "C:/scan/a.txt",
        30,
        false,
    )
    .unwrap();
    let second_id = insert_file_entry(
        &connection,
        session_id,
        Some(root_id),
        "b.txt",
        "C:/scan/b.txt",
        20,
        false,
    )
    .unwrap();
    let _third_id = insert_file_entry(
        &connection,
        session_id,
        Some(root_id),
        "c.txt",
        "C:/scan/c.txt",
        10,
        false,
    )
    .unwrap();

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

    assert_eq!(
        children.iter().map(|entry| entry.id).collect::<Vec<_>>(),
        vec![second_id]
    );
}

#[test]
fn get_largest_files_returns_files_only_sorted_by_size() {
    let temp = tempdir().unwrap();
    let state = AppState::new(temp.path().join("fastdisk.sqlite"));
    let connection = state.open_connection().unwrap();
    let session_id = create_scan_session(&connection, "C:/scan", "2026-06-07T00:00:00Z").unwrap();
    insert_entry_with_details(
        &connection,
        session_id,
        None,
        "folder",
        "C:/scan/folder",
        999,
        true,
        None,
    );
    let small_id = insert_entry_with_details(
        &connection,
        session_id,
        None,
        "small.txt",
        "C:/scan/small.txt",
        10,
        false,
        Some(".txt"),
    );
    let large_id = insert_entry_with_details(
        &connection,
        session_id,
        None,
        "large.iso",
        "C:/scan/large.iso",
        200,
        false,
        Some(".iso"),
    );

    let response = get_largest_files_inner(
        GetLargestFilesInput {
            scan_session_id: session_id,
            limit: 100,
            offset: 0,
            sort_by: None,
            sort_direction: None,
            filters: None,
        },
        &state,
    )
    .unwrap();

    assert_eq!(
        response
            .items
            .iter()
            .map(|entry| entry.id)
            .collect::<Vec<_>>(),
        vec![large_id, small_id]
    );
    assert!(response.items.iter().all(|entry| !entry.is_directory));
}

#[test]
fn get_largest_files_applies_filters_sort_and_allowed_limits() {
    let temp = tempdir().unwrap();
    let state = AppState::new(temp.path().join("fastdisk.sqlite"));
    let connection = state.open_connection().unwrap();
    let session_id = create_scan_session(&connection, "C:/scan", "2026-06-07T00:00:00Z").unwrap();
    let beta_id = insert_entry_with_details(
        &connection,
        session_id,
        None,
        "beta.log",
        "C:/scan/beta.log",
        200,
        false,
        Some(".log"),
    );
    let alpha_id = insert_entry_with_details(
        &connection,
        session_id,
        None,
        "alpha.log",
        "C:/scan/alpha.log",
        100,
        false,
        Some(".log"),
    );
    insert_entry_with_details(
        &connection,
        session_id,
        None,
        "image.iso",
        "C:/scan/image.iso",
        500,
        false,
        Some(".iso"),
    );

    let response = get_largest_files_inner(
        GetLargestFilesInput {
            scan_session_id: session_id,
            limit: 100,
            offset: 0,
            sort_by: Some("name".into()),
            sort_direction: Some("asc".into()),
            filters: Some(LargestFiltersInput {
                extension: Some("log".into()),
                extension_group: None,
                min_size: Some(50),
                max_size: Some(250),
            }),
        },
        &state,
    )
    .unwrap();

    assert_eq!(
        response
            .items
            .iter()
            .map(|entry| entry.id)
            .collect::<Vec<_>>(),
        vec![alpha_id, beta_id]
    );

    let invalid_limit = get_largest_files_inner(
        GetLargestFilesInput {
            scan_session_id: session_id,
            limit: 200,
            offset: 0,
            sort_by: None,
            sort_direction: None,
            filters: None,
        },
        &state,
    );
    assert!(invalid_limit.is_err());
}

#[test]
fn get_largest_folders_returns_folders_only_with_offset() {
    let temp = tempdir().unwrap();
    let state = AppState::new(temp.path().join("fastdisk.sqlite"));
    let connection = state.open_connection().unwrap();
    let session_id = create_scan_session(&connection, "C:/scan", "2026-06-07T00:00:00Z").unwrap();
    let large_id = insert_entry_with_details(
        &connection,
        session_id,
        None,
        "large",
        "C:/scan/large",
        900,
        true,
        None,
    );
    let medium_id = insert_entry_with_details(
        &connection,
        session_id,
        None,
        "medium",
        "C:/scan/medium",
        600,
        true,
        None,
    );
    insert_entry_with_details(
        &connection,
        session_id,
        None,
        "file.bin",
        "C:/scan/file.bin",
        1_000,
        false,
        Some(".bin"),
    );

    let response = get_largest_folders_inner(
        GetLargestFoldersInput {
            scan_session_id: session_id,
            limit: 1,
            offset: 1,
            sort_by: Some("size".into()),
            sort_direction: Some("desc".into()),
        },
        &state,
    )
    .unwrap();

    assert_eq!(
        response
            .items
            .iter()
            .map(|entry| entry.id)
            .collect::<Vec<_>>(),
        vec![medium_id]
    );
    assert_ne!(response.items[0].id, large_id);
    assert!(response.items.iter().all(|entry| entry.is_directory));
}

#[test]
fn reveal_in_explorer_rejects_missing_path_and_accepts_existing_path() {
    let temp = tempdir().unwrap();
    let existing = temp.path().join("present.txt");
    std::fs::write(&existing, b"hello").unwrap();

    assert!(reveal_in_explorer_inner(&existing.to_string_lossy()).is_ok());
    assert!(reveal_in_explorer_inner(&temp.path().join("missing.txt").to_string_lossy()).is_err());
}

fn insert_entry_with_details(
    connection: &rusqlite::Connection,
    scan_session_id: i64,
    parent_id: Option<i64>,
    name: &str,
    path: &str,
    size: i64,
    is_directory: bool,
    extension: Option<&str>,
) -> i64 {
    connection
        .execute(
            "INSERT INTO file_entries
             (scan_session_id, parent_id, name, path, size, is_directory, extension, child_count, descendant_count)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            rusqlite::params![
                scan_session_id,
                parent_id,
                name,
                path,
                size,
                is_directory,
                extension,
                if is_directory { 2 } else { 0 },
                if is_directory { 4 } else { 0 }
            ],
        )
        .unwrap();
    connection.last_insert_rowid()
}
