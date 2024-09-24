pub fn is_empty(v: &str) -> bool {v.is_empty()}
pub fn is_ascii(v: &str) -> bool {v.is_ascii()}
pub fn contains(v: &str, pat: &str) -> bool {v.contains(pat)}
pub fn split_at(v: &str, index: usize) -> (&str, &str) {v.split_at(index)}
pub fn find(v: &str, pat: char) -> usize {v.find(pat).unwrap_or_else(|| v.len())}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result_1 = is_empty(""); assert_eq!(result_1, true);
        let result_2 = is_ascii("rust"); assert_eq!(result_2, true);
        let result_3 = contains("rust", "ru"); assert_eq!(result_3, true);
        let result_4 = split_at("rust", 2); assert_eq!(result_4, ("ru", "st"));
        let result_5 = find("rust", 'u'); assert_eq!(result_5, 1);
    }
}
