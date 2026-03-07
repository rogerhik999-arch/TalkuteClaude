//! Platform abstraction module
//!
//! This module provides cross-platform abstractions for:
//! - System tray integration
//! - Global hotkey registration
//! - Text injection at cursor position
//! - Floating window management

pub mod tray;
pub mod hotkey;
pub mod text_injection;
pub mod window;

#[cfg(target_os = "windows")]
pub mod windows;

#[cfg(target_os = "macos")]
pub mod macos;

#[cfg(target_os = "linux")]
pub mod linux;

pub use tray::PlatformTray;
pub use hotkey::PlatformHotkey;
pub use text_injection::TextInjector;
pub use window::WindowManager;