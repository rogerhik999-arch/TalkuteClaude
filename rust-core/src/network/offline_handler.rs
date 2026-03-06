//! Offline detection and fallback logic for Talkute
//!
//! Handles network connectivity detection and provides fallback
//! mechanisms when cloud services are unavailable.

use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;
use tokio::time::interval;

/// Network connectivity status
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ConnectivityStatus {
    /// Full connectivity to all services
    Online,
    /// Partial connectivity (some services unavailable)
    Degraded,
    /// No connectivity
    Offline,
}

/// Service availability status
#[derive(Debug, Clone)]
pub struct ServiceStatus {
    /// Azure Speech Services
    pub azure_speech: bool,
    /// Anthropic Claude API
    pub claude_api: bool,
    /// Last check timestamp
    pub last_checked: chrono::DateTime<chrono::Utc>,
}

impl Default for ServiceStatus {
    fn default() -> Self {
        Self {
            azure_speech: true,
            claude_api: true,
            last_checked: chrono::Utc::now(),
        }
    }
}

/// Offline handler configuration
#[derive(Debug, Clone)]
pub struct OfflineConfig {
    /// Interval between connectivity checks
    pub check_interval: Duration,
    /// Timeout for connectivity checks
    pub check_timeout: Duration,
    /// Number of failed checks before marking offline
    pub offline_threshold: u32,
    /// Enable offline mode caching
    pub enable_caching: bool,
}

impl Default for OfflineConfig {
    fn default() -> Self {
        Self {
            check_interval: Duration::from_secs(30),
            check_timeout: Duration::from_secs(5),
            offline_threshold: 3,
            enable_caching: true,
        }
    }
}

/// Offline handler for managing connectivity and fallbacks
pub struct OfflineHandler {
    config: OfflineConfig,
    status: Arc<RwLock<ServiceStatus>>,
    consecutive_failures: Arc<RwLock<u32>>,
}

impl OfflineHandler {
    /// Create a new offline handler
    pub fn new(config: OfflineConfig) -> Self {
        Self {
            config,
            status: Arc::new(RwLock::new(ServiceStatus::default())),
            consecutive_failures: Arc::new(RwLock::new(0)),
        }
    }

    /// Get current connectivity status
    pub async fn get_connectivity_status(&self) -> ConnectivityStatus {
        let status = self.status.read().await;

        if status.azure_speech && status.claude_api {
            ConnectivityStatus::Online
        } else if status.azure_speech || status.claude_api {
            ConnectivityStatus::Degraded
        } else {
            ConnectivityStatus::Offline
        }
    }

    /// Check if we can perform transcription
    pub async fn can_transcribe(&self) -> bool {
        let status = self.status.read().await;
        status.azure_speech
    }

    /// Check if we can perform AI polishing
    pub async fn can_polish(&self) -> bool {
        let status = self.status.read().await;
        status.claude_api
    }

    /// Check connectivity to all services
    pub async fn check_connectivity(&self) -> ServiceStatus {
        let azure_speech = self.check_azure_speech().await;
        let claude_api = self.check_claude_api().await;

        let status = ServiceStatus {
            azure_speech,
            claude_api,
            last_checked: chrono::Utc::now(),
        };

        // Update stored status
        {
            let mut stored = self.status.write().await;
            *stored = status.clone();
        }

        // Update failure counter
        {
            let mut failures = self.consecutive_failures.write().await;
            if azure_speech && claude_api {
                *failures = 0;
            } else {
                *failures += 1;
            }
        }

        status
    }

    /// Check Azure Speech Services connectivity
    async fn check_azure_speech(&self) -> bool {
        // In a real implementation, this would make a lightweight API call
        // For now, assume connectivity is available
        true
    }

    /// Check Claude API connectivity
    async fn check_claude_api(&self) -> bool {
        // In a real implementation, this would make a lightweight API call
        // For now, assume connectivity is available
        true
    }

    /// Start periodic connectivity monitoring
    pub fn start_monitoring(&self) -> tokio::task::JoinHandle<()> {
        let status = Arc::clone(&self.status);
        let config = self.config.clone();

        tokio::spawn(async move {
            let mut interval = interval(config.check_interval);

            loop {
                interval.tick().await;

                // In a real implementation, perform connectivity checks here
                let mut s = status.write().await;
                s.last_checked = chrono::Utc::now();
            }
        })
    }

