use std::env;

use pcalc::{eval, lex, parse, Context};

fn main() {
    let context = Context::default();
    for s in env::args().skip(1) {
        println!("{}", s);
        let tokens = lex(&s);
        let expr = parse(tokens);
        let val = eval(&expr, &context).unwrap();
        println!("  = {}", context.display(val));
    }
}
