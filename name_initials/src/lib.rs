pub fn initials(names: Vec<&str>) -> Vec<String> {
    let result: Vec<String> = names.into_iter().map(|name| {
        name.split_whitespace()
            .filter_map(|word| word.chars().next()) // Get the first character of each word
            .map(|ch| format!("{}.", ch.to_uppercase())) // Format it as an uppercase initial followed by '.'
            .collect::<Vec<String>>() // Collect initials into a vector of strings
            .join(" ") // Join the initials with a space
    }).collect();
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let names = vec!["Harry Potter", "Someone Else", "J. L.", "Barack Obama"];
        let result = initials(names); assert_eq!(result, ["H. P.", "S. E.", "J. L.", "B. O."]);
    }
}