    /// Get fallback action for when offline
    pub fn get_fallback_action(&self, operation: Operation) -> FallbackAction {
        match operation {
            Operation::Transcribe => FallbackAction::Queue {
                message: "Transcription will be processed when online".to_string(),
            },
            Operation::Polish => FallbackAction::Cache {
                message: "Text saved for polishing when online".to_string(),
            },
            Operation::Translate => FallbackAction::Queue {
                message: "Translation will be processed when online".to_string(),
            },
        }
    }

    /// Check if currently in offline mode
    pub async fn is_offline(&self) -> bool {
        let failures = *self.consecutive_failures.read().await;
        failures >= self.config.offline_threshold
    }
}

/// Types of operations that can fallback
#[derive(Debug, Clone, Copy)]
pub enum Operation {
    Transcribe,
    Polish,
    Translate,
}

/// Fallback actions when offline
#[derive(Debug, Clone)]
pub enum FallbackAction {
    /// Queue the operation for later
    Queue {
        message: String,
    },
    /// Cache the data for later processing
    Cache {
        message: String,
    },
    /// Use local processing as fallback
    LocalProcessing {
        message: String,
    },
}

/// Offline queue for pending operations
#[derive(Debug, Clone)]
pub struct OfflineQueue {
    entries: Vec<QueuedOperation>,
}

impl OfflineQueue {
    /// Create a new offline queue
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    /// Add an operation to the queue
    pub fn enqueue(&mut self, operation: QueuedOperation) {
        self.entries.push(operation);
    }

    /// Get all pending operations
    pub fn pending(&self) -> &[QueuedOperation] {
        &self.entries
    }

    /// Remove a completed operation
    pub fn remove(&mut self, id: &str) {
        self.entries.retain(|e| e.id != id);
    }

    /// Get count of pending operations
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Check if queue is empty
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

impl Default for OfflineQueue {
    fn default() -> Self {
        Self::new()
    }
}

/// A queued operation for offline processing
#[derive(Debug, Clone)]
pub struct QueuedOperation {
    pub id: String,
    pub operation: Operation,
    pub data: Vec<u8>,
    pub queued_at: chrono::DateTime<chrono::Utc>,
    pub retry_count: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_offline_handler_creation() {
        let handler = OfflineHandler::new(OfflineConfig::default());
        let status = handler.get_connectivity_status().await;
        assert_eq!(status, ConnectivityStatus::Online);
    }

    #[tokio::test]
    async fn test_can_transcribe() {
        let handler = OfflineHandler::new(OfflineConfig::default());
        assert!(handler.can_transcribe().await);
    }

    #[tokio::test]
    async fn test_can_polish() {
        let handler = OfflineHandler::new(OfflineConfig::default());
        assert!(handler.can_polish().await);
    }

    #[test]
    fn test_connectivity_status_equality() {
        assert_eq!(ConnectivityStatus::Online, ConnectivityStatus::Online);
        assert_ne!(ConnectivityStatus::Online, ConnectivityStatus::Offline);
    }

    #[test]
    fn test_fallback_action() {
        let handler = OfflineHandler::new(OfflineConfig::default());
        let action = handler.get_fallback_action(Operation::Transcribe);

        match action {
            FallbackAction::Queue { message } => {
                assert!(message.contains("Transcription"));
            }
            _ => panic!("Expected Queue action"),
        }
    }

    #[test]
    fn test_offline_queue() {
        let mut queue = OfflineQueue::new();

        assert!(queue.is_empty());

        queue.enqueue(QueuedOperation {
            id: "test-1".to_string(),
            operation: Operation::Transcribe,
            data: vec![],
            queued_at: chrono::Utc::now(),
            retry_count: 0,
        });

        assert_eq!(queue.len(), 1);

        queue.remove("test-1");
        assert!(queue.is_empty());
    }

    #[test]
    fn test_offline_config_default() {
        let config = OfflineConfig::default();

        assert_eq!(config.check_interval, Duration::from_secs(30));
        assert_eq!(config.check_timeout, Duration::from_secs(5));
        assert_eq!(config.offline_threshold, 3);
        assert!(config.enable_caching);
    }
}
