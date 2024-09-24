pub fn str_len(s: &str) -> usize {s.len()}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let s = "hello"; let s1 = "camelCase".to_string();
        let strng_lit = str_len(s); assert_eq!(strng_lit, 5);
        let strng = str_len(&s1); assert_eq!(strng, 9);
    }
}
