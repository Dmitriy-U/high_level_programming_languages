mod helpers;

use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(file_name: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(file_name)?;
    Ok(io::BufReader::new(file).lines())
}


fn main() {
    match env::args().nth(1) {
        Some(file_path) => {
            if let Ok(lines) = read_lines(file_path) {
                // Получает итератор, который возвращает Option
                for line in lines {
                    if let Ok(ip) = line {
                        println!("{}", ip);
                    }
                }
            }
        }
        _ => {
            println!("None");
        }
    };
    // let (n, k) = get_input_user_data();
}
