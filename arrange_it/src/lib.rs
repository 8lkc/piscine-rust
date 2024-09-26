pub fn arrange_phrase(phrase: &str) -> String {
    // Split the phrase into words
    let mut words: Vec<&str> = phrase.split_whitespace().collect();

    // Sort the words based on the embedded number in each word
    words.sort_by_key(|word| {
        word.chars()
            .find(|c| c.is_digit(10))
            .unwrap()
            .to_digit(10)
            .unwrap()
    });

    // Remove the numbers from each word and collect them into a new vector
    let cleaned_words: Vec<String> = words
        .iter()
        .map(|&word| word.chars().filter(|c| !c.is_digit(10)).collect())
        .collect();

    // Join the cleaned words back into a single string
    cleaned_words.join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = arrange_phrase("is2 Thi1s T4est 3a"); assert_eq!(result, "This is a Test");
    }
}
