//! Personal dictionary storage module
//!
//! Manages CRUD operations for personal dictionary entries.

use std::collections::HashMap;
use crate::storage::database::Database;
use crate::storage::models::PersonalDictionaryEntry;
use crate::error::{Result, StorageError};

/// Personal dictionary storage service
pub struct DictionaryStorage {
    db: Database,
}

impl DictionaryStorage {
    /// Create new dictionary storage
    pub fn new(db: Database) -> Self {
        Self { db }
    }

    /// Add a dictionary entry
    pub async fn add_entry(
        &self,
        entry: PersonalDictionaryEntry,
    ) -> Result<String> {
        let device_id = &entry.device_id;

        // Check if device profile exists
        let profile = Self::get_or_create_profile(device_id).await?;

        // Insert dictionary entry
        let entry_id = entry.entry_id.clone();
        sqlx::query!(
            "INSERT INTO personal_dictionary (entry_id, device_id, phrase, replacement, case_sensitive, whole_word_only, category, created_at, last_used_at, usage_count) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, 0)"
        )
        .bind(&entry_id)
        .bind(&entry.device_id)
        .bind(&entry.phrase)
        .bind(&entry.replacement)
        .bind(&entry.case_sensitive)
        .bind(&entry.whole_word_only)
        .bind(&entry.category.as_str())
        .bind(&entry.created_at)
        .bind(&entry.last_used_at)
        .execute(&self.db)
        .await
        .map_err(|e| StorageError::DatabaseError(e.to_string()))?;

        Ok(entry_id)
    }

    /// Get all dictionary entries for a device
    pub async fn get_all_entries(&self, device_id: &str) -> Result<Vec<PersonalDictionaryEntry>> {
        sqlx::query_as!(
            "SELECT entry_id, phrase, replacement, case_sensitive, whole_word_only, category, created_at, last_used_at, usage_count FROM personal_dictionary WHERE device_id = ? ORDER BY created_at DESC"
        )
        .bind(device_id)
        .fetch_all(&self.db)
        .await
        .map_err(|e| StorageError::DatabaseError(e.to_string()))?
    }

    /// Get a dictionary entry by ID
    pub async fn get_entry_by_id(&self, entry_id: &str, device_id: &str) -> Result<Option<PersonalDictionaryEntry>> {
        sqlx::query_as!(
            "SELECT entry_id, phrase, replacement, case_sensitive, whole_word_only, category, created_at, last_used_at, usage_count FROM personal_dictionary WHERE entry_id = ? AND device_id = ?"
        )
        .bind(entry_id)
        .bind(device_id)
        .fetch_optional(&self.db)
        .await
        .map_err(|e| StorageError::DatabaseError(e.to_string()))?
    }

