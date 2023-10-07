use std::io;
use regex::Regex;

fn validate_input_string_by_sequence_of_digits(string: &String) -> bool {
    let re = Regex::new(r"^[\d ]+$").unwrap();
    let match_string = &string[..];
    re.is_match(match_string)
}

fn convert_input_to_vector(string_source: String) -> Vec<i32> {
    let mut vector: Vec<i32> = Vec::new();
    let string_split_sequence: Vec<&str> = string_source.split(" ").collect();
    for string_item in string_split_sequence {
        vector.push(string_item.to_string().parse().unwrap());
    }
    vector
}

fn get_user_input_string() -> String {
    let mut user_input = String::new();
    println!("Enter a sequence of integers separated by spaces:");
    io::stdin().read_line(&mut user_input).expect("An incorrect sequence of integers was entered. Sample: 12 345 6 7890");
    user_input = String::from(user_input.trim());
    println!("Typed: {}", user_input);

    if !validate_input_string_by_sequence_of_digits(&user_input) {
        user_input = get_user_input_string();
    }

    user_input
}

pub fn get_user_sequence_of_digits() -> Vec<i32> {
    let user_input_string = get_user_input_string();
    convert_input_to_vector(user_input_string)
}

pub fn get_user_sequence_of_digits_with_count(count: usize) -> Vec<i32> {
    let mut user_input_vector = get_user_sequence_of_digits();
    let length = user_input_vector.len();
    if length != count {
        println!("You need to type {count} of digits");
        user_input_vector = get_user_sequence_of_digits();
    }
    user_input_vector
}

pub fn get_user_two_dimensional_sequence(count: usize) -> Vec<Vec<i32>> {
    let mut user_two_dimensional_sequence: Vec<Vec<i32>> = Vec::new();
    for index in 0..count {
        let row: usize = index + 1;
        println!("{row} row");
        user_two_dimensional_sequence.push(get_user_sequence_of_digits_with_count(count));
    }

    println!("Typed two-dimensional sequence:");
    for row_index in 0..user_two_dimensional_sequence.len() {
        for item_index in 0..user_two_dimensional_sequence[row_index].len() {
            print!("{} ", user_two_dimensional_sequence[row_index][item_index]);
        }
        println!("");
    }

    user_two_dimensional_sequence
}
