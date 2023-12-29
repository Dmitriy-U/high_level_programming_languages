mod helpers;

use std::collections::HashMap;
use crate::helpers::{calculate_vector_brackets, get_prepared_string, validate_string, get_nodes, calculate_vector_without_brackets, change_variables, replace_double_operators};

fn main() {
    let prepared_string = get_prepared_string(std::env::args());

    if prepared_string.is_none() {
        println!("You did not write a math expression: Example: ./laboratory_3 \"1 + 1 (2 + 2)\"");
        return;
    }

    let prepared_string = prepared_string.unwrap();
    if !validate_string(&prepared_string) {
        return;
    }

    let prepared_string = replace_double_operators(prepared_string);

    println!("Prepared the calculation string: {:?}", prepared_string);

    let mut nodes = get_nodes(&prepared_string);
    println!("{nodes:?}");
    change_variables(&mut nodes);

    let nodes_without_brackets = calculate_vector_brackets(nodes);
    let result = calculate_vector_without_brackets(nodes_without_brackets);

    println!("Result: {result}");
}
