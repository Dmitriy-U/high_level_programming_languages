mod helpers;

use crate::helpers::{get_expression, get_prepared_string, validate_string};

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

    let expression = get_expression(&prepared_string);
    // TODO: Calculate
}
