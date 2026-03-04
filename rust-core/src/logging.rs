//! Logging infrastructure for the Talkute core library

use log::{info, warn, error, debug, LevelFilter};
use std::env;

/// Initialize the logging system with configurable level
///
/// # Arguments
///
/// * `level` - Optional logging level. If None, uses RUST_LOG env var or defaults to Info
pub fn init_logging(level: Option<LevelFilter>) {
    let env = env::var("RUST_LOG").unwrap_or_else(|_| {
        let lvl = level.unwrap_or(LevelFilter::Info);
        match lvl {
            LevelFilter::Off => "off",
            LevelFilter::Error => "error",
            LevelFilter::Warn => "warn",
            LevelFilter::Info => "info",
            LevelFilter::Debug => "debug",
            LevelFilter::Trace => "trace",
        }
        .to_string()
    });

    env_logger::builder()
        .filter_level(level.unwrap_or(LevelFilter::Info))
        .format_timestamp(None)
        .format_module_path(false)
        .format_target(false)
        .init();

    info!("Logging initialized with RUST_LOG={}", env);
}

/// Log an info message
#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        log::info!($($arg)*)
    };
}

/// Log a warning message
#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {
        log::warn!($($arg)*)
    };
}

/// Log an error message
#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        log::error!($($arg)*)
    };
}

/// Log a debug message
#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {
        log::debug!($($arg)*)
    };
}

/// Log a trace message
#[macro_export]
macro_rules! trace {
    ($($arg:tt)*) => {
        log::trace!($($arg)*)
    };
}
