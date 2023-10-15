use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::path::Path;
use std::io::{self, BufRead};
use regex::Regex;

fn validate_input_string_by_sequence_of_digits(string: &String) -> bool {
    let re = Regex::new(r"^[\d ]+$").unwrap();
    let match_string = &string[..];
    re.is_match(match_string)
}

fn validate_input_string_by_sorting_direction(string: &String) -> bool {
    let re = Regex::new(r"^(asc|desc)$").unwrap();
    let match_string = &string[..];
    re.is_match(match_string)
}

fn validate_input_string_by_boolean(string: &String) -> bool {
    let re = Regex::new(r"^(y|n)$").unwrap();
    let match_string = &string[..];
    re.is_match(match_string)
}

pub fn convert_input_to_vector(string_source: String) -> Vec<i32> {
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

pub enum SortingDirection {
    ASC,
    DESC,
}

pub fn input_sorting_direction() -> SortingDirection {
    loop {
        let mut user_input = String::new();
        println!("Enter a sorting_direction. \"asc\" or \"desc\":");
        io::stdin().read_line(&mut user_input).expect("An incorrect sequence of integers was entered. Sample: \"asc\" or \"desc\"");
        user_input = String::from(user_input.trim());

        if validate_input_string_by_sorting_direction(&user_input) {
            break if user_input == String::from("asc") { SortingDirection::ASC } else { SortingDirection::DESC };
        } else {
            println!("You need to type \"asc\" or \"desc\"");
        }
    }
}

pub fn get_input_user_selection_conditions() -> (i32, i32) {
    let user_input = loop {
        let mut user_input = String::new();
        println!("Enter a number of students and number of exam backlogs separated by whitespace. Example: 5 1:");
        io::stdin().read_line(&mut user_input).expect("An incorrect mockup.txt count and debt separated was entered. Sample: 5 1");
        user_input = String::from(user_input.trim());
        user_input.replace("  ", " ");

        // TODO: Add check elements count "n" and "k"
        if validate_input_string_by_sequence_of_digits(&user_input) {
            break user_input;
        } else {
            println!("You need to type number of students and number of exam backlogs");
        }
    };

    let user_input = convert_input_to_vector(user_input);
    (user_input[0], user_input[1])
}

fn get_user_sequence_of_integers(min_length: usize) -> Vec<i32> {
    let user_input_string = get_user_input_string();
    let input_vector = convert_input_to_vector(user_input_string);

    if input_vector.len() < min_length {
        println!("Please, type sequence with {min_length} integers minimum1");
        get_user_sequence_of_integers(min_length)
    } else {
        input_vector
    }
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

pub fn get_user_two_dimensional_sequence() -> Vec<Vec<i32>> {
    let mut user_two_dimensional_sequence: Vec<Vec<i32>> = Vec::new();

    let mut row: usize = 1;
    loop {
        println!("{row} row:");
        let user_sequence_of_digits = get_user_sequence_of_integers(1);
        user_two_dimensional_sequence.push(user_sequence_of_digits);

        if !input_need_to_continue() { break; }

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

fn check_sequence_by_sorting_direction(
    sequence: &Vec<i32>,
    sorting_direction: &SortingDirection,
) -> bool {
    let mut temp: i32 = 0;
    let mut result = true;

    for item_index in 0..sequence.len() {
        if item_index == 0 {
            temp = sequence[item_index];
            continue;
        }

        let condition_result = match sorting_direction {
            SortingDirection::ASC => sequence[item_index] > temp,
            SortingDirection::DESC => sequence[item_index] < temp
        };

        if !condition_result {
            result = false;
            break;
        }
    }

    result
}

pub fn check_two_dimensional_sequence_by_sorting_direction(
    two_dimensional_sequence: Vec<Vec<i32>>,
    sorting_direction: SortingDirection,
) -> Vec<usize> {
    let mut wrong_row_indexes: Vec<usize> = Vec::new();

    for row_index in 0..two_dimensional_sequence.len() {
        if !check_sequence_by_sorting_direction(&two_dimensional_sequence[row_index], &sorting_direction) {
            wrong_row_indexes.push(row_index);
        }
    }

    wrong_row_indexes
}

pub fn read_lines<P>(file_name: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(file_name)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn convert_string_to_user_with_grades(row: String) -> (String, Vec<i8>) {
    let mut grades: Vec<i8> = Vec::new();
    let mut file_row_split: Vec<&str> = row.split(" ").collect();
    let student_id = file_row_split[0];
    file_row_split.remove(0);
    for string_item in file_row_split {
        grades.push(string_item.to_string().parse().unwrap());
    }
    (student_id.to_string(), grades)
}

pub fn get_data_from_attributes() -> Option<HashMap<String, Vec<i8>>> {
    match env::args().nth(1) {
        Some(file_path) => {
            match read_lines(file_path) {
                Ok(file_lines) => {
                    let mut students: HashMap<String, Vec<i8>> = HashMap::new();
                    for file_line in file_lines {
                        if let Ok(file_row) = file_line {
                            let (student_id, grades) = convert_string_to_user_with_grades(file_row);
                            students.insert(student_id, grades);
                        }
                    }
                    Some(students)
                }
                _ => None
            }
        }
        _ => None
    }
}