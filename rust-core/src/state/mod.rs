//! Session state management module
//!
//! Provides the session state machine for tracking the voice input flow.

mod session;
mod manager;

pub use session::{SessionState, TranscriptionSession};
pub use manager::SessionManager;