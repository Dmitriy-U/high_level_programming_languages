use crate::helpers::{
    check_two_dimensional_sequence_by_sorting_direction,
    get_user_two_dimensional_sequence,
    input_sorting_direction,
    SortingDirection,
};

mod helpers;

fn main() {
    let two_dimensional_sequence = get_user_two_dimensional_sequence();

    let sorting_direction = input_sorting_direction();

    let wrong_row_indexes = match sorting_direction {
        SortingDirection::ASC => check_two_dimensional_sequence_by_sorting_direction(two_dimensional_sequence, SortingDirection::ASC),
        SortingDirection::DESC => check_two_dimensional_sequence_by_sorting_direction(two_dimensional_sequence, SortingDirection::DESC)
    };

    if wrong_row_indexes.len() > 0 {
        for wrong_row_index in wrong_row_indexes {
            println!("Wrong row index: {wrong_row_index}");
        }
    } else {
        println!("Not found wrong row indexes");
    }
}
