//! Unified context detector

use crate::context::windows::{self as windows_ctx};
use crate::context::macos::{self as macos_ctx};
use crate::context::linux::{self as linux_ctx};
use crate::context::ios::{self as ios_ctx};
use crate::context::android::{self as android_ctx};
use crate::error::{Result, ContextError};
use crate::storage::models::ApplicationContext;

/// Unified context detector that works across all platforms
pub struct ContextDetector {
    platform: Platform,
}

/// Supported platforms
#[derive(Debug, Clone, PartialEq)]
pub enum Platform {
    Windows,
    Macos,
    Linux,
    Ios,
    Android,
}

impl ContextDetector {
    /// Create a new context detector
    pub async fn new() -> Result<Self> {
        let platform = Self::detect_platform();
        Ok(Self { platform })
    }

    /// Detect the current platform
    fn detect_platform() -> Platform {
        if cfg!(target_os = "windows") {
            Platform::Windows
        } else if cfg!(target_os = "macos") {
            Platform::Macos
        } else if cfg!(target_os = "linux") {
            Platform::Linux
        } else if cfg!(target_os = "ios") {
            Platform::Ios
        } else if cfg!(target_os = "android") {
            Platform::Android
        } else {
            Platform::Linux // Default to Linux for unknown platforms
        }
    }

    /// Detect the current application context
    pub async fn detect(&self) -> Result<ApplicationContext> {
        match self.platform {
            Platform::Windows => self.detect_windows().await,
            Platform::Macos => self.detect_macos().await,
            Platform::Linux => self.detect_linux().await,
            Platform::Ios => self.detect_ios().await,
            Platform::Android => self.detect_android().await,
        }
    }

    async fn detect_windows(&self) -> Result<ApplicationContext> {
        windows_ctx::get_active_application_name().await.map(|name| {
            ApplicationContext::new(name, self.categorize_application(&name))
        })
    }

    async fn detect_macos(&self) -> Result<ApplicationContext> {
        macos_ctx::get_active_application().await.map(|app| {
            ApplicationContext::new(app.name, self.categorize_application(&app.name))
        })
    }

    async fn detect_linux(&self) -> Result<ApplicationContext> {
        linux_ctx::get_active_application_name().await.map(|name| {
            ApplicationContext::new(name, self.categorize_application(&name))
        })
    }

    async fn detect_ios(&self) -> Result<ApplicationContext> {
        // On iOS, direct detection is not possible
        // This should trigger manual selection in the Flutter app
        // For now, return a placeholder with manual category
        Ok(ApplicationContext::new(
            "manual-selection".to_string(),
            "other".to_string(),
        ))
    }

    async fn detect_android(&self) -> Result<ApplicationContext> {
        android_ctx::get_active_application_name().await.map(|name| {
            ApplicationContext::new(name, self.categorize_application(&name))
        })
    }

    /// Categorize an application based on its name
    fn categorize_application(&self, name: &str) -> String {
        let name_lower = name.to_lowercase();

        if name_lower.contains("gmail") || name_lower.contains("outlook") || name_lower.contains("mail") {
            "email".to_string()
        } else if name_lower.contains("slack") || name_lower.contains("discord") || name_lower.contains("telegram") || name_lower.contains("whatsapp") || name_lower.contains("wechat") {
            "chat".to_string()
        } else if name_lower.contains("word") || name_lower.contains("docs") || name_lower.contains("notion") || name_lower.contains("typora") {
            "document".to_string()
        } else if name_lower.contains("vscode") || name_lower.contains("idea") || name_lower.contains("vim") || name_lower.contains("emacs") || name_lower.contains("editor") || name_lower.contains("code") {
            "code".to_string()
        } else if name_lower.contains("browser") || name_lower.contains("chrome") || name_lower.contains("firefox") || name_lower.contains("safari") || name_lower.contains("edge") || name_lower.contains("opera") {
            "browser".to_string()
        } else {
            "other".to_string()
        }
    }
}
