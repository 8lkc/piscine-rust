pub fn tic_tac_toe(table: Vec<Vec<&str>>) -> String {
    if diagonals("O", &table) || horizontal("O", &table) || vertical("O", &table) {
        return "player O won".to_string();
    }
    if diagonals("X", &table) || horizontal("X", &table) || vertical("X", &table) {
        return "player X won".to_string();
    }
    "tie".to_string()
}

pub fn diagonals(player: &str, table: &Vec<Vec<&str>>) -> bool {
    // Check both diagonals
    (table[0][0] == player && table[1][1] == player && table[2][2] == player) ||
    (table[0][2] == player && table[1][1] == player && table[2][0] == player)
}

pub fn horizontal(player: &str, table: &Vec<Vec<&str>>) -> bool {
    // Check all rows
    for row in table {
        if row.iter().all(|&cell| cell == player) {
            return true;
        }
    }
    false
}

pub fn vertical(player: &str, table: &Vec<Vec<&str>>) -> bool {
    // Check all columns
    for col in 0..3 {
        if table[0][col] == player && table[1][col] == player && table[2][col] == player {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result1 = tic_tac_toe(vec![
            vec!["O", "X", "O"],
            vec!["O", "O", "X"],
            vec!["X", "#", "X"]
        ]); assert_eq!(result1, "tie");
        let result2 = tic_tac_toe(vec![
            vec!["X", "O", "O"],
            vec!["X", "O", "O"],
            vec!["#", "O", "X"]
        ]); assert_eq!(result2, "player O won");
        let result3 = tic_tac_toe(vec![
            vec!["O", "O", "X"],
            vec!["O", "X", "O"],
            vec!["X", "#", "X"]
        ]); assert_eq!(result3, "player X won");
    }
}
