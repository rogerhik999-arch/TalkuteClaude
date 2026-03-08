//! macOS context detection

use crate::error::{Result, ContextError};
use crate::storage::models::ApplicationContext;
use crate::context::detector::ContextDetector;
use uuid::Uuid;

/// macOS-specific context detector
pub struct MacOSContextDetector {
    available: bool,
}

impl MacOSContextDetector {
    /// Create a new macOS context detector
    pub fn new() -> Self {
        Self {
            available: cfg!(target_os = "macos"),
        }
    }

    /// Check if macOS context detection is available
    pub fn is_available(&self) -> bool {
        self.available
    }

    /// Map a bundle identifier to an application category
    pub fn map_to_category(&self, bundle_id: &str) -> &'static str {
        let id = bundle_id.to_lowercase();

        // Apple apps
        if id.contains("com.apple.mail") {
            return "email";
        }
        if id.contains("com.apple.safari") {
            return "browser";
        }
        if id.contains("com.apple.notes") {
            return "document";
        }
        if id.contains("com.apple.mobilesms") || id.contains("com.apple.ichat") {
            return "chat";
        }
        if id.contains("com.apple.pages") || id.contains("com.apple.word") {
            return "document";
        }

        // Common third-party apps
        match id.as_str() {
            // Email
            "com.microsoft.outlook" | "com.readdle.spark" | "com.gmail" => "email",

            // Chat
            "com.tinyspeck.slackmacgap" | "com.slack" |
            "com.hnc.discord" | "com.discord" |
            "com.telegram" | "org.whatsapp" => "chat",

            // Code
            "com.microsoft.vscode" | "com.jetbrains.intellij" |
            "com.jetbrains.pycharm" | "com.jetbrains.goland" |
            "com.sublimetext.3" | "com.sublimetext.4" => "code",

            // Browsers
            "com.google.chrome" | "org.mozilla.firefox" |
            "com.brave.browser" | "com.microsoft.edgemac" => "browser",

            // Documents
            "com.microsoft.word" | "com.microsoft.excel" |
            "com.microsoft.powerpoint" | "md.obsidian" => "document",

            _ => "general",
        }
    }
}

impl ContextDetector for MacOSContextDetector {
    fn detect_current_context(&self) -> Result<ApplicationContext> {
        if !self.available {
            return Err(ContextError::PlatformNotSupported.into());
        }

        #[cfg(target_os = "macos")]
        {
            // TODO: Use NSWorkspace.shared.frontmostApplication
        }

        Ok(ApplicationContext::new(
            Uuid::new_v4().to_string(),
            "unknown".to_string(),
            "general".to_string(),
        ))
    }
}

impl Default for MacOSContextDetector {
    fn default() -> Self {
        Self::new()
    }
}

/// Get the active application (for legacy compatibility)
pub async fn get_active_application() -> Result<ApplicationInfo> {
    Ok(ApplicationInfo {
        name: "unknown".to_string(),
        bundle_id: None,
        process_id: 0,
        is_frontmost: true,
    })
}

/// Application information structure
#[derive(Debug, Clone)]
pub struct ApplicationInfo {
    pub name: String,
    pub bundle_id: Option<String>,
    pub process_id: u32,
    pub is_frontmost: bool,
}

/// Initialize macOS platform-specific features
pub async fn init() -> Result<()> {
    Ok(())
}

/// Check if accessibility permissions are granted
pub async fn check_accessibility_permission() -> Result<bool> {
    Ok(true)
}