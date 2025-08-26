pub fn get_filename() -> String {
    use std::io::{self, Write};

    print!("Enter the filename: ");
    io::stdout().flush().expect("Failed to flush stdout");

    let mut filename = String::new();
    io::stdin()
        .read_line(&mut filename)
        .expect("Failed to read line");
    filename.trim().to_string()
}
