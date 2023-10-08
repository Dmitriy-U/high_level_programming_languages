use crate::helpers::{get_user_two_dimensional_sequence};

// fn check_x_number(two_dimensional_sequence: Vec<Vec<i32>>) -> bool {
//     let x_number_value: i32 = two_dimensional_sequence[0][1];
//     let mut is_x_number = true;
//
//     for item_index in 0..two_dimensional_sequence[1].len() {
//         if two_dimensional_sequence[1][item_index] != x_number_value {
//             is_x_number = false;
//             break;
//         }
//     }
//
//     if is_x_number == false { is_x_number }
//
//     if two_dimensional_sequence[2][1] != x_number_value { false }
//
//     is_x_number
// }

fn check_x_number(two_dimensional_sequence: Vec<Vec<i32>>) -> Vec<usize> {
    let mut top_item_of_x_number_index: Vec<usize> = Vec::new();

    for row_index in 0..(two_dimensional_sequence.len() - 2) {
        let item_length = two_dimensional_sequence[row_index].len();
        for item_index in 0..(item_length - 1) {
            // Pass first item
            if item_index == 0 { continue; }

            let x_number_value = two_dimensional_sequence[row_index][item_index];
            println!("Try row index: {row_index}; item index: {item_index}; x-number value: {x_number_value}");
            let row_index_next = row_index + 1;
            // Check nex row count
            if two_dimensional_sequence[row_index_next].len() < (item_index + 2) { continue; }

            // Check x-number values
            if (two_dimensional_sequence[row_index_next][item_index - 1] == x_number_value)
                && (two_dimensional_sequence[row_index_next][item_index] == x_number_value)
                && (two_dimensional_sequence[row_index_next][item_index + 1] == x_number_value)
                && (two_dimensional_sequence[row_index_next + 1][item_index] == x_number_value) {
                top_item_of_x_number_index.push(item_index);
            }
        }
    }

    top_item_of_x_number_index
}

pub fn do_laboratory_1_2() {
    println!("Laboratory 1.2");
    let user_two_dimensional_sequence = get_user_two_dimensional_sequence(3);
    let top_item_of_x_number_index = check_x_number(user_two_dimensional_sequence);
    println!("{} {}", top_item_of_x_number_index.len(), top_item_of_x_number_index[0]);
}
