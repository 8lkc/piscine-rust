pub fn to_url(s: &str) -> String {
    let mut url = String::new();
    let chars: Vec<_> = s.chars().collect();
    for char in chars {if char == ' ' {url.push_str("%20")} else {url.push(char)}}
    url
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let s = "Hello, world!";
        let result = to_url(s); assert_eq!(result, "Hello,%20world!");
    }
}
