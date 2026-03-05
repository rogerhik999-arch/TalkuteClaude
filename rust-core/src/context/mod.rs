// Context module
pub mod android;
pub mod detector;
pub mod ios;
pub mod linux;
pub mod macos;
pub mod registry;
pub mod windows;

pub use registry::{ApplicationRegistry, AppCategory};
