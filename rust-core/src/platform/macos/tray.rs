//! macOS system tray implementation

use crate::platform::tray::{PlatformTray, TrayIconState, TrayMenuItem};

/// macOS tray implementation using AppKit
pub struct MacOsTray {
    initialized: bool,
    current_state: TrayIconState,
}

impl MacOsTray {
    pub fn new() -> Self {
        Self {
            initialized: false,
            current_state: TrayIconState::Idle,
        }
    }
}

impl PlatformTray for MacOsTray {
    fn initialize(&mut self) -> Result<(), String> {
        self.initialized = true;
        Ok(())
    }

    fn set_icon(&mut self, state: TrayIconState) -> Result<(), String> {
        if !self.initialized {
            return Err("Tray not initialized".to_string());
        }
        self.current_state = state;
        Ok(())
    }

    fn set_menu(&mut self, _items: Vec<TrayMenuItem>) -> Result<(), String> {
        if !self.initialized {
            return Err("Tray not initialized".to_string());
        }
        Ok(())
    }

    fn update_menu_item(&mut self, _id: &str, _new_label: &str) -> Result<(), String> {
        Ok(())
    }

    fn show_notification(&self, _title: &str, _message: &str) -> Result<(), String> {
        Ok(())
    }

    fn cleanup(&mut self) -> Result<(), String> {
        self.initialized = false;
        Ok(())
    }
}