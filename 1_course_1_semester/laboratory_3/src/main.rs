mod helpers;

use crate::helpers::get_prepared_expression;

#[derive(Debug)]
enum Expr {
    Const(f64),
    Var(String),
    Unary(Box<Expr>),
    Binary(char, Box<Expr>, Box<Expr>),
}

fn main() {
    // let test: Expr = Expr::Binary(
    //     '*',
    //     Box::new(Expr::Const(1.0)),
    //     Box::new(Expr::Binary(
    //         '+',
    //         Box::new(Expr::Const(2.0)),
    //         Box::new(Expr::Const(2.0)),
    //     )));
    // println!("{:?}", test);
    let expression = get_prepared_expression(std::env::args());

    if expression.is_none() {
        println!("You did not write a math expression: Example: ./laboratory_3 \"1 + 1 (2 + 2)\"");
        return;
    }

    println!("Prepared expression: {:?}", expression.unwrap());
}
