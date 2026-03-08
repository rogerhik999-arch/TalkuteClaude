//! Global hotkey abstraction trait
//!
//! Provides a unified interface for global hotkey functionality across platforms.

use std::sync::Arc;

/// Hotkey event types
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HotkeyAction {
    Pressed,
    Released,
}

/// Hotkey event for streaming to Flutter
#[derive(Clone, Debug)]
pub struct HotkeyEvent {
    pub hotkey: String,
    pub action: HotkeyAction,
}

impl HotkeyAction {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Pressed => "pressed",
            Self::Released => "released",
        }
    }
}

/// Platform-agnostic hotkey trait
///
/// Implementations should handle:
/// - Global hotkey registration (works even when app is not focused)
/// - Press and release event detection (for push-to-talk)
/// - Hotkey conflict detection
/// - Platform-specific modifier key handling (Ctrl vs Cmd)
pub trait PlatformHotkey: Send + Sync {
    /// Register a global hotkey
    ///
    /// # Arguments
    /// * `hotkey` - Key combination string (e.g., "Ctrl+Shift+Space")
    ///
    /// # Returns
    /// * `Ok(())` if registered successfully
    /// * `Err("Hotkey conflict")` if another app has registered this key
    /// * `Err("Invalid hotkey")` if the hotkey string is malformed
    fn register(&mut self, hotkey: &str) -> Result<(), String>;

    /// Unregister the current hotkey
    fn unregister(&mut self) -> Result<(), String>;

    /// Check if a hotkey is currently registered
    fn is_registered(&self) -> bool;

    /// Get the current hotkey string
    fn current_hotkey(&self) -> Option<&str>;

    /// Set callback for hotkey press events
    fn on_press(&mut self, callback: Box<dyn Fn() + Send + Sync>);

    /// Set callback for hotkey release events
    fn on_release(&mut self, callback: Box<dyn Fn() + Send + Sync>);
}

/// Global hotkey manager singleton
pub static HOTKEY_MANAGER: once_cell::sync::Lazy<std::sync::Mutex<Box<dyn PlatformHotkey>>> =
    once_cell::sync::Lazy::new(|| {
        #[cfg(target_os = "windows")]
        {
            std::sync::Mutex::new(Box::new(crate::platform::windows::hotkey::WindowsHotkey::new()))
        }
        #[cfg(target_os = "macos")]
        {
            std::sync::Mutex::new(Box::new(crate::platform::macos::hotkey::MacOsHotkey::new()))
        }
        #[cfg(target_os = "linux")]
        {
            std::sync::Mutex::new(Box::new(crate::platform::linux::hotkey::LinuxHotkey::new()))
        }
        #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
        {
            std::sync::Mutex::new(Box::new(NoOpHotkey::new()))
        }
    });

/// No-op hotkey for unsupported platforms
struct NoOpHotkey {
    hotkey: Option<String>,
}

impl NoOpHotkey {
    fn new() -> Self { Self { hotkey: None } }
}

impl PlatformHotkey for NoOpHotkey {
    fn register(&mut self, hotkey: &str) -> Result<(), String> {
        self.hotkey = Some(hotkey.to_string());
        Ok(())
    }
    fn unregister(&mut self) -> Result<(), String> {
        self.hotkey = None;
        Ok(())
    }
    fn is_registered(&self) -> bool { self.hotkey.is_some() }
    fn current_hotkey(&self) -> Option<&str> { self.hotkey.as_deref() }
    fn on_press(&mut self, _callback: Box<dyn Fn() + Send + Sync>) {}
    fn on_release(&mut self, _callback: Box<dyn Fn() + Send + Sync>) {}
}