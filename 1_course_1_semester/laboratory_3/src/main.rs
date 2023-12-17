mod helpers;

use crate::helpers::{calculate_vector_brackets, get_prepared_string, validate_string, get_nodes, calculate_vector_without_brackets, change_variables};

fn main() {
    let prepared_string = get_prepared_string(std::env::args());

    if prepared_string.is_none() {
        println!("You did not write a math expression: Example: ./laboratory_3 \"1 + 1 (2 + 2)\"");
        return;
    }

    // TODO: validate double variable is_operator_low_priority(index + 1)
    let prepared_string = prepared_string.unwrap();
    if !validate_string(&prepared_string) {
        return;
    }

    println!("Prepared the calculation string: {:?}", prepared_string);
    let mut nodes = get_nodes(&prepared_string);
    change_variables(&mut nodes);
    let nodes_without_brackets = calculate_vector_brackets(nodes);
    let result = calculate_vector_without_brackets(nodes_without_brackets);
    println!("Result: {result}");
}
