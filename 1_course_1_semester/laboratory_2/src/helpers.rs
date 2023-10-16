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

fn validate_input_string_by_student_id_with_grades(string: &String) -> bool {
    let re = Regex::new(r"^[\w-]+ [\d ]+$").unwrap();
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

fn get_input_user_student() -> (String, Vec<i8>) {
    loop {
        println!("Enter a row of student id with grades separated by whitespace. Example: 1a8a9ea3-24e2-4975-aa10-b73ae1a822e7 5 3 4 4 4 4 4 0 2 4:");
        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).expect("An incorrect row was entered. Sample: 1a8a9ea3-24e2-4975-aa10-b73ae1a822e7 5 3 4 4 4 4 4 0 2 4");
        user_input = String::from(user_input.trim());
        user_input = user_input.replace("  ", " ");

        if validate_input_string_by_student_id_with_grades(&user_input) {
            break convert_string_to_user_with_grades(user_input)
        } else {
            println!("You need to type number of students and number of backlogs");
        }
    }
}

pub fn get_input_user_student_list(student_count: usize) -> HashMap<String, Vec<i8>> {
    let mut student_list: HashMap<String, Vec<i8>> = HashMap::new();

    for index in 0..student_count {
        println!("Student {}", index + 1);

        let (user_id, grades) = get_input_user_student();
        student_list.insert(user_id, grades);
    };

    student_list
}

pub fn get_input_user_selection_conditions() -> (usize, usize) {
    let user_input = loop {
        let mut user_input = String::new();
        println!("Enter a number of students and number of backlogs separated by whitespace. Example: 5 1:");
        io::stdin().read_line(&mut user_input).expect("An incorrect typed data. Sample: 5 1");
        user_input = String::from(user_input.trim());
        user_input = user_input.replace("  ", " ");

        if validate_input_string_by_sequence_of_digits(&user_input) {
            break user_input;
        } else {
            println!("You need to type number of students and number of backlogs");
        }
    };

    let user_input = convert_input_to_vector(user_input);
    (user_input[0] as usize, user_input[1] as usize)
}

pub fn get_input_user_selection_condition() -> usize {
    let user_input = loop {
        let mut user_input = String::new();
        println!("Enter a number of backlogs separated by whitespace. Example: 5:");
        io::stdin().read_line(&mut user_input).expect("An incorrect typed data. Sample: 5");
        user_input = String::from(user_input.trim());
        user_input = user_input.replace("  ", " ");

        if validate_input_string_by_sequence_of_digits(&user_input) {
            break user_input;
        } else {
            println!("You need to type number of backlogs");
        }
    };

    let user_input = convert_input_to_vector(user_input);
    user_input[0] as usize
}

fn read_lines<P>(file_name: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(file_name)?;
    Ok(io::BufReader::new(file).lines())
}

fn convert_string_to_user_with_grades(row: String) -> (String, Vec<i8>) {
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

pub fn get_backlog_student_list(student_list: HashMap<String, Vec<i8>>, backlog_values: [i8; 2], min_block_numbers: usize) -> HashMap<String, Vec<i8>> {
    let mut backlog_student_list: HashMap<String, Vec<i8>> = HashMap::new();

    for (student_id, grades) in student_list {
        let backlog_grades = grades.into_iter().filter(|&grade| backlog_values.contains(&grade)).collect::<Vec<_>>();
        if backlog_grades.len() >= min_block_numbers {
            backlog_student_list.insert(student_id, backlog_grades);
        }
    }

    backlog_student_list
}
