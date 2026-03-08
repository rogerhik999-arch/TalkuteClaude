//! Tests for usage quota checking
//!
//! Tests the quota validation and tracking functionality.

use talkute_core::quota::tracker::QuotaTracker;
use talkute_core::storage::database::Database;

#[tokio::test]
async fn test_quota_tracker_creation() {
    let db = Database::in_memory().expect("Failed to create database");
    let tracker = QuotaTracker::new(db);
    assert!(tracker.is_ok());
}

#[tokio::test]
async fn test_quota_initial_state() {
    let db = Database::in_memory().expect("Failed to create database");
    let tracker = QuotaTracker::new(db).expect("Failed to create tracker");

    // Initial usage should be 0
    let usage = tracker.get_weekly_usage().await.expect("Failed to get usage");
    assert_eq!(usage, 0);
}

#[tokio::test]
async fn test_quota_available_when_under_limit() {
    let db = Database::in_memory().expect("Failed to create database");
    let tracker = QuotaTracker::new(db).expect("Failed to create tracker");

    // Should be available when under limit
    let available = tracker.check_quota(100).await.expect("Failed to check quota");
    assert!(available);
}

#[tokio::test]
async fn test_quota_add_words() {
    let db = Database::in_memory().expect("Failed to create database");
    let tracker = QuotaTracker::new(db).expect("Failed to create tracker");

    // Add some words
    tracker.add_words(50).await.expect("Failed to add words");

    // Usage should reflect the addition
    let usage = tracker.get_weekly_usage().await.expect("Failed to get usage");
    assert_eq!(usage, 50);
}

#[tokio::test]
async fn test_quota_multiple_additions() {
    let db = Database::in_memory().expect("Failed to create database");
    let tracker = QuotaTracker::new(db).expect("Failed to create tracker");

    // Add words multiple times
    tracker.add_words(100).await.expect("Failed to add words");
    tracker.add_words(200).await.expect("Failed to add words");
    tracker.add_words(50).await.expect("Failed to add words");

    // Usage should be cumulative
    let usage = tracker.get_weekly_usage().await.expect("Failed to get usage");
    assert_eq!(usage, 350);
}

#[tokio::test]
async fn test_quota_exceeds_limit() {
    let db = Database::in_memory().expect("Failed to create database");
    let tracker = QuotaTracker::new(db).expect("Failed to create tracker");

    // Add words up to the limit (4000)
    tracker.add_words(4000).await.expect("Failed to add words");

    // Check quota should fail for additional words
    let available = tracker.check_quota(1).await.expect("Failed to check quota");
    assert!(!available);
}

#[tokio::test]
async fn test_quota_near_limit() {
    let db = Database::in_memory().expect("Failed to create database");
    let tracker = QuotaTracker::new(db).expect("Failed to create tracker");

    // Add words near the limit
    tracker.add_words(3900).await.expect("Failed to add words");

    // Should be available for 100 words
    let available = tracker.check_quota(100).await.expect("Failed to check quota");
    assert!(available);

    // Should not be available for 101 words
    let available = tracker.check_quota(101).await.expect("Failed to check quota");
    assert!(!available);
}

#[tokio::test]
async fn test_quota_remaining_calculation() {
    let db = Database::in_memory().expect("Failed to create database");
    let tracker = QuotaTracker::new(db).expect("Failed to create tracker");

    // Add some words
    tracker.add_words(1500).await.expect("Failed to add words");

    // Get remaining quota
    let remaining = tracker.get_remaining_quota().await.expect("Failed to get remaining");
    assert_eq!(remaining, 2500); // 4000 - 1500
}

#[tokio::test]
async fn test_quota_percentage_used() {
    let db = Database::in_memory().expect("Failed to create database");
    let tracker = QuotaTracker::new(db).expect("Failed to create tracker");

    // Add 1000 words (25% of 4000)
    tracker.add_words(1000).await.expect("Failed to add words");

    // Get percentage used
    let percentage = tracker.get_percentage_used().await.expect("Failed to get percentage");
    assert!((percentage - 25.0).abs() < 0.1);
}

#[tokio::test]
async fn test_quota_weekly_limit() {
    let db = Database::in_memory().expect("Failed to create database");
    let tracker = QuotaTracker::new(db).expect("Failed to create tracker");

    // Weekly limit should be 4000
    let limit = tracker.get_weekly_limit();
    assert_eq!(limit, 4000);
}

#[tokio::test]
async fn test_quota_warning_threshold() {
    let db = Database::in_memory().expect("Failed to create database");
    let tracker = QuotaTracker::new(db).expect("Failed to create tracker");

    // Add 3200 words (80% of 4000)
    tracker.add_words(3200).await.expect("Failed to add words");

    // Should be at warning threshold (80%)
    let at_warning = tracker.is_at_warning_threshold().await.expect("Failed to check warning");
    assert!(at_warning);
}

#[tokio::test]
async fn test_quota_below_warning_threshold() {
    let db = Database::in_memory().expect("Failed to create database");
    let tracker = QuotaTracker::new(db).expect("Failed to create tracker");

    // Add 3000 words (75% of 4000)
    tracker.add_words(3000).await.expect("Failed to add words");

    // Should not be at warning threshold
    let at_warning = tracker.is_at_warning_threshold().await.expect("Failed to check warning");
    assert!(!at_warning);
}