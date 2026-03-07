//! Session manager for state machine transitions

use std::sync::Arc;
use tokio::sync::{Mutex, broadcast};
use super::{SessionState, TranscriptionSession};

/// Event emitted when session state changes
#[derive(Clone, Debug)]
pub struct SessionStateEvent {
    pub session_id: String,
    pub state: SessionState,
    pub error_message: Option<String>,
    pub progress: Option<f32>,
}

/// Manages the active transcription session and state transitions
pub struct SessionManager {
    /// Current active session (if any)
    current_session: Arc<Mutex<Option<TranscriptionSession>>>,
    /// Broadcast channel for state change events
    state_sender: broadcast::Sender<SessionStateEvent>,
}

impl SessionManager {
    /// Create a new session manager
    pub fn new() -> Self {
        let (state_sender, _) = broadcast::channel(16);
        Self {
            current_session: Arc::new(Mutex::new(None)),
            state_sender,
        }
    }

    /// Subscribe to state change events
    pub fn subscribe(&self) -> broadcast::Receiver<SessionStateEvent> {
        self.state_sender.subscribe()
    }

    /// Start a new recording session
    pub async fn start_session(&self) -> Result<String, String> {
        let mut session_guard = self.current_session.lock().await;

        // Check if there's already an active session
        if let Some(ref session) = *session_guard {
            if session.state == SessionState::Recording || session.state == SessionState::Processing {
                return Err("Already recording".to_string());
            }
        }

        // Create new session
        let mut new_session = TranscriptionSession::new();
        new_session.start_recording();
        let session_id = new_session.id.to_string();

        // Emit state change event
        let _ = self.state_sender.send(SessionStateEvent {
            session_id: session_id.clone(),
            state: SessionState::Recording,
            error_message: None,
            progress: None,
        });

        *session_guard = Some(new_session);
        Ok(session_id)
    }

    /// Stop the current recording
    pub async fn stop_recording(&self, session_id: &str) -> Result<(), String> {
        let mut session_guard = self.current_session.lock().await;

        if let Some(ref mut session) = *session_guard {
            if session.id.to_string() != session_id {
                return Err("Invalid session".to_string());
            }
            if session.state != SessionState::Recording {
                return Err("Not recording".to_string());
            }

            // For now, use placeholder duration
            session.stop_recording(0);

            // Emit state change event
            let _ = self.state_sender.send(SessionStateEvent {
                session_id: session.id.to_string(),
                state: SessionState::Processing,
                error_message: None,
                progress: Some(0.0),
            });

            Ok(())
        } else {
            Err("No active session".to_string())
        }
    }

    /// Cancel the current recording
    pub async fn cancel_session(&self, session_id: &str) -> Result<(), String> {
        let mut session_guard = self.current_session.lock().await;

        if let Some(ref mut session) = *session_guard {
            if session.id.to_string() != session_id {
                return Err("Invalid session".to_string());
            }

            session.cancel();

            // Emit state change event
            let _ = self.state_sender.send(SessionStateEvent {
                session_id: session.id.to_string(),
                state: SessionState::Idle,
                error_message: None,
                progress: None,
            });

            *session_guard = None;
            Ok(())
        } else {
            Err("No active session".to_string())
        }
    }

    /// Complete the session with polished text
    pub async fn complete_session(&self, session_id: &str, polished_text: String) -> Result<(), String> {
        let mut session_guard = self.current_session.lock().await;

        if let Some(ref mut session) = *session_guard {
            if session.id.to_string() != session_id {
                return Err("Invalid session".to_string());
            }

            session.complete(polished_text);

            // Emit state change event
            let _ = self.state_sender.send(SessionStateEvent {
                session_id: session.id.to_string(),
                state: SessionState::Idle,
                error_message: None,
                progress: Some(1.0),
            });

            *session_guard = None;
            Ok(())
        } else {
            Err("No active session".to_string())
        }
    }

    /// Fail the session with an error
    pub async fn fail_session(&self, session_id: &str, error_message: String) -> Result<(), String> {
        let mut session_guard = self.current_session.lock().await;

        if let Some(ref mut session) = *session_guard {
            if session.id.to_string() != session_id {
                return Err("Invalid session".to_string());
            }

            session.fail(error_message.clone());

            // Emit state change event
            let _ = self.state_sender.send(SessionStateEvent {
                session_id: session.id.to_string(),
                state: SessionState::Error,
                error_message: Some(error_message),
                progress: None,
            });

            Ok(())
        } else {
            Err("No active session".to_string())
        }
    }

    /// Retry a failed session
    pub async fn retry_session(&self, session_id: &str) -> Result<(), String> {
        let mut session_guard = self.current_session.lock().await;

        if let Some(ref mut session) = *session_guard {
            if session.id.to_string() != session_id {
                return Err("Invalid session".to_string());
            }

            if !session.retry() {
                return Err("Max retries exceeded".to_string());
            }

            // Emit state change event
            let _ = self.state_sender.send(SessionStateEvent {
                session_id: session.id.to_string(),
                state: SessionState::Processing,
                error_message: None,
                progress: Some(0.0),
            });

            Ok(())
        } else {
            Err("No active session".to_string())
        }
    }

    /// Get the current session state
    pub async fn get_state(&self) -> SessionState {
        let session_guard = self.current_session.lock().await;
        session_guard.as_ref().map(|s| s.state).unwrap_or(SessionState::Idle)
    }

    /// Get the current session ID (if any)
    pub async fn get_session_id(&self) -> Option<String> {
        let session_guard = self.current_session.lock().await;
        session_guard.as_ref().map(|s| s.id.to_string())
    }
}

impl Default for SessionManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Global session manager singleton
pub static SESSION_MANAGER: once_cell::sync::Lazy<SessionManager> =
    once_cell::sync::Lazy::new(SessionManager::new);