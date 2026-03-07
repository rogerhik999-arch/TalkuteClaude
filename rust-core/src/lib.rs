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
//! - `platform`: Cross-platform abstractions (tray, hotkey, text injection, window)
//! - `state`: Session state management
//! - `quota`: Usage quota tracking
//! - `storage`: Local SQLite database
//! - `error`: Error types and handling

// Async runtime with tokio
pub use tokio;
use tokio::runtime::Builder;

// Logging infrastructure
pub use env_logger;
pub use log;
pub mod logging;
pub use logging::{init_logging, info, warn, error, debug, trace, LevelFilter};

// Public API
pub mod ai;
pub mod context;
pub mod error;
pub mod ffi;
pub mod network;
pub mod platform;
pub mod processing;
pub mod quota;
pub mod speech;
pub mod state;
pub mod storage;
pub mod tools;

// Re-export common types
pub use error::{Error, Result};

/// Get the global async runtime
///
/// This creates a multi-threaded runtime optimized for I/O-bound operations.
/// Use for async FFI functions that need to spawn tasks.
pub fn get_runtime() -> &'static tokio::runtime::Runtime {
    static_runtime::get()
}

/// Static runtime initialization using once_cell
mod static_runtime {
    use super::Builder;
    use once_cell::sync::Lazy;
    use tokio::runtime::Runtime;

    pub static RUNTIME: Lazy<Runtime> = Lazy::new(|| {
        Builder::new_multi_thread()
            .worker_threads(4)
            .thread_name("talkute-worker")
            .enable_all()
            .build()
            .expect("Failed to create tokio runtime")
    });

    pub fn get() -> &'static Runtime {
        &RUNTIME
    }
}

