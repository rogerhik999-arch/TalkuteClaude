//! Tests for dictionary FFI functions

use talkute_core::storage::database::Database;
use talkute_core::storage::models::{PersonalDictionaryEntry, DictionaryEntryCategory};
use talkute_core::ffi::dict_ffi::*;
use chrono::Utc;

fn create_entry(phrase: &str, replacement: &str) -> PersonalDictionaryEntry {
    PersonalDictionaryEntry {
        entry_id: format!("entry-{}", uuid::Uuid::new_v4()),
        device_id: "test-device".to_string(),
        phrase: phrase.to_string(),
        replacement: replacement.to_string(),
        case_sensitive: false,
        whole_word_only: true,
        category: DictionaryEntryCategory::Technical,
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
fn test_add_dictionary_entry() {
    let db = Database::in_memory().expect("Failed to create database");
    setup_device(&db);

    let entry = create_entry("API", "Application Programming Interface");

    let result = add_dictionary_entry(&db, &entry);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), entry.entry_id);
}

#[test]
fn test_get_all_dictionary_entries() {
    let db = Database::in_memory().expect("Failed to create database");
    setup_device(&db);

    // Add entries
    let entry1 = create_entry("API", "Application Programming Interface");
    let entry2 = create_entry("CI/CD", "Continuous Integration/Continuous Deployment");

    add_dictionary_entry(&db, &entry1).expect("Failed to add entry1");
    add_dictionary_entry(&db, &entry2).expect("Failed to add entry2");

    // Get all entries
    let entries = get_all_dictionary_entries(&db, "test-device").expect("Failed to get entries");
    assert_eq!(entries.len(), 2);
}

#[test]
fn test_get_dictionary_entry_by_id() {
    let db = Database::in_memory().expect("Failed to create database");
    setup_device(&db);

    let entry = create_entry("API", "Application Programming Interface");

    add_dictionary_entry(&db, &entry).expect("Failed to add entry");

    // Get entry by ID
    let result = get_dictionary_entry_by_id(&db, &entry.entry_id, "test-device")
        .expect("Failed to get entry");

    assert!(result.is_some());
    let found = result.unwrap();
    assert_eq!(found.phrase, "API");
    assert_eq!(found.replacement, "Application Programming Interface");
}

#[test]
fn test_remove_dictionary_entry() {
    let db = Database::in_memory().expect("Failed to create database");
    setup_device(&db);

    let entry = create_entry("API", "Application Programming Interface");

    add_dictionary_entry(&db, &entry).expect("Failed to add entry");

    // Remove entry
    remove_dictionary_entry(&db, &entry.entry_id, "test-device")
        .expect("Failed to remove entry");

    // Verify removal
    let entries = get_all_dictionary_entries(&db, "test-device").expect("Failed to get entries");
    assert_eq!(entries.len(), 0);
}

#[test]
fn test_update_dictionary_entry() {
    let db = Database::in_memory().expect("Failed to create database");
    setup_device(&db);

    let entry = create_entry("API", "Application Programming Interface");

    add_dictionary_entry(&db, &entry).expect("Failed to add entry");

    // Update entry
    update_dictionary_entry(
        &db,
        &entry.entry_id,
        "test-device",
        Some("REST API"),
        Some("RESTful Application Programming Interface"),
        Some(true),
        Some(DictionaryEntryCategory::General),
    ).expect("Failed to update entry");

    // Verify update
    let found = get_dictionary_entry_by_id(&db, &entry.entry_id, "test-device")
        .expect("Failed to get entry")
        .expect("Entry not found");

    assert_eq!(found.phrase, "REST API");
    assert_eq!(found.replacement, "RESTful Application Programming Interface");
    assert!(found.case_sensitive);
    assert_eq!(found.category, DictionaryEntryCategory::General);
}

#[test]
fn test_increment_dictionary_usage() {
    let db = Database::in_memory().expect("Failed to create database");
    setup_device(&db);

    let entry = create_entry("API", "Application Programming Interface");

    add_dictionary_entry(&db, &entry).expect("Failed to add entry");

    // Increment usage
    increment_dictionary_usage(&db, &entry.entry_id, "test-device")
        .expect("Failed to increment usage");

    // Verify increment
    let found = get_dictionary_entry_by_id(&db, &entry.entry_id, "test-device")
        .expect("Failed to get entry")
        .expect("Entry not found");

    assert_eq!(found.usage_count, 1);
    assert!(found.last_used_at.is_some());
}

#[test]
fn test_apply_dictionary() {
    let db = Database::in_memory().expect("Failed to create database");
    setup_device(&db);

    // Add entry
    let entry = create_entry("API", "Application Programming Interface");
    add_dictionary_entry(&db, &entry).expect("Failed to add entry");

    // Apply dictionary to text
    let text = "I need to use the API for my project";
    let result = apply_dictionary(&db, "test-device", text).expect("Failed to apply dictionary");

    assert!(result.contains("Application Programming Interface"));
}

#[test]
fn test_multiple_devices_isolated() {
    let db = Database::in_memory().expect("Failed to create database");

    // Setup both devices
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

    // Add entry for device1
    let entry1 = PersonalDictionaryEntry {
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

    // Add entry for device2
    let entry2 = PersonalDictionaryEntry {
        entry_id: "entry-2".to_string(),
        device_id: "device-2".to_string(),
        phrase: "API".to_string(),
        replacement: "Advanced Programming Interface".to_string(),
        case_sensitive: false,
        whole_word_only: true,
        category: DictionaryEntryCategory::Technical,
        created_at: Utc::now(),
        last_used_at: None,
        usage_count: 0,
    };

    add_dictionary_entry(&db, &entry1).expect("Failed to add entry1");
    add_dictionary_entry(&db, &entry2).expect("Failed to add entry2");

    // Verify isolation
    let entries1 = get_all_dictionary_entries(&db, "device-1").expect("Failed to get entries");
    let entries2 = get_all_dictionary_entries(&db, "device-2").expect("Failed to get entries");

    assert_eq!(entries1.len(), 1);
    assert_eq!(entries2.len(), 1);
    assert_eq!(entries1[0].replacement, "Application Programming Interface");
    assert_eq!(entries2[0].replacement, "Advanced Programming Interface");
}