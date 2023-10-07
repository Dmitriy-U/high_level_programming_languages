use crate::helpers::{get_user_sequence_of_digits};

fn sort(sequence: &mut Vec<i32>) {
    let mut index_min: usize = 0;
    let mut index_temp: i32 = 0;
    let length = sequence.len();

    for index in 0..length {
        index_min = index;

        for index_in_slice in (index + 1)..length {
            if sequence[index_in_slice] < sequence[index_min] {
                index_min = index_in_slice;
            }
        }

        if index != index_min {
            index_temp = sequence[index];
            sequence[index] = sequence[index_min];
            sequence[index_min] = index_temp;
        }

        let count = index + 1;
        println!("Iteration {count} {:?}", sequence);
    }
}

pub fn do_laboratory_1_1() {
    println!("--== Laboratory 1.2 ==--");
    let mut user_input_vector = get_user_sequence_of_digits();
    sort(&mut user_input_vector);
}
