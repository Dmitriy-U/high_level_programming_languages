use regex::{Regex};
use std::env::Args;

enum Operator {
    PLUS,
    MINUS,
    MULTIPLY,
    DIVIDE,
}

#[derive(Debug,Clone)]
enum Expr {
    Const(f64),
    Var(String),
    Unary(Box<Expr>),
    Binary(char, Box<Expr>, Box<Expr>),
}

fn get_operator(sub_char: char) -> Operator {
    match sub_char {
        '+' => Operator::PLUS,
        '-' => Operator::MINUS,
        '*' => Operator::MULTIPLY,
        '/' => Operator::DIVIDE,
        _ => Operator::PLUS
    }
}

fn is_operator_char(sub_char: char) -> bool {
    match Regex::new(r"[\+\-\*\/]").unwrap().is_match(&String::from(sub_char)) {
        true => { true }
        false => { false }
    }
}

fn is_operator(operator: &String) -> bool {
    match Regex::new(r"[\+\-\*\/]").unwrap().is_match(operator) {
        true => { true }
        false => { false }
    }
}

fn is_operator_high_priority_char(sub_char: char) -> bool {
    match Regex::new(r"[\*\/]").unwrap().is_match(&String::from(sub_char)) {
        true => { true }
        false => { false }
    }
}

fn is_operator_high_priority(operator: &String) -> bool {
    match Regex::new(r"[\*\/]").unwrap().is_match(operator) {
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

    let mut prepared_string = String::new();

    let chars = trimmed_string.chars().collect::<Vec<char>>();
    let mut i: usize = 0;

    while i < chars.len() {
        let sub_char = chars[i];

        if sub_char == '(' && i != 0 {
            if is_number_char(chars[i - 1].clone()) {
                prepared_string.push('*');
            }
        }

        prepared_string.push(sub_char);
        i += 1;
    }

    // TODO: --1 => +1

    return Some(prepared_string);
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

fn calculate_expression(expression: Expr) -> String {
    let result = match &expression {
        Expr::Binary(operator, exp_1, exp_2) if matches!(exp_1, Expr) && matches!(exp_2, Expr) => {
            println!("Expr -> {:?} - {:?}", exp_1, exp_2);
            Some(0) // TODO: to complete
            // match operator {
            //     '+' => { Some(x + y) }
            //     '-' => { Some(x - y) }
            //     '*' => { Some(x * y) }
            //     '/' => { Some(x / y) }
            //     _ => { Some(x + y) }
            // }
        }
        _ => { None }
    };

    return result.unwrap().to_string();
}

fn calculate_simple_binary(value_1: String, operator: char, value_2: String) -> String {
    let value_1: f64 = value_1.parse().unwrap();
    let value_1: Expr = Expr::Const(value_1);
    let value_2: f64 = value_2.parse().unwrap();
    let value_2: Expr = Expr::Const(value_2);

    calculate_expression(Expr::Binary(operator, Box::new(value_1), Box::new(value_2)))
}

fn calculate_high_priority_expression(nodes: Vec<String>) -> Vec<String> {
    // TODO: to complete
    // TODO: add unary operator
    let mut result_nodes: Vec<String> = Vec::new();
    let mut buffer_nodes: Vec<String> = Vec::new();

    let mut i: usize = 0;
    let nodes_len = nodes.len();
    while i < nodes_len {
        let node_item = nodes[i].clone();
        println!("{}", node_item);

        if is_operator_high_priority(&node_item) {
            let calculated_string = calculate_simple_binary(buffer_nodes.pop().unwrap(), node_item.chars().nth(0).unwrap(), nodes[i + 1].clone());
            result_nodes.append(&mut buffer_nodes);
            result_nodes.push(calculated_string);
            i += 2;
            continue;
        }

        buffer_nodes.push(node_item);

        i += 1;
    }

    result_nodes.append(&mut buffer_nodes);

    return result_nodes;
}

fn get_nodes(string: String) -> Vec<String> {
    let mut nodes: Vec<String> = Vec::new();
    let mut buffer: String = String::new();

    let mut i: usize = 0;
    let chars = string.chars().collect::<Vec<char>>();
    let len = chars.len();
    while i < len {
        let sub_char: char = chars[i];

        if is_number_char(sub_char) {
            buffer.push(sub_char);
            if i == len - 1 {
                nodes.push(buffer);
                buffer = String::new();
            }
        }

        if is_operator_char(sub_char) {
            nodes.push(buffer);
            nodes.push(sub_char.to_string());
            buffer = String::new();
        }

        i += 1;
    }

    return nodes;
}

fn calculate_brackets_part(string: &String, index: usize) -> (String, usize) {
    let mut result_string = String::new();
    let mut i: usize = index;

    let chars = string.chars().collect::<Vec<char>>();

    while i <= chars.len() {
        let sub_char: char = chars[i];
        println!("{} -> {}", i, result_string);

        if sub_char == '(' {
            let (result_string_nested, new_index) = calculate_brackets_part(&string, i + 1);
            result_string = format!("{}{}", result_string, result_string_nested);
            i = new_index;
        }

        if sub_char == ')' {
            i += 1;
            break;
        }

        result_string.push(sub_char);

        i += 1;
    }

    let nodes = get_nodes(result_string.clone());
    let result_nodes = calculate_high_priority_expression(nodes);
    println!("Expression in brackets {}", result_string);

    return (result_string, i);
}

pub fn calculate_brackets(string: String) -> String {
    let mut result_string = String::new();
    let mut i: usize = 0;

    let chars = string.chars().collect::<Vec<char>>();

    while i <= chars.len() {
        println!("{} -> {}", i, result_string);
        let sub_char: char = chars[i];

        if sub_char == '(' {
            let (result_string_nested, new_index) = calculate_brackets_part(&string, i + 1);
            result_string = format!("{}{}", result_string, result_string_nested);
            i = new_index;
            continue;
        }

        result_string.push(sub_char);

        i += 1;
    }

    return result_string;
}
