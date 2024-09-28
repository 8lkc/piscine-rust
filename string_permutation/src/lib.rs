use std::collections::HashMap;

pub fn is_permutation(s1: &str, s2: &str) -> bool {
    if s1.len() != s2.len() {return false}
    let mut char_count = HashMap::new();
    for char in s1.chars() {*char_count.entry(char).or_insert(0) += 1}
    for char in s2.chars() {
        match char_count.get_mut(&char) {
            Some(count) => {*count -= 1; if *count < 0 {return false}}
            None => {return false}
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let word = "thought";
        let word1 = "thougth";
        assert_eq!(is_permutation(word, word1), true);
    }
}
