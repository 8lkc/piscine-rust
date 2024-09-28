pub fn capitalize_first(input: &str) -> String {
    let mut result = input.to_lowercase().chars().collect::<Vec<_>>();
    if let Some(first) = result.first_mut() {*first = first.to_ascii_uppercase()}
    result.iter().collect()
}

pub fn title_case(input: &str) -> String {
    let words  = input.split_whitespace().collect::<Vec<_>>();
    let mut result = Vec::new();
    for word in words.iter() {
        let temp = capitalize_first(*word);
        result.push(temp);
    }
    result.join(" ")
}

pub fn change_case(input: &str) -> String {
    input.chars().map(|c| {
        if c.is_ascii_uppercase() {
            c.to_ascii_lowercase()
        } else {
        c.to_ascii_uppercase()}
    }).collect()
}

//test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(capitalize_first("hello"), "Hello");
        assert_eq!(capitalize_first("joe is missing"), "Joe is missing");
    }

    #[test]
    fn test_title_case() {
        let r = title_case("jill is leaving A");
        assert_eq!(r, "Jill Is Leaving A");
    }

    #[test]
    fn test_change_case() {
        let r = change_case("heLLo THere");
        assert_eq!(r, "HEllO thERE");
    }
}
