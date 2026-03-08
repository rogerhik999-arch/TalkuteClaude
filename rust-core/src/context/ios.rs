//! iOS context detection
//!
//! iOS does not allow automatic detection of the foreground app.
//! This module provides manual context selection as a fallback.

use crate::error::{Result, ContextError};
use crate::storage::models::ApplicationContext;
use crate::context::detector::ContextDetector;
use uuid::Uuid;
use std::sync::atomic::{AtomicI32, Ordering};

/// iOS-specific context detector with manual selection support
pub struct IOSContextDetector {
    available: bool,
    /// Currently selected manual context (-1 means not set)
    manual_context: AtomicI32,
}

/// Manual context categories available on iOS
const MANUAL_CONTEXTS: &[&str] = &[
    "email",
    "chat",
    "document",
    "code",
    "browser",
    "general",
];

impl IOSContextDetector {
    /// Create a new iOS context detector
    pub fn new() -> Self {
        Self {
            available: cfg!(target_os = "ios"),
            manual_context: AtomicI32::new(-1), // -1 = not set
        }
    }

    /// Check if iOS context detection is available
    pub fn is_available(&self) -> bool {
        self.available
    }

    /// Get available manual context categories
    pub fn get_manual_contexts(&self) -> Vec<String> {
        MANUAL_CONTEXTS.iter().map(|s| s.to_string()).collect()
    }

    /// Set the current context manually
    pub fn set_manual_context(&self, context: &str) -> Result<()> {
        let index = MANUAL_CONTEXTS.iter()
            .position(|&c| c == context)
            .ok_or_else(|| ContextError::InvalidCategory)?;

        self.manual_context.store(index as i32, Ordering::SeqCst);
        Ok(())
    }

    /// Get the currently selected context
    pub fn get_current_context(&self) -> Result<ApplicationContext> {
        let index = self.manual_context.load(Ordering::SeqCst);

        let category = if index >= 0 && (index as usize) < MANUAL_CONTEXTS.len() {
            MANUAL_CONTEXTS[index as usize]
        } else {
            "general"
        };

        Ok(ApplicationContext::new(
            Uuid::new_v4().to_string(),
            format!("manual-{}", category),
            category.to_string(),
        ))
    }

    /// Get the default context (used when nothing is set)
    pub fn get_default_context(&self) -> ApplicationContext {
        ApplicationContext::new(
            Uuid::new_v4().to_string(),
            "default".to_string(),
            "general".to_string(),
        )
    }

    /// Map a bundle identifier to an application category
    pub fn map_to_category(&self, bundle_id: &str) -> &'static str {
        let id = bundle_id.to_lowercase();

        // Apple apps
        if id.contains("com.apple.mobilemail") {
            return "email";
        }
        if id.contains("com.apple.mobilesms") {
            return "chat";
        }
        if id.contains("com.apple.mobilesafari") {
            return "browser";
        }
        if id.contains("com.apple.pages") {
            return "document";
        }

        // Third-party apps (using contains for case-insensitive matching)
        if id.contains("readdle") || id.contains("pages") {
            return "document";
        }
        if id.contains("instagram") || id.contains("discord") {
            return "chat";
        }
        if id.contains("gmail") || id.contains("outlook") {
            return "email";
        }
        if id.contains("chrome") || id.contains("firefox") {
            return "browser";
        }

        "general"
    }
}

impl ContextDetector for IOSContextDetector {
    fn detect_current_context(&self) -> Result<ApplicationContext> {
        // On iOS, return the manually set context or default
        self.get_current_context()
    }
}

impl Default for IOSContextDetector {
    fn default() -> Self {
        Self::new()
    }
}

/// Initialize iOS platform-specific features
pub async fn init() -> Result<()> {
    Ok(())
}

/// Get the active application name
pub async fn get_active_application_name() -> Result<String> {
    Err(ContextError::PlatformNotSupported.into())
}

/// Check if accessibility permissions are granted
pub async fn check_accessibility_permission() -> Result<bool> {
    Ok(true)
}

/// Manual context selection result
#[derive(Debug, Clone)]
pub struct ManualContextSelection {
    pub application_name: String,
    pub application_category: String,
    pub custom_instructions: Option<String>,
}