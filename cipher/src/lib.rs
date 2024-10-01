#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CipherError {pub validation: bool, pub expected: String}

impl CipherError {
    pub fn new(validation: bool, expected: String) -> CipherError {CipherError {validation, expected}}
}

pub fn cipher(original: &str, ciphered: &str) -> Option<Result<bool, CipherError>> {
    if original.is_empty() || ciphered.is_empty() {return None}
    let expected_cipher: String = original.chars().map(|c| {
        if c.is_ascii_uppercase() {(b'Z' - (c as u8 - b'A')) as char}
        else if c.is_ascii_lowercase() {(b'z' - (c as u8 - b'a')) as char}
        else {c}
    }).collect();
    if expected_cipher == ciphered {Some(Ok(true))}
    else {Some(Err(CipherError::new(false, expected_cipher)))}
}
