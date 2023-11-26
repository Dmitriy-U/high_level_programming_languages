use regex::{Regex};
use std::env::Args;

const REGEX_OPERATOR: Regex = Regex::new(r"[\+\-\*/]").unwrap();

const REGEX_MANY_OPERATORS: Regex = Regex::new(r"[\+\-\*/]{3}").unwrap();

pub fn is_operator(char: String) -> bool {
    match REGEX_OPERATOR.is_match(&char) {
        true => {true}
        false => {false}
    }
}

// pub fn get_prepared_expression(args: Args) -> Option<Vec<String>> {
//     let args: Vec<String> = args.skip(1).collect();
//
//     if args.is_empty() {
//         return None;
//     }
//
//     let mut prepared_chars: Vec<String> = Vec::new();
//     let chars: Vec<char> = args[0].chars().collect();
//     for char in chars {
//         let char_string = char.to_string();
//         if char_string == " " { continue; }
//         if char_string == "(" {
//             match prepared_chars.len() {
//                 0 => { () }
//                 n => {
//                     if !is_operator(prepared_chars[n - 1].to_string()) {
//                         prepared_chars.push(String::from("*"));
//                     }
//                     ()
//                 }
//             };
//         }
//         prepared_chars.push(char_string);
//         println!("{}", char);
//     }
//
//     return Some(prepared_chars);
// }

pub fn validate_double_operators(string_args: &String) -> bool {
    let result = match Regex::new(r"[\+\-\*/]{3}").unwrap().find(&string_args) {
        None => { true }
        Some(found_double_match) => {
            println!("More operators: \"{}\"", found_double_match.as_str());
            false
        }
    };

    return result;
}

pub fn get_prepared_expression(args: Args) -> Option<Vec<String>> {
    let args: Vec<String> = args.skip(1).collect();

    if args.is_empty() {
        return None;
    }

    let trimmed_string = args[0].to_string().replace(" ", "");
    let check_double_operators_result = validate_double_operators(&trimmed_string);
    if !check_double_operators_result {
        return None;
    }

    return Some(prepared_chars);
}
