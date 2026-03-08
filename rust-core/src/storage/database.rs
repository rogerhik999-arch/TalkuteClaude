//! Database connection module

use crate::error::{Result, StorageError};
use rusqlite::Connection;
use std::path::Path;

/// Database connection wrapper
pub struct Database {
    conn: Connection,
}

impl Database {
    /// Open or create a new database at the specified path
    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        let conn = Connection::open(path.as_ref())
            .map_err(|_| StorageError::ConnectionFailed)?;

        let db = Self { conn };
        db.init()?;
        Ok(db)
    }

    /// Open or create an in-memory database (for testing)
    pub fn in_memory() -> Result<Self> {
        let conn = Connection::open_in_memory()
            .map_err(|_| StorageError::ConnectionFailed)?;

        let db = Self { conn };
        db.init()?;
        Ok(db)
    }

    /// Initialize the database with the schema
    fn init(&self) -> Result<()> {
        // Get the schema from the embedded SQL
        let schema = include_str!("../../migrations/001_initial_schema.sql");
        self.conn
            .execute_batch(schema)
            .map_err(|e| StorageError::QueryFailed(e.to_string()))?;

        Ok(())
    }

    /// Get a reference to the underlying connection
    pub fn connection(&self) -> &Connection {
        &self.conn
    }

    /// Get a mutable reference to the underlying connection
    pub fn connection_mut(&mut self) -> &mut Connection {
        &mut self.conn
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_in_memory_database() {
        let db = Database::in_memory();
        assert!(db.is_ok());
    }
}
