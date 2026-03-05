//! Android context detection

use crate::error::{Result, ContextError};
use crate::storage::models::ApplicationContext;
use crate::context::detector::ContextDetector;
use uuid::Uuid;

/// Android-specific context detector
pub struct AndroidContextDetector {
    available: bool,
}

impl AndroidContextDetector {
    /// Create a new Android context detector
    pub fn new() -> Self {
        Self {
            available: cfg!(target_os = "android"),
        }
    }

    /// Check if Android context detection is available
    pub fn is_available(&self) -> bool {
        self.available
    }

    /// Map an Android package name to an application category
    pub fn map_to_category(&self, package_name: &str) -> &'static str {
        let pkg = package_name.to_lowercase();

        // Email clients
        if pkg.contains("gmail") || pkg.contains("outlook") || pkg.contains("mail") {
            return "email";
        }
        match pkg.as_str() {
            "com.google.android.gm" | "com.microsoft.outlook" |
            "com.fsck.k9" | "org.thoughtcrime.securesms" => "email",

            // Chat applications
            "com.slack" | "com.discord" | "com.telegram.messenger" |
            "com.whatsapp" | "com.facebook.orca" | "com.snapchat.android" => "chat",

            // Code editors
            "com.ghisler.android.totalcommander" | "com.aide.ui" |
            "com.rhmsoft.code.editor" | "org.kore.kore" => "code",

            // Browsers
            "com.android.chrome" | "org.mozilla.firefox" |
            "com.brave.browser" | "com.opera.browser" => "browser",

            // Document editors
            "com.microsoft.office.word" | "com.microsoft.office.excel" |
            "com.google.android.apps.docs.editors.docs" |
            "com.google.android.apps.docs.editors.sheets" => "document",

            _ => "general",
        }
    }
}

impl ContextDetector for AndroidContextDetector {
    fn detect_current_context(&self) -> Result<ApplicationContext> {
        if !self.available {
            return Err(ContextError::PlatformNotSupported.into());
        }

        #[cfg(target_os = "android")]
        {
            // TODO: Use UsageStatsManager or AccessibilityService
        }

        Ok(ApplicationContext::new(
            Uuid::new_v4().to_string(),
            "unknown".to_string(),
            "general".to_string(),
        ))
    }
}

impl Default for AndroidContextDetector {
    fn default() -> Self {
        Self::new()
    }
}

/// Initialize Android platform-specific features
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

/// Request accessibility permissions from the user
pub async fn request_accessibility_permission() -> Result<bool> {
    Ok(false)
}