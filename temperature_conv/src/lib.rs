pub fn fahrenheit_to_celsius(f: f64) -> f64 {(f-32.0)/(9.0/5.0)}
pub fn celsius_to_fahrenheit(c: f64) -> f64 {(9.0/5.0)*c+32.0}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let c_result = fahrenheit_to_celsius(-459.67); assert_eq!(c_result, -273.15);
        let f_result = celsius_to_fahrenheit(0.0); assert_eq!(f_result, 32.0);
    }
}
