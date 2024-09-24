pub fn fibonacci(n: u32) -> u32 {
    if n == 0 {return 0;}
    if n == 1 {return 1;}
    let mut prev = 0; let mut curr = 1;
    for _ in 2..=n {let next = prev + curr; prev = curr; curr = next;}
    curr
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let fibo_2 = fibonacci(2); assert_eq!(fibo_2, 1);
        let fibo_4 = fibonacci(4); assert_eq!(fibo_4, 3);
        let fibo_22 = fibonacci(22); assert_eq!(fibo_22, 17711);
        let fibo_20 = fibonacci(20); assert_eq!(fibo_20, 6765);
    }
}
