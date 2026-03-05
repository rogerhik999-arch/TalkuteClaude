//! Personal dictionary storage module
//!
//! Manages CRUD operations for personal dictionary entries.

use crate::storage::database::Database;
use crate::storage::models::{PersonalDictionaryEntry, DictionaryEntryCategory};
use crate::error::{Result, StorageError};
use rusqlite::params;
use chrono::{DateTime, Utc};

/// Personal dictionary storage service
pub struct DictionaryStorage<'a> {
    db: &'a Database,
}

impl<'a> DictionaryStorage<'a> {
    /// Create new dictionary storage
    pub fn new(db: &'a Database) -> Self {
        Self { db }
    }

    /// Add a dictionary entry
    pub fn add_entry(&self, entry: &PersonalDictionaryEntry) -> Result<String> {
        let entry_id = entry.entry_id.clone();

        self.db.connection().execute(
            "INSERT OR REPLACE INTO personal_dictionary
             (entry_id, device_id, phrase, replacement, case_sensitive, whole_word_only, category, created_at, last_used_at, usage_count)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
            params![
                entry.entry_id,
                entry.device_id,
                entry.phrase,
                entry.replacement,
                entry.case_sensitive as i32,
                entry.whole_word_only as i32,
                entry.category.as_str(),
                entry.created_at.to_rfc3339(),
                entry.last_used_at.map(|t| t.to_rfc3339()),
                entry.usage_count,
            ],
        ).map_err(|e| StorageError::QueryFailed(e.to_string()))?;

        Ok(entry_id)
    }

    /// Get all dictionary entries for a device
    pub fn get_all_entries(&self, device_id: &str) -> Result<Vec<PersonalDictionaryEntry>> {
        let mut stmt = self.db.connection().prepare(
            "SELECT entry_id, device_id, phrase, replacement, case_sensitive, whole_word_only, category, created_at, last_used_at, usage_count
             FROM personal_dictionary WHERE device_id = ?1 ORDER BY created_at DESC"
        ).map_err(|e| StorageError::QueryFailed(e.to_string()))?;

        let entries = stmt.query_map(params![device_id], |row| {
            let case_sensitive: i32 = row.get(4)?;
            let whole_word_only: i32 = row.get(5)?;
            let category_str: String = row.get(6)?;
            let created_at_str: String = row.get(7)?;
            let last_used_at_str: Option<String> = row.get(8)?;

            Ok(PersonalDictionaryEntry {
                entry_id: row.get(0)?,
                device_id: row.get(1)?,
                phrase: row.get(2)?,
                replacement: row.get(3)?,
                case_sensitive: case_sensitive != 0,
                whole_word_only: whole_word_only != 0,
                category: match category_str.as_str() {
                    "technical" => DictionaryEntryCategory::Technical,
                    "business" => DictionaryEntryCategory::Business,
                    "medical" => DictionaryEntryCategory::Medical,
                    _ => DictionaryEntryCategory::General,
                },
                created_at: DateTime::parse_from_rfc3339(&created_at_str)
                    .map(|dt| dt.with_timezone(&Utc))
                    .unwrap_or_else(|_| Utc::now()),
                last_used_at: last_used_at_str
                    .and_then(|s| DateTime::parse_from_rfc3339(&s).ok())
                    .map(|dt| dt.with_timezone(&Utc)),
                usage_count: row.get(9)?,
            })
        }).map_err(|e| StorageError::QueryFailed(e.to_string()))?;

        let mut result = Vec::new();
        for entry in entries {
            result.push(entry.map_err(|e| StorageError::QueryFailed(e.to_string()))?);
        }

        Ok(result)
    }

