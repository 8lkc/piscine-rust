pub fn rev_str(input: &str) -> String {
    let mut result = String::new();
    let chars: Vec<_> = input.chars().rev().collect();
    for letter in chars {result.push(letter);}
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result_1 = rev_str("Hello, world!"); assert_eq!(result_1, "!dlrow ,olleH".to_string());
        let result_2 = rev_str("Hello, my name is Roman"); assert_eq!(result_2, "namoR si eman ym ,olleH".to_string());
        let result_3 = rev_str("I have a nice car!"); assert_eq!(result_3, "!rac ecin a evah I".to_string());
        let result_4 = rev_str("How old are You"); assert_eq!(result_4, "uoY era dlo woH".to_string());
        let result_5 = rev_str("ex: this is an example água"); assert_eq!(result_5, "augá elpmaxe na si siht :xe".to_string());
    }
}
