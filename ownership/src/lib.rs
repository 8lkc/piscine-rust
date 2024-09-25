pub fn first_subword(mut s: String) -> String {
    // Handle snake_case
    if let Some(pos) = s.find('_') {s.truncate(pos); return s;}
    // Handle camelCase and PascalCase
    for (i, ch) in s.char_indices() {if ch.is_uppercase() && i != 0 {s.truncate(i); return s;}}
    // If no special cases were found, return the original string
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let s1 = String::from("helloWorld");
        let s2 = String::from("snake_case");
        let s3 = String::from("CamelCase");
        let s4 = String::from("just");

        let r1 = first_subword(s1); assert_eq!(r1, "hello");
        let r2 = first_subword(s2); assert_eq!(r2, "snake");
        let r3 = first_subword(s3); assert_eq!(r3, "Camel");
        let r4 = first_subword(s4); assert_eq!(r4, "just");
    }
}
