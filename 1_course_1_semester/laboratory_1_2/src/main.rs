mod helpers;

use crate::helpers::{check_x_number, get_user_two_dimensional_sequence};

fn main() {
    let user_two_dimensional_sequence = get_user_two_dimensional_sequence();
    let top_item_of_x_number_index = check_x_number(user_two_dimensional_sequence);

    let x_number_count = top_item_of_x_number_index.len();
    if x_number_count > 0 {
        println!("Found {} x-number", top_item_of_x_number_index.len());
        for item in top_item_of_x_number_index {
            print!("{item} ");
        }
        println!("");
    } else {
        println!("X-number not found");
    }
}
