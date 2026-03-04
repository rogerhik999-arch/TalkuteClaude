//! Session manager for voice input sessions

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;
use chrono::{DateTime, Utc};

use crate::error::{Error, Result};
use super::bridge::{SessionStatus, VoiceSessionInfo};

/// Internal session state
#[derive(Debug, Clone)]
pub struct Session {
    pub id: String,
    pub device_id: String,
    pub context_id: Option<String>,
    pub status: SessionStatus,
    pub started_at: DateTime<Utc>,
    pub raw_transcription: String,
    pub polished_text: String,
    pub word_count: i32,
}

impl Session {
    pub fn new(device_id: String, context_id: Option<String>) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            device_id,
            context_id,
            status: SessionStatus::Idle,
            started_at: Utc::now(),
            raw_transcription: String::new(),
            polished_text: String::new(),
            word_count: 0,
        }
    }

    pub fn to_info(&self) -> VoiceSessionInfo {
        let duration = (Utc::now() - self.started_at).num_seconds();
        VoiceSessionInfo {
            session_id: self.id.clone(),
            status: self.status.clone(),
            started_at: self.started_at.to_rfc3339(),
            duration_seconds: duration,
            word_count: self.word_count,
        }
    }
}

/// Global session manager
pub struct SessionManager {
    sessions: Arc<RwLock<HashMap<String, Session>>>,
}

impl SessionManager {
    /// Create a new session manager
    pub fn new() -> Self {
        Self {
            sessions: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Get the global session manager instance
    pub fn global() -> &'static Self {
        static INSTANCE: once_cell::sync::Lazy<SessionManager> =
            once_cell::sync::Lazy::new(SessionManager::new);
        &INSTANCE
    }

    /// Create a new session
    pub async fn create_session(&self, device_id: &str, context_id: Option<String>) -> Result<String> {
        let session = Session::new(device_id.to_string(), context_id);
        let session_id = session.id.clone();

        let mut sessions = self.sessions.write().await;
        sessions.insert(session_id.clone(), session);

        Ok(session_id)
    }

    /// Get a session by ID
    pub async fn get_session(&self, session_id: &str) -> Result<Session> {
        let sessions = self.sessions.read().await;
        sessions.get(session_id)
            .cloned()
            .ok_or_else(|| Error::Unknown(format!("Session not found: {}", session_id)))
    }

    /// Update session status
    pub async fn update_status(&self, session_id: &str, status: SessionStatus) -> Result<()> {
        let mut sessions = self.sessions.write().await;
        if let Some(session) = sessions.get_mut(session_id) {
            session.status = status;
            Ok(())
        } else {
            Err(Error::Unknown(format!("Session not found: {}", session_id)))
        }
    }

    /// Set raw transcription
    pub async fn set_raw_transcription(&self, session_id: &str, text: &str) -> Result<()> {
        let mut sessions = self.sessions.write().await;
        if let Some(session) = sessions.get_mut(session_id) {
            session.raw_transcription = text.to_string();
            session.word_count = text.split_whitespace().count() as i32;
            Ok(())
        } else {
            Err(Error::Unknown(format!("Session not found: {}", session_id)))
        }
    }

    /// Set polished text
    pub async fn set_polished_text(&self, session_id: &str, text: &str) -> Result<()> {
        let mut sessions = self.sessions.write().await;
        if let Some(session) = sessions.get_mut(session_id) {
            session.polished_text = text.to_string();
            Ok(())
        } else {
            Err(Error::Unknown(format!("Session not found: {}", session_id)))
        }
    }

    /// Remove a session
    pub async fn remove_session(&self, session_id: &str) -> Result<Session> {
        let mut sessions = self.sessions.write().await;
        sessions.remove(session_id)
            .ok_or_else(|| Error::Unknown(format!("Session not found: {}", session_id)))
    }

    /// Get session info
    pub async fn get_session_info(&self, session_id: &str) -> Result<VoiceSessionInfo> {
        let session = self.get_session(session_id).await?;
        Ok(session.to_info())
    }
}

impl Default for SessionManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Extension trait for async methods
impl SessionManager {
    /// Check if a session exists
    pub async fn session_exists(&self, session_id: &str) -> bool {
        let sessions = self.sessions.read().await;
        sessions.contains_key(session_id)
    }

    /// Get all active sessions
    pub async fn get_all_sessions(&self) -> Vec<VoiceSessionInfo> {
        let sessions = self.sessions.read().await;
        sessions.values().map(|s| s.to_info()).collect()
    }
}
