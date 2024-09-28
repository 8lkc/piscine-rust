use std::collections::HashMap;

pub fn is_anagram(s1: &str, s2: &str) -> bool {
    let s1 = s1.to_lowercase().replace(" ", "");
    let s2 = s2.to_lowercase().replace(" ", "");
    if s1.len() != s2.len() {return false}
    let mut count_s1 = HashMap::new();
    let mut count_s2 = HashMap::new();
    for char in s1.chars() {*count_s1.entry(char).or_insert(0) += 1}
    for char in s2.chars() {*count_s2.entry(char).or_insert(0) += 1}
    count_s1 == count_s2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let s1 = "listen";
        let s2 = "silent";
        assert_eq!(is_anagram(s1, s2), true);
    }
}
