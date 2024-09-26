pub fn is_armstrong_number(nb: u32) -> Option<u32> {
    // Convert the number to a string to easily access individual digits
    let digits: Vec<u32> = nb.to_string().chars()
        .map(|d| d.to_digit(10).unwrap())
        .collect();

    // Calculate the power based on the number of digits
    let power = digits.len() as u32;

    // Compute the sum of each digit raised to the power
    let sum: u32 = digits.iter().map(|&d| d.pow(power)).sum();

    // Return the number if it's an Armstrong number, otherwise None
    if sum == nb {
        Some(nb)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result_0 = is_armstrong_number(0); assert_eq!(result_0, Some(0));
        let result_1 = is_armstrong_number(1); assert_eq!(result_1, Some(1));
        let result_2 = is_armstrong_number(153); assert_eq!(result_2, Some(153));
        let result_3 = is_armstrong_number(370); assert_eq!(result_3, Some(370));
        let result_4 = is_armstrong_number(371); assert_eq!(result_4, Some(371));
        let result_5 = is_armstrong_number(407); assert_eq!(result_5, Some(407));
        let result_6 = is_armstrong_number(400); assert_eq!(result_6, None);
        let result_7 = is_armstrong_number(198); assert_eq!(result_7, None);
    }
}
