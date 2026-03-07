//! macOS text injection implementation

use crate::platform::text_injection::{TextInjector, InjectionResult};

/// macOS text injector using CGEvent keyboard simulation
pub struct MacOsTextInjector;

impl MacOsTextInjector {
    pub fn new() -> Self {
        Self
    }
}

impl TextInjector for MacOsTextInjector {
    fn inject(&self, text: &str) -> InjectionResult {
        // TODO: Implement using CGEvent keyboard simulation
        match self.copy_to_clipboard(text) {
            Ok(()) => InjectionResult::ClipboardFallback,
            Err(e) => InjectionResult::Failed(e),
        }
    }

    fn copy_to_clipboard(&self, _text: &str) -> Result<(), String> {
        // TODO: Implement using NSPasteboard
        Ok(())
    }

    fn can_inject(&self) -> bool {
        // TODO: Check accessibility permissions
        true
    }

    fn get_focused_app(&self) -> Option<String> {
        // TODO: Implement using NSWorkspace
        None
    }
}