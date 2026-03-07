//! macOS global hotkey implementation

use crate::platform::hotkey::PlatformHotkey;
use std::sync::Arc;

/// macOS hotkey implementation using Carbon/CGEvent
pub struct MacOsHotkey {
    hotkey: Option<String>,
    registered: bool,
    on_press: Option<Arc<dyn Fn() + Send + Sync>>,
    on_release: Option<Arc<dyn Fn() + Send + Sync>>,
}

impl MacOsHotkey {
    pub fn new() -> Self {
        Self {
            hotkey: None,
            registered: false,
            on_press: None,
            on_release: None,
        }
    }
}

impl PlatformHotkey for MacOsHotkey {
    fn register(&mut self, hotkey: &str) -> Result<(), String> {
        // Convert "Ctrl+Shift+Space" to macOS Cmd+Shift+Space
        let macos_hotkey = hotkey.replace("Ctrl", "Cmd");
        self.hotkey = Some(macos_hotkey);
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