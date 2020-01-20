use std::env;

pub use crate::context::*;
pub use crate::error::*;
pub use crate::interpreter::*;
pub use crate::lexer::*;
pub use crate::parser::*;

mod context;
mod error;
mod interpreter;
mod lexer;
mod parser;

fn main() {
    let context = Context::default();
    for s in env::args().skip(1) {
        println!("{}", s);
        let tokens = lexer::lex(&s);
        let expr = parser::parse(tokens);
        let val = eval(&expr, &context).unwrap();
        println!("  = {}", context.display(val));
    }
}
