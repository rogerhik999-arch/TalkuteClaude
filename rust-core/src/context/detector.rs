//! Unified context detector

use crate::context::windows::WindowsContextDetector;
use crate::context::macos::MacOSContextDetector;
use crate::context::linux::LinuxContextDetector;
use crate::context::ios::IOSContextDetector;
use crate::context::android::AndroidContextDetector;
use crate::error::{Result, ContextError};
use crate::storage::models::ApplicationContext;
use uuid::Uuid;

/// Trait for platform-specific context detectors
pub trait ContextDetector {
    /// Detect the current application context
    fn detect_current_context(&self) -> Result<ApplicationContext>;
}

/// Unified context detector that works across all platforms
pub struct UnifiedContextDetector {
    platform: Platform,
    windows_detector: WindowsContextDetector,
    macos_detector: MacOSContextDetector,
    linux_detector: LinuxContextDetector,
    ios_detector: IOSContextDetector,
    android_detector: AndroidContextDetector,
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

impl UnifiedContextDetector {
    /// Create a new context detector
    pub async fn new() -> Result<Self> {
        let platform = Self::detect_platform();
        Ok(Self {
            platform,
            windows_detector: WindowsContextDetector::new(),
            macos_detector: MacOSContextDetector::new(),
            linux_detector: LinuxContextDetector::new(),
            ios_detector: IOSContextDetector::new(),
            android_detector: AndroidContextDetector::new(),
        })
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

    /// Get the current platform
    pub fn platform(&self) -> &Platform {
        &self.platform
    }

    /// Detect the current application context
    pub async fn detect(&self) -> Result<ApplicationContext> {
        match self.platform {
            Platform::Windows => self.windows_detector.detect_current_context(),
            Platform::Macos => self.macos_detector.detect_current_context(),
            Platform::Linux => self.linux_detector.detect_current_context(),
            Platform::Ios => self.ios_detector.detect_current_context(),
            Platform::Android => self.android_detector.detect_current_context(),
        }
    }

    /// Categorize an application based on its name
    pub fn categorize_application(&self, name: &str) -> String {
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

    /// Get the platform-specific detector
    pub fn get_windows_detector(&self) -> &WindowsContextDetector {
        &self.windows_detector
    }

    /// Get the macOS detector
    pub fn get_macos_detector(&self) -> &MacOSContextDetector {
        &self.macos_detector
    }

    /// Get the Linux detector
    pub fn get_linux_detector(&self) -> &LinuxContextDetector {
        &self.linux_detector
    }

    /// Get the iOS detector
    pub fn get_ios_detector(&self) -> &IOSContextDetector {
        &self.ios_detector
    }

    /// Get the Android detector
    pub fn get_android_detector(&self) -> &AndroidContextDetector {
        &self.android_detector
    }
}

// Re-export the old ContextDetector as UnifiedContextDetector alias
pub type ContextDetectorService = UnifiedContextDetector;