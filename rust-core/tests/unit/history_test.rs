//! History repository unit tests

use talkute_core::storage::history::{HistoryRepository, SqliteHistoryRepository};
use talkute_core::storage::database::Database;
use talkute_core::storage::models::HistoryEntry;
use chrono::Utc;

fn create_test_entry(raw_text: &str, polished_text: &str) -> HistoryEntry {
    HistoryEntry {
        entry_id: uuid::Uuid::new_v4().to_string(),
        device_id: "test-device".to_string(),
        raw_transcription: raw_text.to_string(),
        polished_text: polished_text.to_string(),
        final_text: polished_text.to_string(),
        context: None,
        created_at: Utc::now(),
        word_count: raw_text.split_whitespace().count() as i32,
        duration_ms: 5000,
        was_edited: false,
    }
}

#[tokio::test]
async fn test_add_entry() {
    let db = Database::in_memory().unwrap();
    let repo = SqliteHistoryRepository::new(db);

    let entry = create_test_entry("Hello world", "Hello world.");
    let result = repo.add(&entry).await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_get_recent() {
    let db = Database::in_memory().unwrap();
    let repo = SqliteHistoryRepository::new(db);

    repo.add(&create_test_entry("First", "First.")).await.unwrap();
    repo.add(&create_test_entry("Second", "Second.")).await.unwrap();

    let recent = repo.get_recent(10).await.unwrap();
    assert_eq!(recent.len(), 2);
}

#[tokio::test]
async fn test_get_by_id() {
    let db = Database::in_memory().unwrap();
    let repo = SqliteHistoryRepository::new(db);

    let entry = create_test_entry("Test", "Test.");
    repo.add(&entry).await.unwrap();

    let found = repo.get_by_id(&entry.entry_id).await.unwrap();
    assert!(found.is_some());
    assert_eq!(found.unwrap().raw_transcription, "Test");
}

#[tokio::test]
async fn test_delete_entry() {
    let db = Database::in_memory().unwrap();
    let repo = SqliteHistoryRepository::new(db);

    let entry = create_test_entry("Test", "Test.");
    repo.add(&entry).await.unwrap();

    repo.delete(&entry.entry_id).await.unwrap();

    let found = repo.get_by_id(&entry.entry_id).await.unwrap();
    assert!(found.is_none());
}

#[tokio::test]
async fn test_clear_all() {
    let db = Database::in_memory().unwrap();
    let repo = SqliteHistoryRepository::new(db);

    repo.add(&create_test_entry("First", "First.")).await.unwrap();
    repo.add(&create_test_entry("Second", "Second.")).await.unwrap();

    repo.clear().await.unwrap();

    let recent = repo.get_recent(10).await.unwrap();
    assert!(recent.is_empty());
}

#[tokio::test]
async fn test_get_by_date_range() {
    let db = Database::in_memory().unwrap();
    let repo = SqliteHistoryRepository::new(db);

    repo.add(&create_test_entry("Test", "Test.")).await.unwrap();

    let now = Utc::now();
    let start = now - chrono::Duration::days(1);
    let end = now + chrono::Duration::days(1);

    let entries = repo.get_by_date_range(start, end).await.unwrap();
    assert_eq!(entries.len(), 1);
}

#[tokio::test]
async fn test_search() {
    let db = Database::in_memory().unwrap();
    let repo = SqliteHistoryRepository::new(db);

    repo.add(&create_test_entry("Hello world", "Hello world.")).await.unwrap();
    repo.add(&create_test_entry("Goodbye moon", "Goodbye moon.")).await.unwrap();

    let results = repo.search("Hello").await.unwrap();
    assert_eq!(results.len(), 1);
    assert!(results[0].raw_transcription.contains("Hello"));
}

#[tokio::test]
async fn test_get_total_word_count() {
    let db = Database::in_memory().unwrap();
    let repo = SqliteHistoryRepository::new(db);

    repo.add(&create_test_entry("One two three", "One two three.")).await.unwrap();
    repo.add(&create_test_entry("Four five", "Four five.")).await.unwrap();

    let total = repo.get_total_word_count().await.unwrap();
    assert_eq!(total, 5); // 3 + 2 words
}

#[tokio::test]
async fn test_cleanup_old_entries() {
    let db = Database::in_memory().unwrap();
    let repo = SqliteHistoryRepository::new(db);

    repo.add(&create_test_entry("Test", "Test.")).await.unwrap();

    // Clean up entries older than 0 days (should remove everything)
    let deleted = repo.cleanup_old_entries(0).await.unwrap();
    assert!(deleted > 0);

    let recent = repo.get_recent(10).await.unwrap();
    assert!(recent.is_empty());
}

#[tokio::test]
async fn test_mark_as_edited() {
    let db = Database::in_memory().unwrap();
    let repo = SqliteHistoryRepository::new(db);

    let entry = create_test_entry("Test", "Test.");
    repo.add(&entry).await.unwrap();

    repo.mark_as_edited(&entry.entry_id).await.unwrap();

    let found = repo.get_by_id(&entry.entry_id).await.unwrap().unwrap();
    assert!(found.was_edited);
}