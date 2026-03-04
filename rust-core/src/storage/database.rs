//! Database connection module

use crate::error::{Result, StorageError};
use rusqlite::{Connection, params};
use std::path::Path;

/// Database connection wrapper
pub struct Database {
    conn: Connection,
}

impl Database {
    /// Open or create a new database at the specified path
    pub fn open(path: &str) -> Result<Self> {
        let conn = Connection::open(path)
            .map_err(|e| StorageError::ConnectionFailed)?;

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

    /// Execute a statement with parameters
    pub fn execute<'a>(&self, sql: &str, params: &'a [(&'a str, &'a dyn ToSql)]) -> Result<u64> {
        self.conn.execute(sql, params)
            .map_err(|e| StorageError::QueryFailed(e.to_string()))
    }
}

/// Trait for values that can be converted to SQL parameters
pub trait ToSql {
    fn to_sql(&self) -> rusqlite::types::Value;
}

impl ToSql for String {
    fn to_sql(&self) -> rusqlite::types::Value {
        rusqlite::types::Value::Text(self.clone())
    }
}

impl ToSql for &str {
    fn to_sql(&self) -> rusqlite::types::Value {
        rusqlite::types::Value::Text(self.to_string())
    }
}

impl ToSql for i32 {
    fn to_sql(&self) -> rusqlite::types::Value {
        rusqlite::types::Value::Integer(*self as i64)
    }
}

impl ToSql for i64 {
    fn to_sql(&self) -> rusqlite::types::Value {
        rusqlite::types::Value::Integer(*self)
    }
}

impl ToSql for f64 {
    fn to_sql(&self) -> rusqlite::types::Value {
        rusqlite::types::Value::Real(*self)
    }
}

impl ToSql for bool {
    fn to_sql(&self) -> rusqlite::types::Value {
        rusqlite::types::Value::Integer(if *self { 1 } else { 0 })
    }
}

impl<T: ToSql> ToSql for Option<T> {
    fn to_sql(&self) -> rusqlite::types::Value {
        match self {
            Some(v) => v.to_sql(),
            None => rusqlite::types::Value::Null,
        }
    }
}
