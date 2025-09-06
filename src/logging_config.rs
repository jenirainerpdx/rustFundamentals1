//! Logging configuration module
//!
//! This module provides utilities for configuring logging behavior.
//! It supports setting log levels and directing log output to various targets
//! such as stdout, stderr, or a file.

/// Log levels supported by the logger.
/// # Examples
/// ```
/// use rust_fundamentals1::logging_config::LogLevel;
/// let level = LogLevel::Info;
/// assert_eq!(level.as_str(), "INFO");
/// ```
pub enum LogLevel {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

impl LogLevel {
    pub fn as_str(&self) -> &str {
        match self {
            LogLevel::Error => "ERROR",
            LogLevel::Warn => "WARN",
            LogLevel::Info => "INFO",
            LogLevel::Debug => "DEBUG",
            LogLevel::Trace => "TRACE",
        }
    }
}

/// Log output targets supported by the logger.
/// # Examples
/// ```
/// use rust_fundamentals1::logging_config::LogOutput;
/// let output = LogOutput::Stdout;
/// ```
pub enum LogOutput {
    Stdout,
    Stderr,
    File(String),
}

/// Initializes the logger with the specified log level and output target.
/// # Examples
/// ```
/// use rust_fundamentals1::logging_config::{init_logger, LoggingConfig, LogLevel, LogOutput};
/// let config = LoggingConfig::new(LogLevel::Info, LogOutput::Stdout);
/// init_logger(config);
/// ```
pub fn init_logger(config: LoggingConfig) {
    std::env::set_var("RUST_LOG", config.level.as_str());
    match config.output {
        LogOutput::Stdout => env_logger::init(),
        LogOutput::Stderr => env_logger::Builder::new()
            .target(env_logger::Target::Stderr)
            .init(),
        LogOutput::File(path) => {
            let file = std::fs::File::create(path).expect("Failed to create log file");
            env_logger::Builder::new()
                .target(env_logger::Target::Pipe(Box::new(file)))
                .init();
        }
    }
}

/// Configuration for the logger including log level and output target.
/// # Examples
/// ```
/// use rust_fundamentals1::logging_config::{LoggingConfig, LogLevel, LogOutput};
/// let config = LoggingConfig::new(LogLevel::Info, LogOutput::Stdout);
/// ```
pub struct LoggingConfig {
    pub level: LogLevel,
    pub output: LogOutput,
}


impl LoggingConfig {
    pub fn new(level: LogLevel, output: LogOutput) -> Self {
        LoggingConfig { level, output }
    }
}
