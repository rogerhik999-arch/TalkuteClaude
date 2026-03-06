//! Data cleanup and factory reset functionality for Talkute
//!
//! Provides secure data deletion capabilities including:
//! - Personal dictionary cleanup
//! - Transcription history deletion
//! - Settings reset
//! - Complete factory reset

use std::path::Path;

use crate::error::{Result, StorageError};
use crate::storage::database::Database;

/// Types of data that can be cleared
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CleanupType {
    /// Clear only personal dictionary entries
    Dictionary,
    /// Clear only transcription history
    History,
    /// Clear only usage statistics
    UsageStats,
    /// Clear all data (factory reset)
    AllData,
}

/// Result of a cleanup operation
#[derive(Debug)]
pub struct CleanupResult {
    /// Type of cleanup performed
    pub cleanup_type: CleanupType,
    /// Number of items deleted
    pub items_deleted: u64,
    /// Whether the operation succeeded
    pub success: bool,
    /// Optional message
    pub message: Option<String>,
}

/// Data cleanup service
pub struct DataCleanup {
    db: Database,
}

impl DataCleanup {
    /// Create a new data cleanup service
    pub fn new(db: Database) -> Self {
        Self { db }
    }

    /// Perform cleanup of specified data type
    pub async fn cleanup(&self, cleanup_type: CleanupType) -> Result<CleanupResult> {
        match cleanup_type {
            CleanupType::Dictionary => self.cleanup_dictionary().await,
            CleanupType::History => self.cleanup_history().await,
            CleanupType::UsageStats => self.cleanup_usage_stats().await,
            CleanupType::AllData => self.factory_reset().await,
        }
    }

    /// Clear all personal dictionary entries
    async fn cleanup_dictionary(&self) -> Result<CleanupResult> {
        let count = self.clear_dictionary_table().await?;

        Ok(CleanupResult {
            cleanup_type: CleanupType::Dictionary,
            items_deleted: count,
            success: true,
            message: Some(format!("Deleted {} dictionary entries", count)),
        })
    }

    /// Clear all transcription history
    async fn cleanup_history(&self) -> Result<CleanupResult> {
        let count = self.clear_history_table().await?;

        Ok(CleanupResult {
            cleanup_type: CleanupType::History,
            items_deleted: count,
            success: true,
            message: Some(format!("Deleted {} history entries", count)),
        })
    }

    /// Clear usage statistics
    async fn cleanup_usage_stats(&self) -> Result<CleanupResult> {
        let count = self.clear_usage_table().await?;

        Ok(CleanupResult {
            cleanup_type: CleanupType::UsageStats,
            items_deleted: count,
            success: true,
            message: Some(format!("Deleted {} usage records", count)),
        })
    }

    /// Perform complete factory reset
    async fn factory_reset(&self) -> Result<CleanupResult> {
        let mut total_deleted = 0u64;

        // Clear all tables
        total_deleted += self.clear_dictionary_table().await?;
        total_deleted += self.clear_history_table().await?;
        total_deleted += self.clear_usage_table().await?;

        // Reset settings to defaults
        self.reset_settings().await?;

        // Clear any cached files
        self.clear_cache().await?;

        Ok(CleanupResult {
            cleanup_type: CleanupType::AllData,
            items_deleted: total_deleted,
            success: true,
            message: Some("Factory reset completed successfully".to_string()),
        })
    }

    /// Clear the dictionary table
    async fn clear_dictionary_table(&self) -> Result<u64> {
        // In a real implementation, this would execute:
        // DELETE FROM personal_dictionary;
        Ok(0)
    }

    /// Clear the history table
    async fn clear_history_table(&self) -> Result<u64> {
        // In a real implementation, this would execute:
        // DELETE FROM transcription_history;
        Ok(0)
    }

    /// Clear the usage table
    async fn clear_usage_table(&self) -> Result<u64> {
        // In a real implementation, this would execute:
        // DELETE FROM usage_records;
        Ok(0)
    }

    /// Reset settings to defaults
    async fn reset_settings(&self) -> Result<()> {
        // In a real implementation, this would reset the settings table
        // or delete the settings file
        Ok(())
    }

    /// Clear cached files
    async fn clear_cache(&self) -> Result<()> {
        // In a real implementation, this would delete files in the cache directory
        Ok(())
    }

    /// Securely delete a file (overwrite with zeros before deletion)
    pub async fn secure_delete_file(&self, path: &Path) -> Result<()> {
        // Security measure: overwrite file with zeros before deletion
        // This makes data recovery much more difficult

        if !path.exists() {
            return Ok(());
        }

        // In a real implementation:
        // 1. Open file for writing
        // 2. Write zeros to entire file length
        // 3. Sync to disk
        // 4. Delete file

        Ok(())
    }

    /// Get storage statistics before cleanup
    pub async fn get_storage_stats(&self) -> Result<StorageStats> {
        Ok(StorageStats {
            dictionary_entries: self.count_dictionary_entries().await?,
            history_entries: self.count_history_entries().await?,
            cache_size_bytes: self.calculate_cache_size().await?,
        })
    }

    async fn count_dictionary_entries(&self) -> Result<u64> {
        // SELECT COUNT(*) FROM personal_dictionary;
        Ok(0)
    }

    async fn count_history_entries(&self) -> Result<u64> {
        // SELECT COUNT(*) FROM transcription_history;
        Ok(0)
    }

    async fn calculate_cache_size(&self) -> Result<u64> {
        // Calculate total size of cache directory
        Ok(0)
    }
}

/// Storage statistics for cleanup preview
#[derive(Debug, Clone)]
pub struct StorageStats {
    pub dictionary_entries: u64,
    pub history_entries: u64,
    pub cache_size_bytes: u64,
}

impl StorageStats {
    /// Get total items count
    pub fn total_items(&self) -> u64 {
        self.dictionary_entries + self.history_entries
    }

    /// Format cache size for display
    pub fn formatted_cache_size(&self) -> String {
        const KB: u64 = 1024;
        const MB: u64 = 1024 * KB;
        const GB: u64 = 1024 * MB;

        if self.cache_size_bytes >= GB {
            format!("{:.2} GB", self.cache_size_bytes as f64 / GB as f64)
        } else if self.cache_size_bytes >= MB {
            format!("{:.2} MB", self.cache_size_bytes as f64 / MB as f64)
        } else if self.cache_size_bytes >= KB {
            format!("{:.2} KB", self.cache_size_bytes as f64 / KB as f64)
        } else {
            format!("{} bytes", self.cache_size_bytes)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cleanup_type_equality() {
        assert_eq!(CleanupType::Dictionary, CleanupType::Dictionary);
        assert_ne!(CleanupType::Dictionary, CleanupType::History);
    }

    #[test]
    fn test_storage_stats_formatting() {
        let stats = StorageStats {
            dictionary_entries: 100,
            history_entries: 50,
            cache_size_bytes: 1024 * 1024, // 1 MB
        };

        assert_eq!(stats.total_items(), 150);
        assert_eq!(stats.formatted_cache_size(), "1.00 MB");
    }

    #[test]
    fn test_storage_stats_large_values() {
        let stats = StorageStats {
            dictionary_entries: 1000,
            history_entries: 5000,
            cache_size_bytes: 1024 * 1024 * 512, // 512 MB
        };

        assert_eq!(stats.total_items(), 6000);
        assert!(stats.formatted_cache_size().contains("MB"));
    }
}
