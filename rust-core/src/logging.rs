//! Logging infrastructure for the Talkute core library

pub use log::{debug, error, info, trace, warn, LevelFilter};
use std::env;

/// Initialize the logging system with configurable level
///
/// # Arguments
///
/// * `level` - Optional logging level. If None, uses RUST_LOG env var or defaults to Info
pub fn init_logging(level: Option<LevelFilter>) {
    let env_level = env::var("RUST_LOG").unwrap_or_else(|_| {
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

    info!("Logging initialized with RUST_LOG={}", env_level);
}
