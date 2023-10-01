mod helpers;

use crate::helpers::{convert_input_to_vector, get_user_input_string, sort};

fn main() {
    let user_input_string = get_user_input_string();
    let mut user_input_vector = convert_input_to_vector(user_input_string);
    sort(&mut user_input_vector);
    // TODO: Вывести для каждого уникального элемента массива процент от общего количества элементов
}
