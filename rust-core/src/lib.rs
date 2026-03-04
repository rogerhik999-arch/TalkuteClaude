//! Talkute AI Voice Input Assistant - Core Library
//!
//! This library implements the core logic for the AI voice input assistant,
//! including speech recognition, text processing, context detection, and AI integration.
//!
//! # Architecture
//!
//! The library is organized into the following modules:
//!
//! - `ffi`: Flutter bridge definitions
//! - `speech`: Speech recognition and audio capture
//! - `processing`: Text processing (filler removal, self-correction, formatting)
//! - `ai`: AI integration (prompts, polisher, client)
//! - `context`: Platform-specific context detection
//! - `quota`: Usage quota tracking
//! - `storage`: Local SQLite database
//! - `error`: Error types and handling

// Async runtime
pub use tokio;

// Logging infrastructure
pub use env_logger;
pub use log;

// Public API
pub mod ai;
pub mod context;
pub mod error;
pub mod ffi;
pub mod processing;
pub mod quota;
pub mod speech;
pub mod storage;

// Re-export common types
pub use error::{Error, Result};

// Initialize logging when the library is loaded
#[Initializer]
pub fn init_logging() {
    env_logger::init();
}
