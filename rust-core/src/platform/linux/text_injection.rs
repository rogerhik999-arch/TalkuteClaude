//! Linux text injection implementation

use crate::platform::text_injection::{TextInjector, InjectionResult};

/// Linux text injector using XTest extension
pub struct LinuxTextInjector;

impl LinuxTextInjector {
    pub fn new() -> Self {
        Self
    }
}

impl TextInjector for LinuxTextInjector {
    fn inject(&self, text: &str) -> InjectionResult {
        // TODO: Implement using XTest extension
        match self.copy_to_clipboard(text) {
            Ok(()) => InjectionResult::ClipboardFallback,
            Err(e) => InjectionResult::Failed(e),
        }
    }

    fn copy_to_clipboard(&self, _text: &str) -> Result<(), String> {
        // TODO: Implement using X11 clipboard
        Ok(())
    }

    fn can_inject(&self) -> bool {
        true
    }

    fn get_focused_app(&self) -> Option<String> {
        // TODO: Implement using XGetInputFocus
        None
    }
}