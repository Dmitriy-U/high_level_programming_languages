use std::env::Args;

pub fn is_operator(char: String) -> bool {
    if char == "+" {
        true
    } else if char == "-" {
        true
    } else if char == "*" {
        true
    } else if char == "/" {
        true
    } else {
        false
    }
}

pub fn get_prepared_expression(args: Args) -> Option<Vec<String>> {
    let args: Vec<String> = args.skip(1).collect();

    if args.is_empty() {
        return None;
    }

    let mut prepared_chars: Vec<String> = Vec::new();
    let chars: Vec<char> = args[0].chars().collect();
    for char in chars {
        let char_string = char.to_string();
        if char_string == " " { continue; }
        if char_string == "(" {
            match prepared_chars.len() {
                0 => { () }
                n => {
                    if !is_operator(prepared_chars[n - 1].to_string()) {
                        prepared_chars.push(String::from("*"));
                    }
                    ()
                }
            };
        }
        prepared_chars.push(char_string);
        println!("{}", char);
    }

    return Some(prepared_chars);
}
