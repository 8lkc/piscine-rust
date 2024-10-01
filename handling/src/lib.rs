use std::{fs::{File, OpenOptions}, io::Write, path::Path};

pub fn open_or_create(file: &str, content: &str) {
    if !Path::new(file).exists() {
        let mut file = File::create(file).expect("Failed to create the file");
        file.write_all(content.as_bytes()).expect("Failed to write to the file");
    } else {
        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true) // Clear the existing content
            .open(file)
            .expect("Failed to open the file");
        file.write_all(content.as_bytes()).expect("Failed to write to the file");
    }
}
