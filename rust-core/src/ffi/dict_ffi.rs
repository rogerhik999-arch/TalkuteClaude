//! Dictionary FFI functions
//!
//! Provides FFI functions for managing personal dictionary entries.

use crate::storage::models::{PersonalDictionaryEntry, DictionaryEntryCategory};
use crate::storage::dictionary::{DictionaryStorage, apply_dictionary_to_text};
use crate::storage::database::Database;
use crate::error::Result;

/// Add a dictionary entry
pub fn add_dictionary_entry(db: &Database, entry: &PersonalDictionaryEntry) -> Result<String> {
    let storage = DictionaryStorage::new(db);
    storage.add_entry(entry)
}

/// Get all dictionary entries for a device
pub fn get_all_dictionary_entries(db: &Database, device_id: &str) -> Result<Vec<PersonalDictionaryEntry>> {
    let storage = DictionaryStorage::new(db);
    storage.get_all_entries(device_id)
}

/// Get a dictionary entry by ID
pub fn get_dictionary_entry_by_id(db: &Database, entry_id: &str, device_id: &str) -> Result<Option<PersonalDictionaryEntry>> {
    let storage = DictionaryStorage::new(db);
    storage.get_entry_by_id(entry_id, device_id)
}

/// Remove a dictionary entry
pub fn remove_dictionary_entry(db: &Database, entry_id: &str, device_id: &str) -> Result<()> {
    let storage = DictionaryStorage::new(db);
    storage.remove_entry(entry_id, device_id)
}

/// Update a dictionary entry
pub fn update_dictionary_entry(
    db: &Database,
    entry_id: &str,
    device_id: &str,
    phrase: Option<&str>,
    replacement: Option<&str>,
    case_sensitive: Option<bool>,
    category: Option<DictionaryEntryCategory>,
) -> Result<()> {
    let storage = DictionaryStorage::new(db);
    storage.update_entry(entry_id, device_id, phrase, replacement, case_sensitive, category)
}

/// Increment usage count for a dictionary entry
pub fn increment_dictionary_usage(db: &Database, entry_id: &str, device_id: &str) -> Result<()> {
    let storage = DictionaryStorage::new(db);
    storage.increment_usage(entry_id, device_id)
}

/// Apply dictionary entries to text
pub fn apply_dictionary(db: &Database, device_id: &str, text: &str) -> Result<String> {
    let storage = DictionaryStorage::new(db);
    let entries = storage.get_all_entries(device_id)?;
    Ok(apply_dictionary_to_text(text, &entries))
}