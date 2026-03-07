//! System tray abstraction trait
//!
//! Provides a unified interface for system tray functionality across platforms.

use std::sync::Arc;

/// Menu item for the system tray context menu
#[derive(Clone, Debug)]
pub struct TrayMenuItem {
    pub id: String,
    pub label: String,
    pub enabled: bool,
    pub is_separator: bool,
}

/// Tray icon states matching session states
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TrayIconState {
    Idle,
    Recording,
    Processing,
    Error,
}

impl TrayIconState {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Idle => "idle",
            Self::Recording => "recording",
            Self::Processing => "processing",
            Self::Error => "error",
        }
    }
}

/// Platform-agnostic tray trait
///
/// Implementations should handle:
/// - Icon changes based on application state
/// - Context menu with configurable items
/// - Platform-specific initialization and cleanup
pub trait PlatformTray: Send + Sync {
    /// Initialize the system tray with default icon
    fn initialize(&mut self) -> Result<(), String>;

    /// Set the tray icon state
    fn set_icon(&mut self, state: TrayIconState) -> Result<(), String>;

    /// Set the tray context menu items
    fn set_menu(&mut self, items: Vec<TrayMenuItem>) -> Result<(), String>;

    /// Update a menu item's label (for quota display, etc.)
    fn update_menu_item(&mut self, id: &str, new_label: &str) -> Result<(), String>;

    /// Show a notification from the tray
    fn show_notification(&self, title: &str, message: &str) -> Result<(), String>;

    /// Cleanup and remove the tray icon
    fn cleanup(&mut self) -> Result<(), String>;
}

/// Global tray manager singleton
pub static TRAY_MANAGER: once_cell::sync::Lazy<std::sync::Mutex<Box<dyn PlatformTray>>> =
    once_cell::sync::Lazy::new(|| {
        #[cfg(target_os = "windows")]
        {
            std::sync::Mutex::new(Box::new(crate::platform::windows::tray::WindowsTray::new()))
        }
        #[cfg(target_os = "macos")]
        {
            std::sync::Mutex::new(Box::new(crate::platform::macos::tray::MacOsTray::new()))
        }
        #[cfg(target_os = "linux")]
        {
            std::sync::Mutex::new(Box::new(crate::platform::linux::tray::LinuxTray::new()))
        }
        #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
        {
            std::sync::Mutex::new(Box::new(NoOpTray::new()))
        }
    });

/// No-op tray for unsupported platforms
struct NoOpTray;

impl NoOpTray {
    fn new() -> Self { Self }
}

impl PlatformTray for NoOpTray {
    fn initialize(&mut self) -> Result<(), String> { Ok(()) }
    fn set_icon(&mut self, _state: TrayIconState) -> Result<(), String> { Ok(()) }
    fn set_menu(&mut self, _items: Vec<TrayMenuItem>) -> Result<(), String> { Ok(()) }
    fn update_menu_item(&mut self, _id: &str, _new_label: &str) -> Result<(), String> { Ok(()) }
    fn show_notification(&self, _title: &str, _message: &str) -> Result<(), String> { Ok(()) }
    fn cleanup(&mut self) -> Result<(), String> { Ok(()) }
}