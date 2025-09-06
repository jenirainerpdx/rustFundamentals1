//! Logging configuration module
//!
//! This module provides utilities for configuring logging behavior.
//! It supports setting log levels and directing log output to various targets
//! such as stdout, stderr, or a file.

use flexi_logger::{FileSpec, Logger};

/// Log levels supported by the logger.
/// # Examples
/// ```
/// use rust_fundamentals1::logging_config::LogLevel;
/// let level = LogLevel::Info;
/// assert_eq!(level.as_str(), "INFO");
/// ```
#[derive(Debug, PartialEq)]
pub enum LogLevel {
    Debug,
    Error,
    Info,
    Trace,
    Warn,
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
#[derive(Debug, PartialEq, Eq)]
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
    match config.get_output() {
        LogOutput::Stdout => {
            Logger::try_with_str(config.get_level().as_str())
                .unwrap()
                .log_to_stdout()
                .start()
                .unwrap();
        }
        LogOutput::Stderr => {
            Logger::try_with_str(config.get_level().as_str())
                .unwrap()
                .log_to_stderr()
                .start()
                .unwrap();
        }
        LogOutput::File(path) => {
            let filespec = FileSpec::default().directory(".").basename(path);

            Logger::try_with_str(config.get_level().as_str())
                .unwrap()
                .log_to_file(filespec)
                .start()
                .unwrap();
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
    level: LogLevel,
    output: LogOutput,
}

/// Configuration for the logger including log level and output target.
/// # Examples (probably unnecessary to test getters and setters... )
/// ```
/// use rust_fundamentals1::logging_config::{LoggingConfig, LogLevel, LogOutput};
/// let mut config = LoggingConfig::new(LogLevel::Info, LogOutput::Stdout);
/// config.set_level(LogLevel::Debug);
/// config.set_output(LogOutput::File("log.txt".to_string()));
/// assert_eq!(config.get_level(), &LogLevel::Debug);
/// assert_eq!(config.get_output(), &LogOutput::File("log.txt".to_string()));
/// ```
impl LoggingConfig {
    pub fn new(level: LogLevel, output: LogOutput) -> Self {
        LoggingConfig { level, output }
    }
    pub fn set_level(&mut self, level: LogLevel) {
        self.level = level;
    }
    pub fn set_output(&mut self, output: LogOutput) {
        self.output = output;
    }
    pub fn get_level(&self) -> &LogLevel {
        &self.level
    }
    pub fn get_output(&self) -> &LogOutput {
        &self.output
    }
}
