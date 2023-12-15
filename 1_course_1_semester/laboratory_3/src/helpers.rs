use regex::{Regex};
use std::env::Args;

enum Operator {
    PLUS,
    MINUS,
    MULTIPLY,
    DIVIDE,
}

#[derive(Debug, Clone)]
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
    let result = match expression {
        Expr::Binary(operator, exp_1, exp_2) => {
            let x = *exp_1;
            let y = *exp_2;
            let x = match x {
                Expr::Const(val) => { Some(val) }
                _ => { None }
            };
            let y = match y {
                Expr::Const(val) => { Some(val) }
                _ => { None }
            };
            let x = x.unwrap();
            let y = y.unwrap();
            let result = match operator {
                '+' => { x + y }
                '-' => { x - y }
                '*' => { x * y }
                '/' => { x / y }
                _ => { x + y }
            };
            Some(result)
        }
        _ => { None }
    };
    return result.unwrap().to_string();
}

fn calculate_simple_binary(value_1: String, operator: char, value_2: String) -> String {
    let value_1: f64 = value_1.parse().unwrap_or(0.0);
    let value_1: Expr = Expr::Const(value_1);
    let value_2: f64 = value_2.parse().unwrap_or(0.0);
    let value_2: Expr = Expr::Const(value_2);

    calculate_expression(Expr::Binary(operator, Box::new(value_1), Box::new(value_2)))
}

fn calculate_high_priority_expression(nodes: &Vec<String>) -> Vec<String> {
    // TODO: add variable
    let mut result_nodes: Vec<String> = Vec::new();

    let mut i: usize = 0;
    let nodes_len = nodes.len();
    while i < nodes_len {
        let node = nodes[i].clone();

        if is_operator_high_priority(&node) {
            let calculated_string = calculate_simple_binary(
                result_nodes.pop().unwrap_or("0.0".to_string()),
                node.chars().nth(0).unwrap(),
                nodes[i + 1].clone(),
            );
            result_nodes.push(calculated_string);
            i += 2;
            continue;
        }

        result_nodes.push(node);

        i += 1;
    }

    return result_nodes;
}

fn calculate_low_priority_expression(nodes: Vec<String>) -> f64 {
    // TODO: add variable
    let mut result: f64 = 0.0;

    let mut i: usize = 0;
    let nodes_len = nodes.len();
    while i < nodes_len {
        let node = nodes[i].clone();

        if is_operator(&node) {
            if i == 1 {
                let calculated_string = calculate_simple_binary(nodes[i - 1].clone(), node.chars().nth(0).unwrap(), nodes[i + 1].clone());
                result = calculated_string.parse::<f64>().unwrap();
            } else {
                let calculated_string = calculate_simple_binary(result.to_string(), node.chars().nth(0).unwrap(), nodes[i + 1].clone());
                result = calculated_string.parse::<f64>().unwrap();
            }
            i += 2;
            continue;
        }

        i += 1;
    }

    return result;
}

pub fn get_nodes(string: &String) -> Vec<String> {
    let mut nodes: Vec<String> = Vec::new();
    let mut buffer: String = String::new();

    let mut i: usize = 0;
    let chars = string.chars().collect::<Vec<char>>();
    let len = chars.len();
    while i < len {
        let sub_char: char = chars[i];

        if sub_char == '(' || sub_char == ')' {
            if buffer.len() > 0 {
                nodes.push(buffer.clone());
                buffer = String::new();
            }
            nodes.push(sub_char.to_string());
            i += 1;
            continue;
        }

        if sub_char == '-' && is_number_char(chars[i + 1]) {
            if is_number_char(buffer.chars().last().unwrap_or('+')) {
                nodes.push(buffer);
                buffer = String::new();
                nodes.push('+'.to_string());
            }
            buffer.push(sub_char);
            i += 1;
            continue;
        }

        if is_number_char(sub_char) {
            buffer.push(sub_char);
            if i == len - 1 {
                nodes.push(buffer);
                buffer = String::new();
            }
            i += 1;
            continue;
        }

        if is_operator_char(sub_char) {
            if buffer.len() > 0 {
                nodes.push(buffer.clone());
                buffer = String::new();
            }
            nodes.push(sub_char.to_string());
            i += 1;
            continue;
        }

        buffer.push(sub_char);

        i += 1;
    }

    return nodes;
}

fn calculate_vector_between_brackets(nodes: &Vec<String>, index: usize) -> (f64, usize) {
    let mut result_vector: Vec<String> = Vec::new();
    let mut i: usize = index;

    while i < nodes.len() {
        let node = nodes[i].clone();

        if node == '('.to_string() {
            let (result_nested, new_index) = calculate_vector_between_brackets(nodes, i + 1);
            result_vector.push(result_nested.to_string());
            i = new_index;
            continue;
        }

        if node == ')'.to_string() {
            i += 1;
            break;
        }

        result_vector.push(node);

        i += 1;
    }

    let calculated_nodes = calculate_high_priority_expression(&result_vector);
    let result = calculate_low_priority_expression(calculated_nodes);

    return (result, i);
}

pub fn calculate_vector_brackets(nodes: Vec<String>) -> Vec<String> {
    let mut result_vector: Vec<String> = Vec::new();
    let mut i: usize = 0;

    while i < nodes.len() {
        let node = nodes[i].clone();

        if node == '('.to_string() {
            let (result, new_index) = calculate_vector_between_brackets(&nodes, i + 1);

            result_vector.push(result.to_string());

            i = new_index;
            continue;
        }

        result_vector.push(node);

        i += 1;
    }

    return result_vector;
}

pub fn calculate_vector_without_brackets(nodes: Vec<String>) -> f64 {
    let calculated_nodes = calculate_high_priority_expression(&nodes);
    calculate_low_priority_expression(calculated_nodes)
}
