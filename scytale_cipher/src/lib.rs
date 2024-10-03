pub fn scytale_cipher(message: String, i: u32) -> String {
    let mut result = String::new();
    let chunks = split_into_chunks(message, i as usize);
    let number_of_chunks = chunks.len();
    for j in 0..i as usize {
        for i in 0..number_of_chunks {result.push(chunks[i][j])}
    }
    result.trim().to_string()
}

fn split_into_chunks(s: String, chunk_size: usize) -> Vec<Vec<char>> {
    let mut result = Vec::new();
    let mut chars_iter = s.chars().peekable();
    while chars_iter.peek().is_some() {
        let mut chunk = Vec::with_capacity(chunk_size);
        for _ in 0..chunk_size {
            if let Some(c) = chars_iter.next() {chunk.push(c)}
            else {chunk.push(' ')}
        }
        result.push(chunk);
    } result
}

#[test]
fn test_scytale_cipher() {
    scytale_cipher(String::from("attack morning"), 6);
    assert_eq!(&scytale_cipher(String::from("scytale Code"), 6), "sec yCtoadle");
    assert_eq!(&scytale_cipher(String::from("scytale Code"), 8), "sCcoydtea l e");
    assert_eq!(&scytale_cipher(String::from(""), 4), "");
    assert_eq!(&scytale_cipher(String::from("qwerty qwerty"), 13), "qwerty qwerty");
    assert_eq!(&scytale_cipher(String::from("attack morning"), 6), "a ntmgto ar cn ki");
}
