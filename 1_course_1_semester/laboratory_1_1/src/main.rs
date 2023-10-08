mod helpers;

use crate::helpers::{get_user_sequence_of_digits, sort};

fn main() {
    let mut user_input_vector = get_user_sequence_of_digits();
    sort(&mut user_input_vector);
}
