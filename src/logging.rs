use tracing::metadata::LevelFilter;
use tracing_subscriber::EnvFilter;

#[derive(Debug, Copy, Clone)]
pub enum LoggingStyle {
    /// Uses compact logging.
    Compact,
    /// Uses JSON formatted logging
    Json,
}

/// Initializes the tracing and logging system.
///
/// This method uses the default environment filter to configure logging.
/// Please use the `RUST_LOG` environment variable to tune.
pub fn initialize(style: LoggingStyle) {
    let filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::INFO.into())
        .from_env_lossy();

    let formatter = tracing_subscriber::fmt()
        .with_file(false)
        .with_line_number(false)
        .with_thread_ids(true)
        .with_target(true)
        .with_env_filter(filter);

    match style {
        LoggingStyle::Compact => formatter.init(),
        LoggingStyle::Json => formatter.json().init(),
    }
}
