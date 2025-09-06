//! Utility functions for user input handling
//! Provides a function to get user input with a custom prompt.
//! This function prints the prompt to the console and reads a line of input from the user.

pub mod file_writer;
pub mod logging_config;

use std::io::BufRead;

/// Gets user input from the console with a custom prompt.
/// Returns the input as a trimmed String.
/// # Examples:
/// ```
/// use rust_fundamentals1::get_input;
/// let name = get_input("Enter your name: ");
/// println!("Hello, {}", name);
/// ```
/// # Panics
/// This function will panic if it fails to flush stdout or read from stdin.
///
/// # Errors
///  None
///
pub fn get_input(prompt: &str) -> String {
    use std::io::{self, Write};

    print!("{}", prompt);
    io::stdout().flush().expect("Failed to flush stdout");

    let stdin = std::io::stdin();
    let mut reader = stdin.lock();
    _read_stdin(&mut reader)
}

fn _read_stdin<R: BufRead>(reader: &mut R) -> String {
    let mut read_input = String::new();
    reader
        .read_line(&mut read_input)
        .expect("Failed to read line");
    let path = "output.txt";
    let content = "Hello, world!";
    file_writer::write_to_file(path, content).expect("Failed to write to file");
    read_input.trim().to_string()
}
