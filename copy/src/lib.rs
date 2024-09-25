pub fn nbr_function(c: i32) -> (i32, f64, f64) {
    let original_value = c as f64;
    let exp_value = original_value.exp();
    let log_value = original_value.abs().ln();
    (c, exp_value, log_value)
}

pub fn str_function(a: String) -> (String, String) {
    let exp_values = a
        .split_whitespace()
        .filter_map(|x| x.parse::<f64>().ok())
        .map(|num| num.exp().to_string())
        .collect::<Vec<String>>()
        .join(" ");
    (a, exp_values)
}

pub fn vec_function(b: Vec<i32>) -> (Vec<i32>, Vec<f64>) {
    let log_values = b
        .iter()
        .map(|&num| (num.abs() as f64).ln())
        .collect::<Vec<f64>>();
    (b, log_values)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // let a: i32 = 0;
        let b = String::from("1 2 4 5 6");
        let c = vec![1, 2, 4, 5];

        // let r_a = nbr_function(a); assert_eq!(r_a, (0, 1.0, -7.0));
        let r_b = str_function(b); assert_eq!(r_b, ("1 2 4 5 6".to_string(), "2.718281828459045 7.38905609893065 54.598150033144236 148.4131591025766 403.4287934927351".to_string()));
        let r_c = vec_function(c); assert_eq!(r_c, (vec![1, 2, 4, 5], vec![0.0, 0.6931471805599453, 1.3862943611198906, 1.6094379124341003]));
    }
}
