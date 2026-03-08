//! Tests for dictionary import/export functionality

use talkute_core::storage::database::Database;
use talkute_core::storage::models::{PersonalDictionaryEntry, DictionaryEntryCategory};
use talkute_core::storage::dictionary::DictionaryStorage;
use chrono::Utc;

fn create_entry(phrase: &str, replacement: &str, category: DictionaryEntryCategory) -> PersonalDictionaryEntry {
    PersonalDictionaryEntry {
        entry_id: format!("entry-{}", uuid::Uuid::new_v4()),
        device_id: "test-device".to_string(),
        phrase: phrase.to_string(),
        replacement: replacement.to_string(),
        case_sensitive: false,
        whole_word_only: true,
        category,
        created_at: Utc::now(),
        last_used_at: None,
        usage_count: 0,
    }
}

fn setup_device(db: &Database) {
    db.connection().execute(
        "INSERT OR IGNORE INTO device_profiles (device_id, created_at, last_active_at)
         VALUES ('test-device', datetime('now'), datetime('now'))",
        [],
    ).expect("Failed to create device profile");
}

#[test]
fn test_export_dictionary_to_json() {
    let db = Database::in_memory().expect("Failed to create database");
    setup_device(&db);

    let storage = DictionaryStorage::new(&db);

    // Add entries
    let entry1 = create_entry("API", "Application Programming Interface", DictionaryEntryCategory::Technical);
    let entry2 = create_entry("Q1", "First Quarter", DictionaryEntryCategory::Business);

    storage.add_entry(&entry1).expect("Failed to add entry1");
    storage.add_entry(&entry2).expect("Failed to add entry2");

    // Export to JSON
    let json = storage.export_to_json("test-device").expect("Failed to export");

    // Verify JSON structure
    assert!(json.contains("\"phrase\": \"API\""));
    assert!(json.contains("\"replacement\": \"Application Programming Interface\""));
    assert!(json.contains("\"phrase\": \"Q1\""));
    assert!(json.contains("\"category\": \"technical\""));
    assert!(json.contains("\"category\": \"business\""));
}

#[test]
fn test_export_empty_dictionary() {
    let db = Database::in_memory().expect("Failed to create database");
    setup_device(&db);

    let storage = DictionaryStorage::new(&db);

    // Export empty dictionary
    let json = storage.export_to_json("test-device").expect("Failed to export");

    // Should be valid JSON array
    let parsed: serde_json::Value = serde_json::from_str(&json).expect("Invalid JSON");
    assert!(parsed.is_array());
    assert_eq!(parsed.as_array().unwrap().len(), 0);
}

#[test]
fn test_import_dictionary_from_json() {
    let db = Database::in_memory().expect("Failed to create database");
    setup_device(&db);

    let storage = DictionaryStorage::new(&db);

    // Import JSON
    let json = r#"[
        {
            "phrase": "API",
            "replacement": "Application Programming Interface",
            "category": "technical",
            "case_sensitive": false,
            "whole_word_only": true
        },
        {
            "phrase": "ROI",
            "replacement": "Return on Investment",
            "category": "business",
            "case_sensitive": true,
            "whole_word_only": true
        }
    ]"#;

    let count = storage.import_from_json("test-device", json).expect("Failed to import");

    assert_eq!(count, 2);

    // Verify entries were imported
    let entries = storage.get_all_entries("test-device").expect("Failed to get entries");
    assert_eq!(entries.len(), 2);

    // Check specific entries
    let api_entry = entries.iter().find(|e| e.phrase == "API");
    assert!(api_entry.is_some());
    assert_eq!(api_entry.unwrap().replacement, "Application Programming Interface");

    let roi_entry = entries.iter().find(|e| e.phrase == "ROI");
    assert!(roi_entry.is_some());
    assert_eq!(roi_entry.unwrap().replacement, "Return on Investment");
    assert!(roi_entry.unwrap().case_sensitive);
}

#[test]
fn test_import_handles_invalid_json() {
    let db = Database::in_memory().expect("Failed to create database");
    setup_device(&db);

    let storage = DictionaryStorage::new(&db);

    // Invalid JSON
    let json = r#"not valid json"#;

    let result = storage.import_from_json("test-device", json);
    assert!(result.is_err());
}

