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

pub fn atbash_cipher(input: &str) -> String {
    input.chars().map(|c| {
        if c.is_ascii_alphabetic() {
            let a = if c.is_ascii_uppercase() { 'A' } else { 'a' };
            let z = if c.is_ascii_uppercase() { 'Z' } else { 'z' };
            let mirrored_char = (z as u8) + (a as u8) - (c as u8);
            mirrored_char as char
        } else {
            c
        }
    }).collect()
}

pub fn cipher(original: &str, ciphered: &str) -> Option<Result<bool, CipherError>> {
    let expected = atbash_cipher(original);
    if expected == ciphered {
        Some(Ok(true))
    } else {
        Some(Err(CipherError::new(false, expected)))
    }
}
