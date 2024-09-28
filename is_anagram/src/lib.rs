use std::collections::HashMap;

// Function to check if one string is an anagram of another
pub fn is_anagram(s1: &str, s2: &str) -> bool {
    // Remove spaces and convert to lowercase
    let s1 = s1.to_lowercase().replace(" ", "");
    let s2 = s2.to_lowercase().replace(" ", "");

    // Early return if lengths are different
    if s1.len() != s2.len() {
        return false;
    }

    // Create HashMaps to count character occurrences
    let mut count_s1 = HashMap::new();
    let mut count_s2 = HashMap::new();

    for char in s1.chars() {
        *count_s1.entry(char).or_insert(0) += 1;
    }

    for char in s2.chars() {
        *count_s2.entry(char).or_insert(0) += 1;
    }

    // Compare the two HashMaps
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
