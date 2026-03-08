//! Floating window management abstraction trait
//!
//! Provides a unified interface for floating capsule window across platforms.

/// Visual state of the floating capsule
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CapsuleState {
    Idle,
    Recording,
    Processing,
    Success,
    Error,
}

impl CapsuleState {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Idle => "idle",
            Self::Recording => "recording",
            Self::Processing => "processing",
            Self::Success => "success",
            Self::Error => "error",
        }
    }
}

/// Window position for the floating capsule
#[derive(Clone, Copy, Debug)]
pub struct WindowPosition {
    pub x: f64,
    pub y: f64,
}

/// Window size for the floating capsule
#[derive(Clone, Copy, Debug)]
pub struct WindowSize {
    pub width: f64,
    pub height: f64,
}

impl Default for WindowSize {
    fn default() -> Self {
        Self {
            width: 200.0,
            height: 64.0,
        }
    }
}

/// Platform-agnostic window manager trait
///
/// Implementations should handle:
/// - Creating a frameless, always-on-top window
/// - Platform-specific visual effects (acrylic, vibrancy)
/// - Window positioning and sizing
/// - Show/hide animations
pub trait WindowManager: Send + Sync {
    /// Show the floating capsule window
    ///
    /// Should center the window on screen by default
    fn show(&mut self) -> Result<(), String>;

    /// Hide the floating capsule window
    fn hide(&mut self) -> Result<(), String>;

    /// Check if the window is currently visible
    fn is_visible(&self) -> bool;

    /// Set the visual state of the capsule
    fn set_state(&mut self, state: CapsuleState) -> Result<(), String>;

    /// Set window position
    fn set_position(&mut self, position: WindowPosition) -> Result<(), String>;

    /// Set window size
    fn set_size(&mut self, size: WindowSize) -> Result<(), String>;

    /// Get current window position
    fn get_position(&self) -> Option<WindowPosition>;

    /// Get current window size
    fn get_size(&self) -> WindowSize;

    /// Center the window on screen
    fn center(&mut self) -> Result<(), String>;
}

/// Global window manager singleton
pub static WINDOW_MANAGER: once_cell::sync::Lazy<std::sync::Mutex<Box<dyn WindowManager>>> =
    once_cell::sync::Lazy::new(|| {
        #[cfg(target_os = "windows")]
        {
            std::sync::Mutex::new(Box::new(crate::platform::windows::window::WindowsWindowManager::new()))
        }
        #[cfg(target_os = "macos")]
        {
            std::sync::Mutex::new(Box::new(crate::platform::macos::window::MacOsWindowManager::new()))
        }
        #[cfg(target_os = "linux")]
        {
            std::sync::Mutex::new(Box::new(crate::platform::linux::window::LinuxWindowManager::new()))
        }
        #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
        {
            std::sync::Mutex::new(Box::new(NoOpWindowManager::new()))
        }
    });

/// No-op window manager for unsupported platforms
struct NoOpWindowManager {
    visible: bool,
    state: CapsuleState,
    position: Option<WindowPosition>,
    size: WindowSize,
}

impl NoOpWindowManager {
    fn new() -> Self {
        Self {
            visible: false,
            state: CapsuleState::Idle,
            position: None,
            size: WindowSize::default(),
        }
    }
}

impl WindowManager for NoOpWindowManager {
    fn show(&mut self) -> Result<(), String> {
        self.visible = true;
        Ok(())
    }
    fn hide(&mut self) -> Result<(), String> {
        self.visible = false;
        Ok(())
    }
    fn is_visible(&self) -> bool { self.visible }
    fn set_state(&mut self, state: CapsuleState) -> Result<(), String> {
        self.state = state;
        Ok(())
    }
    fn set_position(&mut self, position: WindowPosition) -> Result<(), String> {
        self.position = Some(position);
        Ok(())
    }
    fn set_size(&mut self, size: WindowSize) -> Result<(), String> {
        self.size = size;
        Ok(())
    }
    fn get_position(&self) -> Option<WindowPosition> { self.position }
    fn get_size(&self) -> WindowSize { self.size }
    fn center(&mut self) -> Result<(), String> {
        self.position = Some(WindowPosition { x: 0.0, y: 0.0 });
        Ok(())
    }
}