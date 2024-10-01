#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CipherError {
    pub validation: bool,
    pub expected: String,
}

impl CipherError {
    pub fn new(validation: bool, expected: String) -> CipherError {
        CipherError { validation, expected }
    }
}

pub fn cipher(original: &str, ciphered: &str) -> Option<Result<bool, CipherError>> {
    // Return None if the original string is empty
    if original.is_empty() {
        return None; // Early return for empty original string
    }

    // Generate the expected Atbash cipher for the original string
    let expected_cipher: String = original
        .chars()
        .map(|c| {
            // Check if character is an uppercase letter
            if c.is_ascii_uppercase() {
                (b'Z' - (c as u8 - b'A')) as char
            // Check if character is a lowercase letter
            } else if c.is_ascii_lowercase() {
                (b'z' - (c as u8 - b'a')) as char
            // Return non-alphabetic characters unchanged
            } else {
                c
            }
        })
        .collect();

    // Compare the expected cipher with the provided cipher
    if expected_cipher == ciphered {
        Some(Ok(true)) // Return true if they match
    } else {
        Some(Err(CipherError::new(false, expected_cipher))) // Return error with the expected cipher
    }
}
