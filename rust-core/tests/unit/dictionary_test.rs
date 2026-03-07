//! Dictionary repository unit tests

use talkute_core::storage::dictionary::{DictionaryRepository, SqliteDictionaryRepository};
use talkute_core::storage::database::Database;
use talkute_core::storage::models::{PersonalDictionaryEntry, DictionaryEntryCategory};
use chrono::Utc;

fn create_test_entry(phrase: &str, replacement: &str) -> PersonalDictionaryEntry {
    PersonalDictionaryEntry {
        entry_id: uuid::Uuid::new_v4().to_string(),
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

#[tokio::test]
async fn test_add_entry() {
    let db = Database::in_memory().unwrap();
    let repo = SqliteDictionaryRepository::new(db);

    let entry = create_test_entry("API", "Application Programming Interface");
    let result = repo.add(&entry).await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_get_all_entries() {
    let db = Database::in_memory().unwrap();
    let repo = SqliteDictionaryRepository::new(db);

    repo.add(&create_test_entry("API", "Application Programming Interface")).await.unwrap();
    repo.add(&create_test_entry("SDK", "Software Development Kit")).await.unwrap();

    let entries = repo.get_all().await.unwrap();
    assert_eq!(entries.len(), 2);
}

#[tokio::test]
async fn test_find_by_phrase() {
    let db = Database::in_memory().unwrap();
    let repo = SqliteDictionaryRepository::new(db);

    repo.add(&create_test_entry("API", "Application Programming Interface")).await.unwrap();

    let found = repo.find_by_phrase("API").await.unwrap();
    assert!(found.is_some());
    assert_eq!(found.unwrap().replacement, "Application Programming Interface");

    let not_found = repo.find_by_phrase("XYZ").await.unwrap();
    assert!(not_found.is_none());
}

#[tokio::test]
async fn test_update_entry() {
    let db = Database::in_memory().unwrap();
    let repo = SqliteDictionaryRepository::new(db);

    let entry = create_test_entry("API", "Application Programming Interface");
    repo.add(&entry).await.unwrap();

    let mut updated = entry.clone();
    updated.replacement = "API Endpoint".to_string();
    repo.update(&updated).await.unwrap();

    let found = repo.find_by_phrase("API").await.unwrap().unwrap();
    assert_eq!(found.replacement, "API Endpoint");
}

#[tokio::test]
async fn test_delete_entry() {
    let db = Database::in_memory().unwrap();
    let repo = SqliteDictionaryRepository::new(db);

    let entry = create_test_entry("API", "Application Programming Interface");
    repo.add(&entry).await.unwrap();

    repo.delete(&entry.entry_id).await.unwrap();

    let found = repo.find_by_phrase("API").await.unwrap();
    assert!(found.is_none());
}

#[tokio::test]
async fn test_increment_usage() {
    let db = Database::in_memory().unwrap();
    let repo = SqliteDictionaryRepository::new(db);

    let entry = create_test_entry("API", "Application Programming Interface");
    repo.add(&entry).await.unwrap();

    repo.increment_usage(&entry.entry_id).await.unwrap();

    let found = repo.find_by_phrase("API").await.unwrap().unwrap();
    assert_eq!(found.usage_count, 1);
}

#[tokio::test]
async fn test_find_by_category() {
    let db = Database::in_memory().unwrap();
    let repo = SqliteDictionaryRepository::new(db);

    let mut entry1 = create_test_entry("API", "Application Programming Interface");
    entry1.category = DictionaryEntryCategory::Technical;
    repo.add(&entry1).await.unwrap();

    let mut entry2 = create_test_entry("FYI", "For Your Information");
    entry2.category = DictionaryEntryCategory::Abbreviation;
    repo.add(&entry2).await.unwrap();

    let technical = repo.find_by_category(DictionaryEntryCategory::Technical).await.unwrap();
    assert_eq!(technical.len(), 1);
}

#[tokio::test]
async fn test_clear_all() {
    let db = Database::in_memory().unwrap();
    let repo = SqliteDictionaryRepository::new(db);

    repo.add(&create_test_entry("API", "Application Programming Interface")).await.unwrap();
    repo.add(&create_test_entry("SDK", "Software Development Kit")).await.unwrap();

    repo.clear().await.unwrap();

    let entries = repo.get_all().await.unwrap();
    assert!(entries.is_empty());
}