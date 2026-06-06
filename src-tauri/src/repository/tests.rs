use crate::db::open_in_memory_database;

use super::{
    bounded_limit, bounded_offset, create_scan_session, insert_file_entry, list_children_ids,
    MAX_LIMIT,
};

#[test]
fn bounds_limit_to_maximum() {
    assert_eq!(bounded_limit(MAX_LIMIT + 10).unwrap(), MAX_LIMIT);
    assert!(bounded_limit(0).is_err());
}

#[test]
fn rejects_negative_offset() {
    assert_eq!(bounded_offset(0).unwrap(), 0);
    assert!(bounded_offset(-1).is_err());
}

#[test]
fn inserts_and_lists_direct_children_only() {
    let connection = open_in_memory_database().expect("database should open");
    let session_id = create_scan_session(&connection, "C:/tmp", "2026-06-07T00:00:00Z").unwrap();
    let root_id =
        insert_file_entry(&connection, session_id, None, "tmp", "C:/tmp", 30, true).unwrap();
    let child_id = insert_file_entry(
        &connection,
        session_id,
        Some(root_id),
        "a.bin",
        "C:/tmp/a.bin",
        20,
        false,
    )
    .unwrap();
    let _grandchild_id = insert_file_entry(
        &connection,
        session_id,
        Some(child_id),
        "nested.bin",
        "C:/tmp/nested.bin",
        10,
        false,
    )
    .unwrap();

    let root_rows = list_children_ids(&connection, session_id, None, 10, 0).unwrap();
    let child_rows = list_children_ids(&connection, session_id, Some(root_id), 10, 0).unwrap();

    assert_eq!(root_rows, vec![root_id]);
    assert_eq!(child_rows, vec![child_id]);
}
