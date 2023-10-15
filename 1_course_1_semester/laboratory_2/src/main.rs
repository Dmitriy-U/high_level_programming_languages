mod helpers;

use std::collections::HashMap;
use crate::helpers::{get_data_from_attributes, get_input_user_selection_conditions};

fn main() {
    let examBacklogValues: [i8;2] = [0, 2];
    let mut student_list: HashMap<String, Vec<i8>> = HashMap::new();

    match get_data_from_attributes() {
        Some(list) => {
            student_list = list
        }
        None => {}
    }

    let (n, k) = get_input_user_selection_conditions();

    // TODO: Complete loop typing of student list

    // TODO: Complete student list sorting
}
