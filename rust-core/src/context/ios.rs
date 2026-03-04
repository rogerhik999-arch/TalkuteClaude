//! Platform-specific initialization for iOS
//!
//! This module handles iOS-specific setup including:
//! - Accessibility API usage
//! - Input method integration
//! - Manual context selection (fallback)

use crate::error::{Result, ContextError};

/// Initialize iOS platform-specific features
pub async fn init() -> Result<()> {
    // TODO: Implement iOS-specific initialization
    // - Initialize UIKit accessibility APIs
    Ok(())
}

/// Get the active application name
///
/// On iOS, direct application context detection is restricted.
/// Users must manually select their target application.
pub async fn get_active_application_name() -> Result<String> {
    // iOS doesn't allow automatic application detection
    // This should trigger manual selection UI in the Flutter app
    Err(ContextError::PlatformNotSupported.into())
}

/// Check if accessibility permissions are granted
pub async fn check_accessibility_permission() -> Result<bool> {
    // TODO: Check iOS accessibility permissions
    // Use UIAccessibility.isVoiceOverRunning()
    Ok(true)
}

/// Manual context selection result
#[derive(Debug, Clone)]
pub struct ManualContextSelection {
    pub application_name: String,
    pub application_category: String,
    pub custom_instructions: Option<String>,
}
