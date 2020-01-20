use std::env;

use crate::math::Context;

mod lexer;
mod math;
mod parser;

fn main() {
    for s in env::args().skip(1) {
        println!("{}", s);
        let tokens = lexer::lex(&s);
        let expr = parser::parse(tokens);
        let context = Context::default();
        println!("  = {}", expr.eval(&context).unwrap());
    }
}
