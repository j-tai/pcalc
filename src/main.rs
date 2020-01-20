use std::env;

use crate::math::Context;

mod lexer;
mod math;
mod parser;

fn main() {
    let context = Context::default();
    for s in env::args().skip(1) {
        println!("{}", s);
        let tokens = lexer::lex(&s);
        let expr = parser::parse(tokens);
        let val = expr.eval(&context).unwrap();
        println!("  = {}", context.display(val));
    }
}
