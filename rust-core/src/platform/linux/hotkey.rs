//! Linux global hotkey implementation

use crate::platform::hotkey::PlatformHotkey;
use std::sync::Arc;

/// Linux hotkey implementation using X11 XGrabKey
pub struct LinuxHotkey {
    hotkey: Option<String>,
    registered: bool,
    on_press: Option<Arc<dyn Fn() + Send + Sync>>,
    on_release: Option<Arc<dyn Fn() + Send + Sync>>,
}

impl LinuxHotkey {
    pub fn new() -> Self {
        Self {
            hotkey: None,
            registered: false,
            on_press: None,
            on_release: None,
        }
    }
}

impl PlatformHotkey for LinuxHotkey {
    fn register(&mut self, hotkey: &str) -> Result<(), String> {
        self.hotkey = Some(hotkey.to_string());
        self.registered = true;
        Ok(())
    }

    fn unregister(&mut self) -> Result<(), String> {
        self.hotkey = None;
        self.registered = false;
        Ok(())
    }

    fn is_registered(&self) -> bool {
        self.registered
    }

    fn current_hotkey(&self) -> Option<&str> {
        self.hotkey.as_deref()
    }

    fn on_press(&mut self, callback: Box<dyn Fn() + Send + Sync>) {
        self.on_press = Some(Arc::from(callback));
    }

    fn on_release(&mut self, callback: Box<dyn Fn() + Send + Sync>) {
        self.on_release = Some(Arc::from(callback));
    }
}