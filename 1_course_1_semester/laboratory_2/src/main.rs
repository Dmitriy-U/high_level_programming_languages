mod helpers;

use std::collections::HashMap;
use crate::helpers::{
    get_backlog_student_list,
    get_data_from_attributes,
    get_input_user_selection_condition,
    get_input_user_selection_conditions,
    get_input_user_student_list,
};

const BACKLOG_VALUES: [i8; 2] = [0, 2];

fn main() {
    let mut student_list: HashMap<String, Vec<i8>> = HashMap::new();

    match get_data_from_attributes() {
        Some(list) => {
            student_list = list
        }
        None => {}
    }

    let min_block_numbers = if student_list.len() == 0 {
        let (n, k) = get_input_user_selection_conditions();
        student_list = get_input_user_student_list(n);
        k
    } else {
        get_input_user_selection_condition()
    };

    let backlog_student_list = get_backlog_student_list(student_list, BACKLOG_VALUES, min_block_numbers);

    match backlog_student_list.len() {
        0 => { println!("There are no backlog students"); }
        _ => {
            println!("Result:");
            for (student_id, _) in backlog_student_list {
                println!("{student_id}");
            }
        }
    }
}
