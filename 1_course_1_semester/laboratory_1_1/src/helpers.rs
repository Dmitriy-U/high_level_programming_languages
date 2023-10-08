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

pub fn sort(sequence: &mut Vec<i32>) {
    let mut index_min: usize = 0;
    let mut index_temp: i32 = 0;
    let length = sequence.len();

    for index in 0..length {
        index_min = index;

        for index_in_slice in (index + 1)..length {
            if sequence[index_in_slice] < sequence[index_min] {
                index_min = index_in_slice;
            }
        }

        if index != index_min {
            index_temp = sequence[index];
            sequence[index] = sequence[index_min];
            sequence[index_min] = index_temp;
        }

        let count = index + 1;
        println!("Iteration {} {:?}", count, sequence);
    }
}
