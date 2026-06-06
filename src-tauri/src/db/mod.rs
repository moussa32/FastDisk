pub mod migrations;

#[cfg(test)]
mod tests;

use std::path::Path;

use rusqlite::Connection;

use crate::models::errors::FastDiskResult;

pub fn open_database(path: impl AsRef<Path>) -> FastDiskResult<Connection> {
    let connection = Connection::open(path)?;
    configure_connection(&connection)?;
    migrations::run_migrations(&connection)?;
    Ok(connection)
}

pub fn open_in_memory_database() -> FastDiskResult<Connection> {
    let connection = Connection::open_in_memory()?;
    configure_connection(&connection)?;
    migrations::run_migrations(&connection)?;
    Ok(connection)
}

fn configure_connection(connection: &Connection) -> FastDiskResult<()> {
    connection.pragma_update(None, "journal_mode", "WAL")?;
    connection.pragma_update(None, "synchronous", "NORMAL")?;
    connection.pragma_update(None, "temp_store", "MEMORY")?;
    Ok(())
}
