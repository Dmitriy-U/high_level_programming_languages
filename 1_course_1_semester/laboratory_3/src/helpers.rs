use regex::{Regex};
use std::env::Args;

enum Operator {
    PLUS,
    MINUS,
    MULTIPLY,
    DIVIDE,
}

#[derive(Debug)]
enum Expr {
    Const(f64),
    Var(char),
    Unary(Box<Expr>),
    Binary(Operator, Box<Expr>, Box<Expr>),
}

enum ItemStack {
    Expr,
    Operator,
}

fn get_operator(sub_char: char) -> ItemStack::Operator {
    match sub_char {
        '+' => Operator::PLUS,
        '-' => Operator::MINUS,
        '*' => Operator::MULTIPLY,
        '/' => Operator::DIVIDE,
        _ => Operator::PLUS
    }
}

fn is_operator_char(sub_char: char) -> bool {
    match Regex::new(r"[\+\-\*/]").unwrap().is_match(&String::from(sub_char)) {
        true => { true }
        false => { false }
    }
}

fn is_variable_char(sub_char: char) -> bool {
    match Regex::new(r"[a-zA-Z]").unwrap().is_match(&String::from(sub_char)) {
        true => { true }
        false => { false }
    }
}

fn is_number_char(sub_char: char) -> bool {
    match Regex::new(r"[0-9\.]").unwrap().is_match(&String::from(sub_char)) {
        true => { true }
        false => { false }
    }
}

pub fn get_prepared_string(args: Args) -> Option<String> {
    let args: Vec<String> = args.skip(1).collect();

    if args.is_empty() {
        return None;
    }

    let trimmed_string = args[0].to_string().replace(" ", "");

    // TODO: -(x) => -1*(x)
    // TODO: --1 => +1

    return Some(trimmed_string);
}

pub fn validate_string(string: &String) -> bool {
    // check too many points in digits
    let result = match Regex::new(r"\d+\.\d+\.").unwrap().find(string) {
        None => { true }
        Some(found_too_many_points) => {
            println!("Found too many points: \"{}\"", found_too_many_points.as_str());
            false
        }
    };
    if !result { return false; }

    // check too many operators
    let result = match Regex::new(r"[\+\-\*\/]{3}").unwrap().find(string) {
        None => { true }
        Some(found_double_match) => {
            println!("More operators: \"{}\"", found_double_match.as_str());
            false
        }
    };
    if !result { return false; }

    // check brackets
    let bracket_open_length: usize = string.matches('(').collect::<Vec<_>>().len();
    let bracket_close_length: usize = string.matches('(').collect::<Vec<_>>().len();
    if bracket_open_length != bracket_close_length {
        println!("Chars \"(\" and \")\" not satisfied");
        return false;
    }

    true
}

fn get_expression(&string: &String, index: usize) {
    let mut items: Vec<ItemStack> = Vec::new();
    let mut buffer_number: String = String::from("");
    let mut i = index;

    while i <= string.len {
        let sub_char: char = string[i];

        // if it is number then add to buffer
        if is_number_char(sub_char) {
            buffer_number.push(sub_char);
            i += 1;
            continue;
        }

        // end parsing number
        if !buffer_number.is_empty() {
            items.push(ItemStack::Expr::Const(buffer_number.parse().unwrap()));
            buffer_number = String::new();
        }

        if is_variable_char(sub_char) {
            items.push(ItemStack::Expr::Var(sub_char));
            i += 1;
            continue;
        }

        if is_operator_char(sub_char) {
            items.push(get_operator(sub_char));
            i += 1;
            continue;
        }

        if sub_char == '(' {
            let (expr, newIndex) = get_expression(&string, i + 1);
            items.push(expr);
            i = newIndex;
            continue;
        }

        if sub_char == ')' {
            break;
        }
    }

    // TODO: Sorting items by priority and return
}
