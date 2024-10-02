extern crate json;

pub struct Food {
    pub name: String,
    pub calories: [String; 2],
    pub fats: f64,
    pub carbs: f64,
    pub proteins: f64,
    pub nbr_of_portions: f64
}

pub fn calculate_macros(foods: Vec<Food>) -> json::JsonValue {
    let mut sum_of_cals = 0.0; let mut sum_of_carbs = 0.0; let mut sum_of_proteins = 0.0; let mut sum_of_fats = 0.0;
    for food in &foods {
        sum_of_cals += extract_calories(&food.calories) * food.nbr_of_portions;
        sum_of_fats += food.fats * food.nbr_of_portions;
        sum_of_carbs += food.carbs * food.nbr_of_portions;
        sum_of_proteins += food.proteins * food.nbr_of_portions;
    }
    let mut json_object = json::JsonValue::new_object();
    json_object["cals"] = round_to_precision(sum_of_cals).into();
    json_object["carbs"] = round_to_precision(sum_of_carbs).into();
    json_object["proteins"] = round_to_precision(sum_of_proteins).into();
    json_object["fats"] = round_to_precision(sum_of_fats).into();
    json_object
}

fn extract_calories(calories: &[String; 2]) -> f64 {calories[1].replace("kcal", "").trim().parse().unwrap_or(0.0)}

fn round_to_precision(value: f64) -> f64 {
    let rounded = (value * 100.0).round() / 100.0;
    if rounded == rounded.trunc() {return (value * 10.0).round() / 10.0;}
    rounded
}
