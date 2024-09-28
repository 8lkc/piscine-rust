use std::collections::HashMap;

pub fn bigger(h: HashMap<&str, i32>) -> i32 {
    let mut max_value = i32::MIN;
    for &value in h.values() {if value > 0 && value > max_value {max_value = value}}
    if max_value > 0 {max_value} else {0}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut hash = HashMap::new();
        hash.insert("Daniel", 122);
        hash.insert("Ashley", 333);
        hash.insert("Katie", 334);
        hash.insert("Robert", 14);
        assert_eq!(bigger(hash), 334);
    }
}
