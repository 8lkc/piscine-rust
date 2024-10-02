use macro_calculator::*;

fn main() {
    let a = vec![
        Food {
            name: String::from("big mac"),
            calories: ["2133.84kJ".to_string(), "510kcal".to_string()],
            proteins: 27.0,
            fats: 26.0,
            carbs: 41.0,
            nbr_of_portions: 2.0,
        },
        Food {
            name: "pizza margherita".to_string(),
            calories: ["1500.59kJ".to_string(), "358.65kcal".to_string()],
            proteins: 13.89,
            fats: 11.21,
            carbs: 49.07,
            nbr_of_portions: 4.9,
        },
        Food {
            name: String::from("extreme burger"),
            calories: ["9999999999999.9999999999999kJ".to_string(), "99999999.9999kcal".to_string()],
            proteins: 999999.9999999999,
            fats: 999999.9999999999,
            carbs: 999999.9999999999,
            nbr_of_portions: 1000.0,
        },
        Food {
            name: String::from("tiny snack"),
            calories: ["0.000001kJ".to_string(), "0.000000001kcal".to_string()],
            proteins: 0.000000001,
            fats: 0.000000001,
            carbs: 0.000000001,
            nbr_of_portions: 0.00000001,
        },
    ];
    println!("{:#}", calculate_macros(a));
}
