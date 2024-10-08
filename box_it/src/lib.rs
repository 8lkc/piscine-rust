pub fn transform_and_save_on_heap(s: String) -> Box<Vec<u32>> {
    let mut result: Vec<u32> = Vec::new();
    let numbers = s.split_whitespace();
    let mut multiplyer: f32;
    let mut temporary_number: String;
    for number in numbers {
        multiplyer = 1.0;
        if number.ends_with("k") {multiplyer = 1000.0; temporary_number = number.replace("k", "")}
        else {temporary_number = number.to_string()};
        result.push((multiplyer * temporary_number.parse::<f32>().unwrap_or(0.0)) as u32);
    }
    Box::new(result)
}

pub fn take_value_ownership(a: Box<Vec<u32>>) -> Vec<u32> {*a}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem;

    #[test]
    fn test_transform() {
        let new_str = String::from("5.5k 8.9k 32");
        let new_str_1 = String::from("6.8k 13.5k");
        let new_str_2 = String::from("20.3k 3.8k 7.7k 992");

        let a = transform_and_save_on_heap(new_str);
        let b = transform_and_save_on_heap(new_str_1);
        let c = transform_and_save_on_heap(new_str_2);

        assert_eq!(a, Box::new(vec![5500, 8900, 32]));
        assert_eq!(b, Box::new(vec![6800, 13500]));
        assert_eq!(c, Box::new(vec![20300, 3800, 7700, 992]));
        assert_eq!(mem::size_of_val(&a), 8);
        assert_eq!(mem::size_of_val(&b), 8);
        assert_eq!(mem::size_of_val(&c), 8);
    }

    #[test]
    fn test_take_value_from_box() {
        let new_str = String::from("5.5k 8.9k 32");
        let new_str_1 = String::from("6.8k 13.5k");
        let new_str_2 = String::from("20.3k 3.8k 7.7k 992");
        let a = take_value_ownership(transform_and_save_on_heap(new_str));
        let b = take_value_ownership(transform_and_save_on_heap(new_str_1));
        let c = take_value_ownership(transform_and_save_on_heap(new_str_2));

        assert_eq!(a, vec![5500, 8900, 32]);
        assert_eq!(b, vec![6800, 13500]);
        assert_eq!(c, vec![20300, 3800, 7700, 992]);
        assert_eq!(mem::size_of_val(&a), 24);
        assert_eq!(mem::size_of_val(&b), 24);
        assert_eq!(mem::size_of_val(&c), 24);
    }
}
