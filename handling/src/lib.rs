use std::{fs::{File, OpenOptions}, io::Write, path::Path};

pub fn open_or_create(file: &str, content: &str) {
    let mut opject: File;
    if Path::new(file).exists() {opject = File::create(file).expect("Failed to create the file");}
    else {
        opject = OpenOptions::new().write(true).truncate(true).open(file).expect("Failed to create the file");
    }
    opject.write_all(content.as_bytes()).expect("Failed to write to the file");
}
