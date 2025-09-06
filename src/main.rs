use rust_fundamentals1::{file_writer, get_input};
use rust_fundamentals1::logging_config::{init_logger, LoggingConfig, LogLevel, LogOutput};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let config = setup_logging_config();
    init_logger(config);
    let filename = get_input("Enter the filename: ");
    log::info!("Attempting to open file: {}", filename);
    let file = File::open(filename);
    let file = match file {
        Ok(file) => file,
        Err(error) => match error.kind() {
            std::io::ErrorKind::NotFound => {
                panic!("File not found: {}", error)
            }
            std::io::ErrorKind::PermissionDenied => {
                panic!("Permission denied: {}", error)
            }
            std::io::ErrorKind::InvalidInput => {
                panic!("Invalid input or invalid path: {}", error)
            }
            _ => {
                panic!("Error opening file: {}", error)
            }
        },
    };

    let reader = BufReader::new(file);
    for line in reader.lines() {
        match line {
            Ok(line) => println!("{}", line),
            Err(error) => match error.kind() {
                std::io::ErrorKind::UnexpectedEof => {
                    panic!("Unexpected end of file: {}", error)
                }
                std::io::ErrorKind::BrokenPipe => {
                    panic!("Broken pipe: {}", error)
                }
                std::io::ErrorKind::WouldBlock => {
                    panic!("Operation would block: {}", error)
                }
                std::io::ErrorKind::TimedOut => {
                    panic!("Operation timed out: {}", error)
                }
                std::io::ErrorKind::Interrupted => {
                    panic!("Operation interrupted: {}", error)
                }
                _ => {
                    panic!("Error reading line: {}", error)
                }
            },
        }
    }
    let path = "output.txt";
    let content = "Hello, world!";
    file_writer::write_to_file(path, content).expect("Failed to write to file");
}

fn setup_logging_config() -> LoggingConfig {
    let level = LogLevel::Info;
    let output = LogOutput::Stdout;
    LoggingConfig::new(level, output)
}
