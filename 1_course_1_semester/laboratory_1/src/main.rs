mod helpers;

use crate::helpers::{convert_input_to_vector, get_user_input_string};

fn main() {
    let user_input_string = get_user_input_string();
    let user_input_vector = convert_input_to_vector(user_input_string);

    println!("{:?}", user_input_vector);
}
