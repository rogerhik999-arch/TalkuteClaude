//! Migration runner for database schema updates

use crate::error::{Result, StorageError};
use crate::storage::database::Database;
use rusqlite::Connection;
use std::fs;
use std::path::Path;

/// Current schema version
pub const SCHEMA_VERSION: i32 = 1;

/// Migration struct
pub struct MigrationRunner {
    db: Database,
}

impl MigrationRunner {
    /// Create a new migration runner
    pub fn new(db: Database) -> Self {
        Self { db }
    }

    /// Run all pending migrations
    pub fn run(&self) -> Result<()> {
        let current_version = self.get_schema_version()?;

        if current_version >= SCHEMA_VERSION {
            return Ok(());
        }

        // Apply migrations sequentially
        let migrations = self.get_migrations(current_version + 1)?;

        for migration in migrations {
            self.apply_migration(&migration)?;
        }

        // Update schema version
        self.update_schema_version(SCHEMA_VERSION)?;

        Ok(())
    }

    /// Get the current schema version
    pub fn get_schema_version(&self) -> Result<i32> {
        let conn = self.db.connection();

        // Create schema_versions table if it doesn't exist
        conn.execute_batch(SCHEMA_VERSIONS_TABLE)
            .map_err(|e| StorageError::QueryFailed(e.to_string()))?;

        let version: Option<i32> = conn.query_row(
            "SELECT version FROM schema_versions ORDER BY version DESC LIMIT 1",
            [],
            |row| row.get(0),
        ).ok();

        Ok(version.unwrap_or(0))
    }

    /// Get migration files for versions >= start_version
    fn get_migrations(&self, start_version: i32) -> Result<Vec<String>> {
        let migrations_dir = Path::new("migrations");

        if !migrations_dir.exists() {
            return Ok(Vec::new());
        }

        let mut migrations = Vec::new();

        for entry in fs::read_dir(migrations_dir)
            .map_err(|e| StorageError::QueryFailed(e.to_string()))?
        {
            let entry = entry.map_err(|e| StorageError::QueryFailed(e.to_string()))?;
            let path = entry.path();

            if path.is_file() {
                let filename = path.file_name()
                    .ok_or_else(|| StorageError::QueryFailed("Invalid filename".to_string()))?;

                let filename_str = filename.to_string_lossy();
                if filename_str.ends_with(".sql") {
                    // Extract version number from filename (e.g., "001_initial_schema.sql" -> 1)
                    let version_num = filename_str.split('_').next()
                        .and_then(|s| s.parse::<i32>().ok())
                        .unwrap_or(0);

                    if version_num >= start_version {
                        let content = fs::read_to_string(&path)
                            .map_err(|e| StorageError::QueryFailed(e.to_string()))?;
                        migrations.push(content);
                    }
                }
            }
        }

        // Sort migrations by version
        migrations.sort();
        Ok(migrations)
    }

    /// Apply a single migration
    fn apply_migration(&self, sql: &str) -> Result<()> {
        self.db.connection()
            .execute_batch(sql)
            .map_err(|e| StorageError::MigrationFailed(e.to_string()))?;
        Ok(())
    }

    /// Update the schema version in the database
    fn update_schema_version(&self, version: i32) -> Result<()> {
        let conn = self.db.connection();

        conn.execute(
            "INSERT INTO schema_versions (version, applied_at) VALUES (?1, ?2)",
            rusqlite::params![version, chrono::Utc::now().to_rfc3339()],
        )
        .map_err(|e| StorageError::MigrationFailed(e.to_string()))?;

        Ok(())
    }
}

/// SQL for schema_versions table
const SCHEMA_VERSIONS_TABLE: &str = "
CREATE TABLE IF NOT EXISTS schema_versions (
    version INTEGER PRIMARY KEY,
    applied_at TEXT NOT NULL
);
";
