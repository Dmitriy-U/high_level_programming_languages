use std::io;
use regex::Regex;

fn validate_input(string: &String) -> bool {
    let re = Regex::new(r"^[\d ]+$").unwrap();
    let match_string = &string[..];
    re.is_match(match_string)
}

pub fn get_user_input_string() -> String {
    let mut user_input = String::new();
    println!("Enter a sequence of integers separated by spaces:");
    io::stdin().read_line(&mut user_input).expect("An incorrect sequence of integers was entered. Sample: 12 345 6 7890");
    user_input = String::from(user_input.trim());
    println!("Введено: {}", user_input);

    if !validate_input(&user_input) {
        user_input = get_user_input_string();
    }

    user_input
}

pub fn convert_input_to_vector(string_source: String) -> Vec<i32> {
    let mut vector: Vec<i32> = Vec::new();
    let string_split_sequence: Vec<&str> = string_source.split(" ").collect();
    for string_item in string_split_sequence {
        vector.push(string_item.to_string().parse().unwrap());
    }
    vector
}
