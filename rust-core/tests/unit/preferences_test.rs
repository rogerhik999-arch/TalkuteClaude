//! Preferences repository unit tests

use talkute_core::storage::preferences::{PreferencesRepository, SqlitePreferencesRepository};
use talkute_core::storage::database::Database;

#[tokio::test]
async fn test_get_nonexistent_preference() {
    let db = Database::in_memory().unwrap();
    let repo = SqlitePreferencesRepository::new(db);

    let value = repo.get("nonexistent_key").await.unwrap();
    assert!(value.is_none());
}

#[tokio::test]
async fn test_set_and_get_preference() {
    let db = Database::in_memory().unwrap();
    let repo = SqlitePreferencesRepository::new(db);

    repo.set("test_key", "test_value").await.unwrap();
    let value = repo.get("test_key").await.unwrap();

    assert_eq!(value, Some("test_value".to_string()));
}

#[tokio::test]
async fn test_update_preference() {
    let db = Database::in_memory().unwrap();
    let repo = SqlitePreferencesRepository::new(db);

    repo.set("test_key", "value1").await.unwrap();
    repo.set("test_key", "value2").await.unwrap();

    let value = repo.get("test_key").await.unwrap();
    assert_eq!(value, Some("value2".to_string()));
}

#[tokio::test]
async fn test_delete_preference() {
    let db = Database::in_memory().unwrap();
    let repo = SqlitePreferencesRepository::new(db);

    repo.set("test_key", "test_value").await.unwrap();
    repo.delete("test_key").await.unwrap();

    let value = repo.get("test_key").await.unwrap();
    assert!(value.is_none());
}

#[tokio::test]
async fn test_get_all_preferences() {
    let db = Database::in_memory().unwrap();
    let repo = SqlitePreferencesRepository::new(db);

    repo.set("key1", "value1").await.unwrap();
    repo.set("key2", "value2").await.unwrap();

    let all = repo.get_all().await.unwrap();
    assert_eq!(all.len(), 2);
    assert_eq!(all.get("key1"), Some(&"value1".to_string()));
    assert_eq!(all.get("key2"), Some(&"value2".to_string()));
}

#[tokio::test]
async fn test_get_bool_preference() {
    let db = Database::in_memory().unwrap();
    let repo = SqlitePreferencesRepository::new(db);

    repo.set("bool_key", "true").await.unwrap();

    let value = repo.get_bool("bool_key").await.unwrap();
    assert!(value);

    let default_value = repo.get_bool("nonexistent").await.unwrap();
    assert!(!default_value);
}

#[tokio::test]
async fn test_get_double_preference() {
    let db = Database::in_memory().unwrap();
    let repo = SqlitePreferencesRepository::new(db);

    repo.set("double_key", "3.14").await.unwrap();

    let value = repo.get_double("double_key").await.unwrap();
    assert!((value - 3.14).abs() < 0.001);
}

#[tokio::test]
async fn test_get_int_preference() {
    let db = Database::in_memory().unwrap();
    let repo = SqlitePreferencesRepository::new(db);

    repo.set("int_key", "42").await.unwrap();

    let value = repo.get_int("int_key").await.unwrap();
    assert_eq!(value, 42);
}