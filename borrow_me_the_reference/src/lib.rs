pub fn delete_and_backspace(s: &mut String) {
    let mut result = Vec::new();
    for ch in s.chars() {match ch {'-' => {result.pop();} _ => result.push(ch),}}
    *s = result.into_iter().collect();

    let mut result = Vec::new();
    for ch in s.chars().rev() {match ch {'+' => {result.pop();} _ => result.push(ch),}}
    *s = result.into_iter().rev().collect(); // Convert the stack back into a string
}

pub fn do_operations(v: &mut Vec<String>) {
    for equation in v.iter_mut() {
        // Split the equation into parts by either '+' or '-'
        if let Some((lhs, rhs)) = equation.split_once('+') {
            let left: i32 = lhs.trim().parse().unwrap_or(0); // Parse left operand
            let right: i32 = rhs.trim().parse().unwrap_or(0); // Parse right operand
            *equation = (left + right).to_string(); // Replace with the sum
        } else if let Some((lhs, rhs)) = equation.split_once('-') {
            let left: i32 = lhs.trim().parse().unwrap_or(0); // Parse left operand
            let right: i32 = rhs.trim().parse().unwrap_or(0); // Parse right operand
            *equation = (left - right).to_string(); // Replace with the difference
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut a = String::from("bpp--o+er+++sskroi-++lcw");
        let mut b: Vec<String> = vec!["2+2".to_string(), "3+2".to_string(), "10-3".to_string(), "5+5".to_string(),];

        delete_and_backspace(&mut a); assert_eq!(a, "borrow");
        do_operations(&mut b); assert_eq!(b, ["4", "5", "7", "10"]);
    }
}