    /// Get a dictionary entry by ID
    pub fn get_entry_by_id(&self, entry_id: &str, device_id: &str) -> Result<Option<PersonalDictionaryEntry>> {
        let mut stmt = self.db.connection().prepare(
            "SELECT entry_id, device_id, phrase, replacement, case_sensitive, whole_word_only, category, created_at, last_used_at, usage_count
             FROM personal_dictionary WHERE entry_id = ?1 AND device_id = ?2"
        ).map_err(|e| StorageError::QueryFailed(e.to_string()))?;

        let mut entries = stmt.query_map(params![entry_id, device_id], |row| {
            let case_sensitive: i32 = row.get(4)?;
            let whole_word_only: i32 = row.get(5)?;
            let category_str: String = row.get(6)?;
            let created_at_str: String = row.get(7)?;
            let last_used_at_str: Option<String> = row.get(8)?;

            Ok(PersonalDictionaryEntry {
                entry_id: row.get(0)?,
                device_id: row.get(1)?,
                phrase: row.get(2)?,
                replacement: row.get(3)?,
                case_sensitive: case_sensitive != 0,
                whole_word_only: whole_word_only != 0,
                category: match category_str.as_str() {
                    "technical" => DictionaryEntryCategory::Technical,
                    "business" => DictionaryEntryCategory::Business,
                    "medical" => DictionaryEntryCategory::Medical,
                    _ => DictionaryEntryCategory::General,
                },
                created_at: DateTime::parse_from_rfc3339(&created_at_str)
                    .map(|dt| dt.with_timezone(&Utc))
                    .unwrap_or_else(|_| Utc::now()),
                last_used_at: last_used_at_str
                    .and_then(|s| DateTime::parse_from_rfc3339(&s).ok())
                    .map(|dt| dt.with_timezone(&Utc)),
                usage_count: row.get(9)?,
            })
        }).map_err(|e| StorageError::QueryFailed(e.to_string()))?;

        match entries.next() {
            Some(entry) => Ok(Some(entry.map_err(|e| StorageError::QueryFailed(e.to_string()))?)),
            None => Ok(None),
        }
    }

    /// Remove a dictionary entry
    pub fn remove_entry(&self, entry_id: &str, device_id: &str) -> Result<()> {
        self.db.connection().execute(
            "DELETE FROM personal_dictionary WHERE entry_id = ?1 AND device_id = ?2",
            params![entry_id, device_id],
        ).map_err(|e| StorageError::QueryFailed(e.to_string()))?;

        Ok(())
    }

    /// Update a dictionary entry
    pub fn update_entry(
        &self,
        entry_id: &str,
        device_id: &str,
        phrase: Option<&str>,
        replacement: Option<&str>,
        case_sensitive: Option<bool>,
        category: Option<DictionaryEntryCategory>,
    ) -> Result<()> {
        let mut updates = Vec::new();
        let mut values: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();

        if let Some(p) = phrase {
            updates.push("phrase = ?");
            values.push(Box::new(p.to_string()));
        }
        if let Some(r) = replacement {
            updates.push("replacement = ?");
            values.push(Box::new(r.to_string()));
        }
        if let Some(c) = case_sensitive {
            updates.push("case_sensitive = ?");
            values.push(Box::new(c as i32));
        }
        if let Some(cat) = category {
            updates.push("category = ?");
            values.push(Box::new(cat.as_str().to_string()));
        }

        if updates.is_empty() {
            return Ok(());
        }

        let sql = format!(
            "UPDATE personal_dictionary SET {} WHERE entry_id = ? AND device_id = ?",
            updates.join(", ")
        );

        values.push(Box::new(entry_id.to_string()));
        values.push(Box::new(device_id.to_string()));

        let params: Vec<&dyn rusqlite::ToSql> = values.iter().map(|v| v.as_ref()).collect();

        self.db.connection().execute(&sql, params.as_slice())
            .map_err(|e| StorageError::QueryFailed(e.to_string()))?;

        Ok(())
    }

    /// Increment usage count for an entry
    pub fn increment_usage(&self, entry_id: &str, device_id: &str) -> Result<()> {
        self.db.connection().execute(
            "UPDATE personal_dictionary SET usage_count = usage_count + 1, last_used_at = ?1 WHERE entry_id = ?2 AND device_id = ?3",
            params![Utc::now().to_rfc3339(), entry_id, device_id],
        ).map_err(|e| StorageError::QueryFailed(e.to_string()))?;

        Ok(())
    }
}

