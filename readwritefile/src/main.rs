use std::fs::OpenOptions;
use std::io::{self, Read, Write};

fn main() {
    println!("Welcome to simple text editor");
    write_to_file();
}

fn write_to_file() {
    let mut data_file = OpenOptions::new()
        .create(true) // Create the file if it doesn't exist
        .append(true) // Open in append mode
        .open("data.txt")
        .expect("Failed to open or create file");

    println!("Start entering text. Type ':wq' to quit.");

    let mut buffer_reader = String::new();
    while buffer_reader.trim() != ":wq" {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        buffer_reader = input.trim().to_string(); // Remove extra newline characters

        if buffer_reader != ":wq" {
            data_file
                .write_all(buffer_reader.as_bytes()) // Write user input to the file
                .expect("Failed to write to file");
            data_file
                .write_all(b"\n") // Add newline after each input
                .expect("Failed to write newline to file");
        }
    }

    println!("Text has been written to the file.");
}
