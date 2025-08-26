use rust_fundamentals1::get_filename;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let filename = get_filename();

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
}
