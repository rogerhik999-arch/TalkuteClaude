//! Text injection abstraction trait
//!
//! Provides a unified interface for injecting text at the cursor position across platforms.

/// Result of a text injection attempt
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum InjectionResult {
    /// Text was successfully injected at cursor
    Injected,
    /// Text was copied to clipboard (fallback used)
    ClipboardFallback,
    /// Injection failed entirely
    Failed(String),
}

/// Platform-agnostic text injector trait
///
/// Implementations should handle:
/// - Simulating keyboard input to type text at cursor
/// - Clipboard fallback when keyboard simulation fails
/// - Platform-specific text encoding and timing
/// - Handling special characters and Unicode
pub trait TextInjector: Send + Sync {
    /// Inject text at the current cursor position
    ///
    /// # Arguments
    /// * `text` - The text to inject
    ///
    /// # Returns
    /// * `InjectionResult::Injected` on success
    /// * `InjectionResult::ClipboardFallback` if keyboard injection failed but clipboard worked
    /// * `InjectionResult::Failed` with error message if both methods failed
    fn inject(&self, text: &str) -> InjectionResult;

    /// Copy text to system clipboard
    ///
    /// Used as fallback when direct injection fails
    fn copy_to_clipboard(&self, text: &str) -> Result<(), String>;

    /// Check if text injection is likely to work
    ///
    /// Returns false if:
    /// - No accessibility permissions (macOS)
    /// - No active text field detected
    fn can_inject(&self) -> bool;

    /// Get the name of the currently focused application
    ///
    /// Used for context detection and logging
    fn get_focused_app(&self) -> Option<String>;
}

/// Global text injector singleton
pub static TEXT_INJECTOR: once_cell::sync::Lazy<Box<dyn TextInjector>> =
    once_cell::sync::Lazy::new(|| {
        #[cfg(target_os = "windows")]
        {
            Box::new(crate::platform::windows::text_injection::WindowsTextInjector::new())
        }
        #[cfg(target_os = "macos")]
        {
            Box::new(crate::platform::macos::text_injection::MacOsTextInjector::new())
        }
        #[cfg(target_os = "linux")]
        {
            Box::new(crate::platform::linux::text_injection::LinuxTextInjector::new())
        }
        #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
        {
            Box::new(NoOpTextInjector::new())
        }
    });

/// No-op text injector for unsupported platforms
struct NoOpTextInjector;

impl NoOpTextInjector {
    fn new() -> Self { Self }
}

impl TextInjector for NoOpTextInjector {
    fn inject(&self, _text: &str) -> InjectionResult {
        InjectionResult::Failed("Platform not supported".to_string())
    }
    fn copy_to_clipboard(&self, _text: &str) -> Result<(), String> {
        Err("Platform not supported".to_string())
    }
    fn can_inject(&self) -> bool { false }
    fn get_focused_app(&self) -> Option<String> { None }
}