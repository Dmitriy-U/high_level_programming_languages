mod helpers;

use crate::helpers::{calculate_brackets, get_prepared_string, validate_string, get_nodes, calculate_without_brackets};

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

    println!("Prepared expression: {:?}", prepared_string);
    let nodes = get_nodes(&prepared_string);
    let nodes_without_brackets = calculate_brackets(nodes);
    let result = calculate_without_brackets(nodes_without_brackets);
    println!("Result: {result}");
}
