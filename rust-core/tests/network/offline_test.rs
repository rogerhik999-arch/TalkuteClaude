//! Tests for offline mode handling
//!
//! Validates the offline detection and fallback logic.

use talkute_core::network::offline_handler::{
    ConnectivityStatus, FallbackAction, OfflineConfig, OfflineHandler, OfflineQueue,
    Operation, QueuedOperation, ServiceStatus,
};
use std::time::Duration;

#[tokio::test]
async fn test_offline_handler_creation() {
    let handler = OfflineHandler::new(OfflineConfig::default());
    let status = handler.get_connectivity_status().await;
    assert_eq!(status, ConnectivityStatus::Online);
}

#[tokio::test]
async fn test_can_transcribe_when_online() {
    let handler = OfflineHandler::new(OfflineConfig::default());
    assert!(handler.can_transcribe().await);
}

#[tokio::test]
async fn test_can_polish_when_online() {
    let handler = OfflineHandler::new(OfflineConfig::default());
    assert!(handler.can_polish().await);
}

#[tokio::test]
async fn test_check_connectivity() {
    let handler = OfflineHandler::new(OfflineConfig::default());
    let status = handler.check_connectivity().await;

    assert!(status.azure_speech);
    assert!(status.claude_api);
}

#[tokio::test]
async fn test_is_offline_initially_false() {
    let handler = OfflineHandler::new(OfflineConfig::default());
    assert!(!handler.is_offline().await);
}

#[test]
fn test_connectivity_status_equality() {
    assert_eq!(ConnectivityStatus::Online, ConnectivityStatus::Online);
    assert_eq!(ConnectivityStatus::Offline, ConnectivityStatus::Offline);
    assert_eq!(ConnectivityStatus::Degraded, ConnectivityStatus::Degraded);

    assert_ne!(ConnectivityStatus::Online, ConnectivityStatus::Offline);
    assert_ne!(ConnectivityStatus::Online, ConnectivityStatus::Degraded);
}

#[test]
fn test_fallback_action_for_transcribe() {
    let handler = OfflineHandler::new(OfflineConfig::default());
    let action = handler.get_fallback_action(Operation::Transcribe);

    match action {
        FallbackAction::Queue { message } => {
            assert!(message.contains("Transcription"));
        }
        _ => panic!("Expected Queue action for Transcribe operation"),
    }
}

#[test]
fn test_fallback_action_for_polish() {
    let handler = OfflineHandler::new(OfflineConfig::default());
    let action = handler.get_fallback_action(Operation::Polish);

    match action {
        FallbackAction::Cache { message } => {
            assert!(message.contains("polishing"));
        }
        _ => panic!("Expected Cache action for Polish operation"),
    }
}

#[test]
fn test_fallback_action_for_translate() {
    let handler = OfflineHandler::new(OfflineConfig::default());
    let action = handler.get_fallback_action(Operation::Translate);

    match action {
        FallbackAction::Queue { message } => {
            assert!(message.contains("Translation"));
        }
        _ => panic!("Expected Queue action for Translate operation"),
    }
}

#[test]
fn test_offline_queue_creation() {
    let queue = OfflineQueue::new();
    assert!(queue.is_empty());
    assert_eq!(queue.len(), 0);
}

#[test]
fn test_offline_queue_enqueue() {
    let mut queue = OfflineQueue::new();

    let operation = QueuedOperation {
        id: "test-1".to_string(),
        operation: Operation::Transcribe,
        data: vec![1, 2, 3],
        queued_at: chrono::Utc::now(),
        retry_count: 0,
    };

    queue.enqueue(operation);

    assert!(!queue.is_empty());
    assert_eq!(queue.len(), 1);
}

#[test]
fn test_offline_queue_remove() {
    let mut queue = OfflineQueue::new();

    queue.enqueue(QueuedOperation {
        id: "test-1".to_string(),
        operation: Operation::Transcribe,
        data: vec![],
        queued_at: chrono::Utc::now(),
        retry_count: 0,
    });

    queue.enqueue(QueuedOperation {
        id: "test-2".to_string(),
        operation: Operation::Polish,
        data: vec![],
        queued_at: chrono::Utc::now(),
        retry_count: 0,
    });

    assert_eq!(queue.len(), 2);

    queue.remove("test-1");
    assert_eq!(queue.len(), 1);

    queue.remove("test-2");
    assert!(queue.is_empty());
}

#[test]
fn test_offline_queue_pending() {
    let mut queue = OfflineQueue::new();

    queue.enqueue(QueuedOperation {
        id: "test-1".to_string(),
        operation: Operation::Transcribe,
        data: vec![],
        queued_at: chrono::Utc::now(),
        retry_count: 0,
    });

    let pending = queue.pending();
    assert_eq!(pending.len(), 1);
    assert_eq!(pending[0].id, "test-1");
}

#[test]
fn test_offline_config_default() {
    let config = OfflineConfig::default();

    assert_eq!(config.check_interval, Duration::from_secs(30));
    assert_eq!(config.check_timeout, Duration::from_secs(5));
    assert_eq!(config.offline_threshold, 3);
    assert!(config.enable_caching);
}

#[test]
fn test_offline_config_custom() {
    let config = OfflineConfig {
        check_interval: Duration::from_secs(60),
        check_timeout: Duration::from_secs(10),
        offline_threshold: 5,
        enable_caching: false,
    };

    assert_eq!(config.check_interval, Duration::from_secs(60));
    assert_eq!(config.check_timeout, Duration::from_secs(10));
    assert_eq!(config.offline_threshold, 5);
    assert!(!config.enable_caching);
}

#[test]
fn test_service_status_default() {
    let status = ServiceStatus::default();

    assert!(status.azure_speech);
    assert!(status.claude_api);
}

#[test]
fn test_queued_operation() {
    let now = chrono::Utc::now();
    let operation = QueuedOperation {
        id: "op-123".to_string(),
        operation: Operation::Transcribe,
        data: vec![1, 2, 3, 4],
        queued_at: now,
        retry_count: 2,
    };

    assert_eq!(operation.id, "op-123");
    assert_eq!(operation.operation, Operation::Transcribe);
    assert_eq!(operation.data.len(), 4);
    assert_eq!(operation.retry_count, 2);
}

#[tokio::test]
async fn test_multiple_connectivity_checks() {
    let handler = OfflineHandler::new(OfflineConfig::default());

    // Perform multiple checks
    for _ in 0..5 {
        let status = handler.check_connectivity().await;
        assert!(status.azure_speech);
        assert!(status.claude_api);
    }

    // Should still be online
    let final_status = handler.get_connectivity_status().await;
    assert_eq!(final_status, ConnectivityStatus::Online);
}

#[test]
fn test_operation_variants() {
    let transcribe = Operation::Transcribe;
    let polish = Operation::Polish;
    let translate = Operation::Translate;

    // Verify all variants exist
    match transcribe {
        Operation::Transcribe => {}
        _ => panic!("Expected Transcribe variant"),
    }

    match polish {
        Operation::Polish => {}
        _ => panic!("Expected Polish variant"),
    }

    match translate {
        Operation::Translate => {}
        _ => panic!("Expected Translate variant"),
    }
}
