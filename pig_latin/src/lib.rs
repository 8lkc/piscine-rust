pub fn pig_latin(text: &str) -> String {
    let mut result = String::new();
    for word in text.split_whitespace() {
        let first_char = word.chars().next().unwrap_or('\0');
        if "aeiou".contains(first_char) {result.push_str(&format!("{}ay", word));}
        else {
            let mut first_vowel_index = None;
            let mut prev_c: char = '_';
            for (i, c) in word.chars().enumerate() {
                if "aeiou".contains(c) {
                    if c == 'u' && prev_c == 'q' && i > 1 {first_vowel_index = Some(i + 1)}
                    else {
                        first_vowel_index = Some(i);
                    }
                    break;
                }
                prev_c = c;
            }
            if let Some(index) = first_vowel_index {
                let (consonants, rest) = word.split_at(index);
                result.push_str(&format!("{}{}ay", rest, consonants));
            } else {result.push_str(&format!("{}ay", word));}
        }
    }
    result.trim_end().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_word_beginning_with_vowel() {
        assert_eq!(pig_latin(&String::from("apple")), "appleay");
        assert_eq!(pig_latin(&String::from("ear")), "earay");
        assert_eq!(pig_latin(&String::from("igloo")), "iglooay");
        assert_eq!(pig_latin(&String::from("object")), "objectay");
        assert_eq!(pig_latin(&String::from("under")), "underay");
        assert_eq!(pig_latin(&String::from("equal")), "equalay");
    }

    #[test]
    fn test_word_beginning_with_consonant() {
        assert_eq!(pig_latin(&String::from("queen")), "ueenqay");
        assert_eq!(pig_latin(&String::from("square")), "aresquay");
        assert_eq!(pig_latin(&String::from("pig")), "igpay");
        assert_eq!(pig_latin(&String::from("koala")), "oalakay");
        assert_eq!(pig_latin(&String::from("yellow")), "ellowyay");
        assert_eq!(pig_latin(&String::from("xenon")), "enonxay");
        assert_eq!(pig_latin(&String::from("qat")), "atqay");
        assert_eq!(pig_latin(&String::from("chair")), "airchay");
        assert_eq!(pig_latin(&String::from("therapy")), "erapythay");
        assert_eq!(pig_latin(&String::from("thrush")), "ushthray");
        assert_eq!(pig_latin(&String::from("school")), "oolschay");
        assert_eq!(pig_latin(&String::from("british")), "itishbray");
    }
}
