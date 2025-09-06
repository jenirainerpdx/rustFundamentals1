/// Simple demo project that reads a filename and then outputs the contents of the file
/// This is more for exercising pattern matching for error handling.
///
use rust_fundamentals1::{file_writer, get_input};
use std::fs::File;
use std::io::{BufRead, BufReader};

/// Entry point of the application
/// # Example:
/// ```
/// make run
/// ... then enter a filename that is on the path
/// ```
///
/// # Panics
///
/// This function will panic for the following reasons:
/// - File not found
/// - Permission denied
/// - Invalid inputs
/// - Some other error in opening the file
///
/// # Errors
///
/// This function will return an error if:
/// - There is an unexpecked End of File
/// - A broken pipe error
/// - Blocking error
/// - Time out error
/// - Interrupted error
/// - Some other error in reading the file
///
fn main() {
    env_logger::init();
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