#[test]
fn test_import_handles_partial_invalid_entries() {
    let db = Database::in_memory().expect("Failed to create database");
    setup_device(&db);

    let storage = DictionaryStorage::new(&db);

    // JSON with one valid and one invalid entry (missing required field)
    let json = r#"[
        {
            "phrase": "API",
            "replacement": "Application Programming Interface",
            "category": "technical",
            "case_sensitive": false,
            "whole_word_only": true
        },
        {
            "phrase": "Missing replacement",
            "category": "general"
        }
    ]"#;

    // Should still import valid entries
    let count = storage.import_from_json("test-device", json).expect("Failed to import");
    assert_eq!(count, 1);

    // Verify only valid entry was imported
    let entries = storage.get_all_entries("test-device").expect("Failed to get entries");
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].phrase, "API");
}

#[test]
fn test_export_import_roundtrip() {
    let db = Database::in_memory().expect("Failed to create database");
    setup_device(&db);

    let storage = DictionaryStorage::new(&db);

    // Add original entries
    let entry1 = create_entry("API", "Application Programming Interface", DictionaryEntryCategory::Technical);
    let entry2 = create_entry("MRI", "Magnetic Resonance Imaging", DictionaryEntryCategory::Medical);

    storage.add_entry(&entry1).expect("Failed to add entry1");
    storage.add_entry(&entry2).expect("Failed to add entry2");

    // Export
    let json = storage.export_to_json("test-device").expect("Failed to export");

    // Clear database
    let entries = storage.get_all_entries("test-device").expect("Failed to get entries");
    for entry in entries {
        storage.remove_entry(&entry.entry_id, "test-device").expect("Failed to remove");
    }

    // Verify cleared
    assert_eq!(storage.get_all_entries("test-device").unwrap().len(), 0);

    // Import back
    let count = storage.import_from_json("test-device", &json).expect("Failed to import");
    assert_eq!(count, 2);

    // Verify entries match
    let imported = storage.get_all_entries("test-device").expect("Failed to get entries");
    assert_eq!(imported.len(), 2);

    let phrases: Vec<&str> = imported.iter().map(|e| e.phrase.as_str()).collect();
    assert!(phrases.contains(&"API"));
    assert!(phrases.contains(&"MRI"));
}

#[test]
fn test_import_merges_with_existing() {
    let db = Database::in_memory().expect("Failed to create database");
    setup_device(&db);

    let storage = DictionaryStorage::new(&db);

    // Add existing entry
    let existing = create_entry("API", "Application Programming Interface", DictionaryEntryCategory::Technical);
    storage.add_entry(&existing).expect("Failed to add existing");

    // Import new entry
    let json = r#"[
        {
            "phrase": "CI/CD",
            "replacement": "Continuous Integration/Continuous Deployment",
            "category": "technical",
            "case_sensitive": false,
            "whole_word_only": true
        }
    ]"#;

    let count = storage.import_from_json("test-device", json).expect("Failed to import");
    assert_eq!(count, 1);

    // Should have both entries
    let entries = storage.get_all_entries("test-device").expect("Failed to get entries");
    assert_eq!(entries.len(), 2);
}

#[test]
fn test_import_respects_device_isolation() {
    let db = Database::in_memory().expect("Failed to create database");

    // Setup two devices
    db.connection().execute(
        "INSERT OR IGNORE INTO device_profiles (device_id, created_at, last_active_at)
         VALUES ('device-1', datetime('now'), datetime('now'))",
        [],
    ).expect("Failed to create device-1");

    db.connection().execute(
        "INSERT OR IGNORE INTO device_profiles (device_id, created_at, last_active_at)
         VALUES ('device-2', datetime('now'), datetime('now'))",
        [],
    ).expect("Failed to create device-2");

    let storage = DictionaryStorage::new(&db);

    // Add entry to device-1
    let entry = PersonalDictionaryEntry {
        entry_id: "entry-1".to_string(),
        device_id: "device-1".to_string(),
        phrase: "API".to_string(),
        replacement: "Application Programming Interface".to_string(),
        case_sensitive: false,
        whole_word_only: true,
        category: DictionaryEntryCategory::Technical,
        created_at: Utc::now(),
        last_used_at: None,
        usage_count: 0,
    };
    storage.add_entry(&entry).expect("Failed to add entry");

    // Export from device-1
    let json = storage.export_to_json("device-1").expect("Failed to export");

    // Import to device-2
    let count = storage.import_from_json("device-2", &json).expect("Failed to import");
    assert_eq!(count, 1);

    // Verify device isolation
    let entries1 = storage.get_all_entries("device-1").expect("Failed to get entries");
    let entries2 = storage.get_all_entries("device-2").expect("Failed to get entries");

    assert_eq!(entries1.len(), 1);
    assert_eq!(entries2.len(), 1);
    assert_ne!(entries1[0].device_id, entries2[0].device_id);
}