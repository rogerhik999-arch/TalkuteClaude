//! Linux context detection

use crate::error::{Result, ContextError};
use crate::storage::models::ApplicationContext;
use crate::context::detector::ContextDetector;
use uuid::Uuid;

/// Linux-specific context detector
pub struct LinuxContextDetector {
    available: bool,
}

impl LinuxContextDetector {
    /// Create a new Linux context detector
    pub fn new() -> Self {
        Self {
            available: cfg!(target_os = "linux"),
        }
    }

    /// Check if Linux context detection is available
    pub fn is_available(&self) -> bool {
        self.available
    }

    /// Map a process name to an application category
    pub fn map_to_category(&self, process_name: &str) -> &'static str {
        let name = process_name.to_lowercase();

        match name.as_str() {
            // Email clients
            "thunderbird" | "evolution" | "geary" | "mail" | "gmail" => "email",

            // Chat applications
            "slack" | "discord" | "telegram" | "whatsapp" | "signal" | "element" => "chat",

            // Code editors/IDEs
            "code" | "vscode" | "codium" | "idea" | "pycharm" | "goland" |
            "sublime_text" | "atom" | "brackets" | "vim" | "nvim" | "emacs" => "code",

            // Document editors
            "libreoffice-writer" | "libreoffice" | "onlyoffice" |
            "typora" | "obsidian" | "notion" => "document",

            // Browsers
            "chrome" | "chromium" | "firefox" | "brave" | "opera" | "vivaldi" | "edge" => "browser",

            // Terminal emulators (treat as code context)
            "gnome-terminal" | "konsole" | "alacritty" | "kitty" |
            "terminator" | "tilix" | "xterm" | "st" => "code",

            _ => "general",
        }
    }
}

impl ContextDetector for LinuxContextDetector {
    fn detect_current_context(&self) -> Result<ApplicationContext> {
        if !self.available {
            return Err(ContextError::PlatformNotSupported.into());
        }

        #[cfg(target_os = "linux")]
        {
            // TODO: Use xdotool or wlr-foreign-toplevel-management for Wayland
        }

        Ok(ApplicationContext::new(
            Uuid::new_v4().to_string(),
            "unknown".to_string(),
            "general".to_string(),
        ))
    }
}

impl Default for LinuxContextDetector {
    fn default() -> Self {
        Self::new()
    }
}

/// Get the active application name (for legacy compatibility)
pub async fn get_active_application_name() -> Result<String> {
    Ok("unknown".to_string())
}

/// Get the active window information
pub async fn get_active_window() -> Result<WindowInfo> {
    Err(ContextError::PlatformNotSupported.into())
}

/// Check if accessibility permissions are granted
pub async fn check_accessibility_permission() -> Result<bool> {
    Ok(true)
}

/// Initialize Linux platform-specific features
pub async fn init() -> Result<()> {
    Ok(())
}

/// Window information structure
#[derive(Debug, Clone)]
pub struct WindowInfo {
    pub id: u64,
    pub title: String,
    pub class: String,
    pub wm_class: Option<String>,
}