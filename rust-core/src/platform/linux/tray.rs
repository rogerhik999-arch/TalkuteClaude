//! Linux system tray implementation

use crate::platform::tray::{PlatformTray, TrayIconState, TrayMenuItem};

/// Linux tray implementation using AppIndicator/GTK
pub struct LinuxTray {
    initialized: bool,
    current_state: TrayIconState,
}

impl LinuxTray {
    pub fn new() -> Self {
        Self {
            initialized: false,
            current_state: TrayIconState::Idle,
        }
    }
}

impl PlatformTray for LinuxTray {
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