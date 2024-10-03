pub fn num_to_ordinal(x: u32) -> String {
    let suffix = match x % 100 {
        11 | 12 | 13 => "th",
        _ => match x % 10 {1 => "st", 2 => "nd", 3 => "rd", _ => "th"},
    };
    format!("{}{}", x, suffix)
}

#[test]
fn test_num_to_ordinal() {
    assert_eq!(num_to_ordinal(0), "0th");
    assert_eq!(num_to_ordinal(1), "1st");
    assert_eq!(num_to_ordinal(12), "12th");
    assert_eq!(num_to_ordinal(22), "22nd");
    assert_eq!(num_to_ordinal(43), "43rd");
    assert_eq!(num_to_ordinal(67), "67th");
    assert_eq!(num_to_ordinal(1901), "1901st");
}
