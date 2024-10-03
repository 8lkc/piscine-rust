use std::collections::HashMap;

pub fn score(word: &str) -> u64 {
    let mut letter_values: HashMap<char, u64> = HashMap::new();

    "AEIOULNRST".chars().for_each(|ch| {letter_values.insert(ch, 1);});
    "DG".chars().for_each(|ch| {letter_values.insert(ch, 2);});
    "BCMP".chars().for_each(|ch| {letter_values.insert(ch, 3);});
    "FHVWY".chars().for_each(|ch| {letter_values.insert(ch, 4);});
    letter_values.insert('K', 5);
    "JX".chars().for_each(|ch| {letter_values.insert(ch, 8);});
    "QZ".chars().for_each(|ch| {letter_values.insert(ch, 10);});

    word.to_uppercase().chars().filter_map(|ch| {if letter_values.contains_key(&ch) {letter_values.get(&ch)} else {None}}).sum()
}
