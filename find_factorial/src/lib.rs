pub fn factorial(num: u64) -> u64 {
    if num == 2 {
        return 2;
    } else if num > 2 {
        let mut fact: u64 = 2; let mut counter = 3;
        loop {fact *= counter; if counter == num {break;} counter += 1;}
        return fact;
    }
    1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result_1 = factorial(0); assert_eq!(result_1, 1);
        let result_2 = factorial(1); assert_eq!(result_2, 1);
        let result_3 = factorial(5); assert_eq!(result_3, 120);
        let result_4 = factorial(10); assert_eq!(result_4, 3628800);
        let result_5 = factorial(19); assert_eq!(result_5, 121645100408832000);
    }
}