    /// Update a dictionary entry
    pub async fn update_entry(
        &self,
        entry_id: &str,
        device_id: &str,
        phrase: Option<&str>,
        replacement: Option<&str>,
        case_sensitive: Option<bool>,
        category: Option<String>,
        last_used_at: Option<chrono::DateTime<Utc>>,
    ) -> Result<()> {
        let updates = vec![];

        if let Some(phrase) = phrase {
            updates.push(("phrase = ?", phrase));
        }
        if let Some(replacement) = replacement {
            updates.push(("replacement = ?", replacement));
        }
        if let Some(case_sensitive) = case_sensitive {
            updates.push(("case_sensitive = ?", *case_sensitive as i32));
        }
        if let Some(category) = category {
            updates.push(("category = ?", category));
        }
        if let Some(last_used) = last_used {
            updates.push(("last_used_at = ?", last_used));
            updates.push(("usage_count = usage_count + 1"));
        }

        if updates.is_empty() {
            return Ok(());
        }

        let set_clause = updates.iter()
            .map(|(k, v)| format!("{k} = {v1}"))
            .collect::<Vec<_>>()
            .join(", ");

        sqlx::query(
            &format!(
                "UPDATE personal_dictionary SET {} WHERE entry_id = ? AND device_id = ?",
                set_clause
            ),
        )
        .bind(entry_id)
        .bind(device_id)
        .execute(&self.db)
        .await
        .map_err(|e| StorageError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    /// Remove a dictionary entry
    pub async fn remove_entry(&self, entry_id: &str, device_id: &str) -> Result<()> {
        sqlx::query(
            "DELETE FROM personal_dictionary WHERE entry_id = ? AND device_id = ?"
        )
        .bind(entry_id)
        .bind(device_id)
        .execute(&self.db)
        .await
        .map_err(|e| StorageError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    /// Get or create device profile
    async fn get_or_create_profile(&self, device_id: &str) -> Result<i32> {
        sqlx::query_as!(
            "SELECT device_id FROM device_profiles WHERE device_id = ?"
        )
        .bind(device_id)
        .fetch_optional(&self.db)
        .await;

        let profile_id = match result {
            Ok(Some(row)) => row.get::<i32>("device_id"),
            Ok(None) => {
                // Create profile
                sqlx::query(
                    "INSERT INTO device_profiles (device_id, created_at, last_active_at, preferred_language, voice_speed_preference, auto_punctuation_enabled, filler_removal_enabled, self_correction_enabled, crash_reporting_enabled) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
                )
                .bind(device_id)
                .execute(&self.db)
                .await
                .map_err(|e| StorageError::DatabaseError(e.to_string()))?;

                self.db.last_insert_rowid().await
                    .map_err(|e| StorageError::DatabaseError(e.to_string()))?
            }
        };

        Ok(profile_id)
    }

    /// Increment usage count for an entry
    pub async fn increment_usage(&self, entry_id: &str, device_id: &str) -> Result<()> {
        sqlx::query(
            "UPDATE personal_dictionary SET usage_count = usage_count + 1, last_used_at = ? WHERE entry_id = ? AND device_id = ?"
        )
        .bind(entry_id)
        .bind(device_id)
        .execute(&self.db)
        .await
        .map_err(|e| StorageError::DatabaseError(e.to_string()))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[tokio::test]
    async fn test_add_entry() {
        let db = Database::in_memory().expect("Failed to create database");
        let storage = DictionaryStorage::new(db);
        let device_id = "test-device";

        // Ensure device profile exists
        let _profile_id = storage.get_or_create_profile(device_id).await.unwrap();

        // Add entry
        let entry_id = storage.add_entry(&PersonalDictionaryEntry {
            entry_id: "entry-1".to_string(),
            device_id: device_id.to_string(),
            phrase: "API",
            replacement: "Application Programming Interface",
            case_sensitive: false,
            category: crate::storage::models::DictionaryEntryCategory::Technical,
            created_at: Utc::now(),
            last_used_at: None,
        }).await.unwrap();

        // Verify entry was added
        let entries = storage.get_all_entries(device_id).await.unwrap();
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].phrase, "API");
    }

    #[tokio::test]
    async fn test_get_all_entries() {
        let db = Database::in_memory().expect("Failed to create database");
        let storage = DictionaryStorage::new(db);
        let device_id = "test-device";

        // Add multiple entries
        storage.add_entry(&PersonalDictionaryEntry {
            entry_id: "entry-1".to_string(),
            device_id: device_id.to_string(),
            phrase: "API",
            replacement: "Application Programming Interface",
            case_sensitive: false,
            category: crate::storage::models::DictionaryEntryCategory::Technical,
            created_at: Utc::now(),
            last_used_at: None,
        }).await.unwrap();

        storage.add_entry(&PersonalDictionaryEntry {
            entry_id: "entry-2".to_string(),
            device_id: device_id.to_string(),
            phrase: "Kubernetes",
            replacement: "K8s",
            case_sensitive: false,
            category: crate::storage::models::DictionaryEntryCategory::Technical,
            created_at: Utc::now(),
            last_used_at: None,
        }).await.unwrap();

        // Get all entries (should be sorted by created_at DESC)
        let entries = storage.get_all_entries(device_id).await.unwrap();
        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0].phrase, "Kubernetes");
        assert_eq!(entries[1].phrase, "API");
    }

    #[tokio::test]
    async fn test_get_entry_by_id() {
        let db = Database::in_memory().expect("Failed to create database");
        let storage = DictionaryStorage::new(db);
        let device_id = "test-device";

        // Add entry
        let entry_id = storage.add_entry(&PersonalDictionaryEntry {
            entry_id: "entry-1".to_string(),
            device_id: device_id.to_string(),
            phrase: "API",
            replacement: "Application Programming Interface",
            case_sensitive: false,
            category: crate::storage::models::DictionaryEntryCategory::Technical,
            created_at: Utc::now(),
            last_used_at: None,
        }).await.unwrap();

        // Get entry
        let entry = storage.get_entry_by_id("entry-1", device_id).await.unwrap();
        assert!(entry.is_some());
        assert_eq!(entry.unwrap().phrase, "API");
    }

    #[tokio::test]
    async fn test_update_entry() {
        let db = Database::in_memory().expect("Failed to create database");
        let storage = DictionaryStorage::new(db);
        let device_id = "test-device";

        // Add entry
        let entry_id = storage.add_entry(&PersonalDictionaryEntry {
            entry_id: "entry-1".to_string(),
            device_id: device_id.to_string(),
            phrase: "API",
            replacement: "Application Programming Interface",
            case_sensitive: false,
            category: crate::storage::models::DictionaryEntryCategory::Technical,
            created_at: Utc::now(),
            last_used_at: None,
        }).await.unwrap();

        // Update entry
        storage.update_entry("entry-1", device_id, Some("REST API"), None, Some("true"), Some(crate::storage::models::DictionaryEntryCategory::Business), None, None)
            .await.unwrap();

        // Verify update
        let entry = storage.get_entry_by_id("entry-1", device_id).await.unwrap();
        assert!(entry.is_some());
        assert_eq!(entry.unwrap().phrase, "REST API");
        assert_eq!(entry.unwrap().replacement, "REST API");
        assert_eq!(entry.unwrap().category, crate::storage::models::DictionaryEntryCategory::Business);
    }

    #[tokio::test]
    async fn test_remove_entry() {
        let db = Database::in_memory().expect("Failed to create database");
        let storage = DictionaryStorage::new(db);
        let device_id = "test-device";

        // Add entry
        let entry_id = storage.add_entry(&PersonalDictionaryEntry {
            entry_id: "entry-1".to_string(),
            device_id: device_id.to_string(),
            phrase: "API",
            replacement: "Application Programming Interface",
            case_sensitive: false,
            category: crate::storage::models::DictionaryEntryCategory::Technical,
            created_at: Utc::now(),
            last_used_at: None,
        }).await.unwrap();

        // Remove entry
        storage.remove_entry("entry-1", device_id).await.unwrap();

        // Verify removal
        let entries = storage.get_all_entries(device_id).await.unwrap();
        assert_eq!(entries.len(), 0);
    }

    #[tokio::test]
    async fn test_increment_usage() {
        let db = Database::in_memory().expect("Failed to create database");
        let storage = DictionaryStorage::new(db);
        let device_id = "test-device";

        // Add entry
        let entry_id = storage.add_entry(&PersonalDictionaryEntry {
            entry_id: "entry-1".to_string(),
            device_id: device_id.to_string(),
            phrase: "API",
            replacement: "Application Programming Interface",
            case_sensitive: false,
            category: crate::storage::models::DictionaryEntryCategory::Technical,
            created_at: Utc::now(),
            last_used_at: None,
        }).await.unwrap();

        // Increment usage
        storage.increment_usage("entry-1", device_id).await.unwrap();

        // Verify increment
        let entry = storage.get_entry_by_id("entry-1", device_id).await.unwrap();
        assert!(entry.is_some());
        assert_eq!(entry.unwrap().usage_count, 1);
    }
}