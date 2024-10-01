use std::{fs::{File, OpenOptions}, io::Write, path::Path};

pub fn open_or_create(file: &str, content: &str) {
    // Check if the file exists
    if !Path::new(file).exists() {
        // Create the file if it doesn't exist
        let mut file = File::create(file).expect("Failed to create the file");
        file.write_all(content.as_bytes()).expect("Failed to write to the file");
    } else {
        // Open the existing file with write access
        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true) // Clear the existing content
            .open(file)
            .expect("Failed to open the file");
        file.write_all(content.as_bytes()).expect("Failed to write to the file");
    }
}
