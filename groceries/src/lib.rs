pub fn insert(vec: &mut Vec<String>, val: String) {vec.push(val);}
pub fn at_index(vec: &Vec<String>, index: usize) -> String {
    let value: Option<&String> = vec.get(index);
    match value {
        Some(value) => value.clone(),
        None => "".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut groceries = vec![
            "yogurt".to_string(),
            "panettone".to_string(),
            "bread".to_string(),
            "cheese".to_string(),
        ];
        let result = at_index(&groceries, 2); assert_eq!(result, "bread".to_string());
    }
}
