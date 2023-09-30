use std::io;
use regex::Regex;

fn validate_input(string: &String) -> bool {
    let re = Regex::new(r"^[\d ]+$").unwrap();
    let match_string = &string[..];
    re.is_match(match_string)
}

fn main() -> io::Result<()> {
    let mut user_input = String::new();
    println!("Введите последовательность цифр:");
    io::stdin().read_line(&mut user_input).expect("Не введёна корректная последовательность цифр");
    user_input = String::from(user_input.trim());
    println!("Введено: {}", user_input);
    assert!(validate_input(&user_input));
    Ok(())
}
