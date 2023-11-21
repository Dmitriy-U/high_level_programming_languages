fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    for item in args {
        let test = match item.parse::<f32>() {
            Some(x) => {x},
            None => {}
        };
        println!("{}", number);
    }
}
