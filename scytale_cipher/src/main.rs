use scytale_cipher::*;

fn main() {
    let input = "scytale Code".to_string();
    let scytale_code = scytale_cipher(input, 8);

    println!("{}$", scytale_code);
}
