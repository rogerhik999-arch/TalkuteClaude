//! Windows context detection

use crate::error::{Result, ContextError};
use crate::storage::models::ApplicationContext;
use crate::context::detector::ContextDetector;
use chrono::Utc;
use uuid::Uuid;

/// Windows-specific context detector
pub struct WindowsContextDetector {
    available: bool,
}

impl WindowsContextDetector {
    /// Create a new Windows context detector
    pub fn new() -> Self {
        Self {
            available: cfg!(target_os = "windows"),
        }
    }

    /// Check if Windows context detection is available
    pub fn is_available(&self) -> bool {
        self.available
    }

    /// Map a process name to an application category
    pub fn map_to_category(&self, process_name: &str) -> &'static str {
        // First normalize the process name
        let name = self.normalize_process_name(process_name);

        match name.as_str() {
            // Email clients
            "outlook" | "thunderbird" | "mail" | "gmail" => "email",

            // Chat applications
            "slack" | "discord" | "telegram" | "whatsapp" | "wechat" | "teams" | "zoom" => "chat",

            // Code editors/IDEs
            "code" | "vscode" | "idea" | "pycharm" | "webstorm" | "clion" | "rider" |
            "goland" | "android studio" | "sublime_text" | "notepad++" | "vim" | "nvim" => "code",

            // Document editors
            "winword" | "word" | "excel" | "powerpnt" | "powerpoint" | "onenote" |
            "notion" | "obsidian" | "typora" => "document",

            // Browsers
            "chrome" | "firefox" | "msedge" | "edge" | "opera" | "brave" | "safari" => "browser",

            // Terminal
            "cmd" | "powershell" | "windowsterminal" | "conemu" | "alacritty" => "code",

            _ => "general",
        }
    }

    /// Normalize a process name (remove path and extension)
    pub fn normalize_process_name(&self, process_name: &str) -> String {
        let name = process_name.to_lowercase();

        // Remove path if present
        let name = if let Some(pos) = name.rfind('\\') {
            &name[pos + 1..]
        } else if let Some(pos) = name.rfind('/') {
            &name[pos + 1..]
        } else {
            &name
        };

        // Remove .exe extension if present
        let name = name.strip_suffix(".exe").unwrap_or(name);

        name.to_string()
    }
}

impl ContextDetector for WindowsContextDetector {
    /// Detect the current foreground application context
    fn detect_current_context(&self) -> Result<ApplicationContext> {
        if !self.available {
            return Err(ContextError::PlatformNotSupported.into());
        }

        // On Windows, we would use GetForegroundWindow() and GetWindowThreadProcessId()
        // For now, return a placeholder that indicates detection is needed
        #[cfg(target_os = "windows")]
        {
            // TODO: Implement actual Windows API call
            // use winapi::um::winuser::GetForegroundWindow;
        }

        // Placeholder for testing on non-Windows
        Ok(ApplicationContext::new(
            Uuid::new_v4().to_string(),
            "unknown".to_string(),
            "general".to_string(),
        ))
    }
}

impl Default for WindowsContextDetector {
    fn default() -> Self {
        Self::new()
    }
}