/// Apply dictionary entries to text
///
/// This function replaces phrases in the text according to dictionary entries.
/// Longer, more specific phrases are applied first to avoid partial matches.
pub fn apply_dictionary_to_text(text: &str, entries: &[PersonalDictionaryEntry]) -> String {
    use regex::Regex;

    if entries.is_empty() {
        return text.to_string();
    }

    // Sort entries by phrase length (descending) to handle longer phrases first
    let mut sorted_entries = entries.to_vec();
    sorted_entries.sort_by(|a, b| b.phrase.len().cmp(&a.phrase.len()));

    let mut result = text.to_string();

    for entry in sorted_entries {
        if entry.phrase.is_empty() {
            continue;
        }

        if entry.whole_word_only {
            // Use word boundary matching for whole-word-only entries
            let pattern = if entry.case_sensitive {
                format!(r"\b{}\b", regex::escape(&entry.phrase))
            } else {
                format!(r"(?i)\b{}\b", regex::escape(&entry.phrase))
            };

            if let Ok(re) = Regex::new(&pattern) {
                result = re.replace_all(&result, entry.replacement.as_str()).to_string();
            }
        } else {
            // Simple substring replacement
            if entry.case_sensitive {
                result = result.replace(&entry.phrase, &entry.replacement);
            } else {
                // Case-insensitive replacement
                let pattern = format!(r"(?i){}", regex::escape(&entry.phrase));
                if let Ok(re) = Regex::new(&pattern) {
                    result = re.replace_all(&result, entry.replacement.as_str()).to_string();
                }
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apply_dictionary_simple() {
        let text = "I need to API the server";
        let entries = vec![
            PersonalDictionaryEntry {
                entry_id: "1".to_string(),
                device_id: "test".to_string(),
                phrase: "API".to_string(),
                replacement: "Application Programming Interface".to_string(),
                case_sensitive: false,
                whole_word_only: true,
                category: DictionaryEntryCategory::Technical,
                created_at: Utc::now(),
                last_used_at: None,
                usage_count: 0,
            },
        ];
        let result = apply_dictionary_to_text(text, &entries);
        assert_eq!(result, "I need to Application Programming Interface the server");
    }

    #[test]
    fn test_apply_dictionary_multiple() {
        let text = "Use the API and CI/CD pipeline";
        let entries = vec![
            PersonalDictionaryEntry {
                entry_id: "1".to_string(),
                device_id: "test".to_string(),
                phrase: "API".to_string(),
                replacement: "Application Programming Interface".to_string(),
                case_sensitive: false,
                whole_word_only: true,
                category: DictionaryEntryCategory::Technical,
                created_at: Utc::now(),
                last_used_at: None,
                usage_count: 0,
            },
            PersonalDictionaryEntry {
                entry_id: "2".to_string(),
                device_id: "test".to_string(),
                phrase: "CI/CD".to_string(),
                replacement: "Continuous Integration/Continuous Deployment".to_string(),
                case_sensitive: true,
                whole_word_only: false,
                category: DictionaryEntryCategory::Technical,
                created_at: Utc::now(),
                last_used_at: None,
                usage_count: 0,
            },
        ];
        let result = apply_dictionary_to_text(text, &entries);
        assert!(result.contains("Application Programming Interface"));
        assert!(result.contains("Continuous Integration/Continuous Deployment"));
    }

    #[test]
    fn test_apply_dictionary_case_sensitive() {
        let text = "The API and api are different";
        let entries = vec![
            PersonalDictionaryEntry {
                entry_id: "1".to_string(),
                device_id: "test".to_string(),
                phrase: "API".to_string(),
                replacement: "Application Programming Interface".to_string(),
                case_sensitive: true,
                whole_word_only: true,
                category: DictionaryEntryCategory::Technical,
                created_at: Utc::now(),
                last_used_at: None,
                usage_count: 0,
            },
        ];
        let result = apply_dictionary_to_text(text, &entries);
        assert!(result.contains("Application Programming Interface"));
        assert!(result.contains("api")); // lowercase should remain
    }

    #[test]
    fn test_apply_dictionary_no_entries() {
        let text = "No changes here";
        let entries: Vec<PersonalDictionaryEntry> = vec![];
        let result = apply_dictionary_to_text(text, &entries);
        assert_eq!(result, text);
    }
}