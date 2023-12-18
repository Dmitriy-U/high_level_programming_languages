use regex::{Regex};
use std::env::{Args};
use std::io;

#[derive(Debug, Clone)]
enum Expr {
    Const(f64),
    Var(String),
    Unary(Box<Expr>),
    Binary(char, Box<Expr>, Box<Expr>),
}

fn merge_low_priority_operators(operator_1: char, operator_2: char) -> char {
    match (operator_1, operator_2) {
        ('+', '+') => { '+' }
        ('+', '-') => { '-' }
        ('-', '+') => { '-' }
        ('-', '-') => { '+' }
        _ => { '+' }
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

fn is_variable(operator: &String) -> bool {
    match Regex::new(r"[a-zA-Z]").unwrap().is_match(operator) {
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

fn is_operator_low_priority(operator: &String) -> bool {
    match Regex::new(r"[\+\-]").unwrap().is_match(operator) {
        true => { true }
        false => { false }
    }
}

fn is_operator_low_priority_char(operator: char) -> bool {
    match Regex::new(r"[\+\-]").unwrap().is_match(&operator.to_string()) {
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

fn is_number(string: &String) -> bool {
    string.parse::<f64>().is_ok()
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

    (result, i)
}

fn get_input_user_value(variable: &String) -> String {
    loop {
        println!("Enter a value (float) for variable {variable}:");
        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).expect("An incorrect variable was entered");
        user_input = String::from(user_input.trim());

        if is_number(&user_input) {
            println!("You choose: {variable} = {user_input}");
            break user_input
        } else {
            println!("You need to type correct value of variable");
        }
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
            let previous_char = chars[i - 1].clone();
            if is_number_char(previous_char) || previous_char == ')' {
                prepared_string.push('*');
            }
        }

        prepared_string.push(sub_char);
        i += 1;
    }

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

    // check double incorrect operators like "1-/1"
    let result = match Regex::new(r"[\+\-]{1}[/\*]{1}|\-\+").unwrap().find(string) {
        None => { true }
        Some(found_incorrect_double_operators) => {
            println!("Double incorrect operators: \"{}\"", found_incorrect_double_operators.as_str());
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

    if buffer.len() > 0 {
        nodes.push(buffer.clone());
        buffer = String::new();
    }

    return nodes;
}

pub fn replace_double_operators(string: String) -> String {
    let mut result_string: String = String::new();
    let mut i: usize = 0;
    let string_items: Vec<char> = string.chars().collect();
    let string_length: usize = string_items.len();

    while i < string_length {
        let string_item: char = string_items[i].clone();

        if is_operator_low_priority_char(string_item) {
            if i + 1 != string_length {
                let string_item_next: char = string_items[i + 1].clone();

                if is_operator_low_priority_char(string_item_next) {
                    let operator = merge_low_priority_operators(string_item, string_item_next);
                    result_string.push(operator);
                    i += 2;
                    continue;
                }
            }
        }

        result_string.push(string_item);

        i += 1;
    }

    result_string
}

pub fn change_variables(nodes: &mut Vec<String>) {
    for node in nodes.iter_mut() {
        if is_variable(node) {
            let value = get_input_user_value(node);
            *node = value;
        }
    }
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
