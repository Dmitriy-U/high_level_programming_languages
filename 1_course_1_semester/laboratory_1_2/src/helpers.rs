use std::io;
use regex::Regex;

fn validate_input_string_by_sequence_of_digits(string: &String) -> bool {
    let re = Regex::new(r"^[\d ]+$").unwrap();
    let match_string = &string[..];
    re.is_match(match_string)
}

fn validate_input_string_by_boolean(string: &String) -> bool {
    let re = Regex::new(r"^(y|n)$").unwrap();
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

fn input_need_to_continue() -> bool {
    loop {
        println!("Need to continue. Type \"y\" for yes or \"n\" for no:");
        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).expect("An incorrect was entered. Sample: \"y\" or \"n\"");
        user_input = String::from(user_input.trim());

        if validate_input_string_by_boolean(&user_input) {
            break user_input == String::from("y");
        } else {
            println!("You need to type \"y\" or \"n\"");
        }
    }
}

pub fn get_user_sequence_of_integers(min_length: usize) -> Vec<i32> {
    let user_input_string = get_user_input_string();
    let input_vector = convert_input_to_vector(user_input_string);

    if input_vector.len() < min_length {
        println!("Please, type sequence with {min_length} integers minimum1");
        get_user_sequence_of_integers(min_length)
    } else {
        input_vector
    }
}

pub fn get_user_two_dimensional_sequence() -> Vec<Vec<i32>> {
    let mut user_two_dimensional_sequence: Vec<Vec<i32>> = Vec::new();

    let mut row: usize = 1;
    loop {
        println!("{row} row:");
        let user_sequence_of_digits = get_user_sequence_of_integers(3);
        user_two_dimensional_sequence.push(user_sequence_of_digits);

        if row >= 3 && !input_need_to_continue() { break; }

        row += 1;
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


pub fn check_x_number(two_dimensional_sequence: Vec<Vec<i32>>) -> Vec<i32> {
    let mut top_item_of_x_number_index: Vec<i32> = Vec::new();

    for row_index in 0..(two_dimensional_sequence.len() - 2) {
        let items_length = two_dimensional_sequence[row_index].len();
        let items_length_next_row = two_dimensional_sequence[row_index + 1].len();
        let item_index_max = if items_length_next_row > items_length {
            items_length
        } else {
            items_length_next_row - 1
        };

        for item_index in 0..item_index_max {
            // Pass first item
            if item_index == 0 { continue; }

            let x_number_value = two_dimensional_sequence[row_index][item_index];
            let row_index_next = row_index + 1;
            // Check nex row count
            if two_dimensional_sequence[row_index_next].len() < (item_index + 2) { continue; }

            // Check x-number values
            if (two_dimensional_sequence[row_index_next][item_index - 1] == x_number_value)
                && (two_dimensional_sequence[row_index_next][item_index] == x_number_value)
                && (two_dimensional_sequence[row_index_next][item_index + 1] == x_number_value)
                && (two_dimensional_sequence[row_index_next + 1][item_index] == x_number_value) {
                top_item_of_x_number_index.push(two_dimensional_sequence[row_index][item_index]);
            }
        }
    }

    top_item_of_x_number_index
}
