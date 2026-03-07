//! Linux floating window management

use crate::platform::window::{WindowManager, CapsuleState, WindowPosition, WindowSize};

/// Linux window manager for floating capsule
pub struct LinuxWindowManager {
    visible: bool,
    state: CapsuleState,
    position: Option<WindowPosition>,
    size: WindowSize,
}

impl LinuxWindowManager {
    pub fn new() -> Self {
        Self {
            visible: false,
            state: CapsuleState::Idle,
            position: None,
            size: WindowSize::default(),
        }
    }
}

impl WindowManager for LinuxWindowManager {
    fn show(&mut self) -> Result<(), String> {
        // TODO: Implement using GTK overlay window
        self.visible = true;
        Ok(())
    }

    fn hide(&mut self) -> Result<(), String> {
        self.visible = false;
        Ok(())
    }

    fn is_visible(&self) -> bool {
        self.visible
    }

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

    fn get_position(&self) -> Option<WindowPosition> {
        self.position
    }

    fn get_size(&self) -> WindowSize {
        self.size
    }

    fn center(&mut self) -> Result<(), String> {
        self.position = Some(WindowPosition { x: 0.0, y: 0.0 });
        Ok(())
    }
}