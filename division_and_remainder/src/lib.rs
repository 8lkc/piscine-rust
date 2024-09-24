pub fn divide(x: i32, y: i32) -> (i32, i32) {
    if y == 0 {panic!("Division by zero is not allowed");}
    let quotient = x / y; let remainder = x % y;
    (quotient, remainder)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = divide(9, 4); assert_eq!(result, (2, 1));
    }
}
