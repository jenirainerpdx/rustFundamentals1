//! Utility functions for writing data to files
//! Provides a function to write a string to a specified file path.
//!
//!

use std::fs::File;
use std::io::Write;

/// Writes the given content to a file at the specified path.
/// Returns a Result indicating success or failure.
/// # Examples:
/// ```
/// use rust_fundamentals1::write_to_file;
/// let path = "output.txt";
/// let content = "Hello, world!";
/// write_to_file(path, content).expect("Failed to write to file");
/// ```
///
/// # Errors
///
/// This function will return an error if:
/// - The file cannot be created or opened
/// - The content cannot be written to the file
///
pub fn write_to_file(path: &str, content: &str) -> std::io::Result<()> {
    let mut file = File::create(path)?;
    log::warn!("Writing to file: {}", path);
    file.write_all(content.as_bytes())?;
    Ok(())
}
