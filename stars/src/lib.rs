pub fn stars(n: u32) -> String {
    let mut result = String::from("*");
    for _i in 1..=(2u32.pow(n) - 1) {result.push('*')}
    result
}
