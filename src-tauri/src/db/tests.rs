use super::{migrations, open_in_memory_database};

fn object_exists(connection: &rusqlite::Connection, name: &str, kind: &str) -> bool {
    connection
        .query_row(
            "SELECT EXISTS(SELECT 1 FROM sqlite_master WHERE name = ?1 AND type = ?2)",
            (name, kind),
            |row| row.get::<_, bool>(0),
        )
        .expect("sqlite_master query should run")
}

#[test]
fn migrations_create_required_tables() {
    let connection = open_in_memory_database().expect("in-memory database should open");

    for table in migrations::REQUIRED_TABLES {
        assert!(
            object_exists(&connection, table, "table"),
            "{table} missing"
        );
    }
}

#[test]
fn migrations_create_required_indexes() {
    let connection = open_in_memory_database().expect("in-memory database should open");

    for index in migrations::REQUIRED_INDEXES {
        assert!(
            object_exists(&connection, index, "index"),
            "{index} missing"
        );
    }
}
