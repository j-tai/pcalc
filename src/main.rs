use std::env;
use std::io;
use std::io::{BufRead, BufReader};

use pcalc::{eval, lex, parse, Context};

fn main() {
    let mut context = Context::default();

    let mut did_calc = false;
    for s in env::args().skip(1) {
        println!("{}", s);
        let tokens = lex(&s);
        let expr = parse(tokens);
        let val = eval(&expr, &mut context).unwrap();
        println!("  = {}", context.display(val));
        did_calc = true;
    }

    if !did_calc {
        // Run REPL
        let stdin = io::stdin();
        for line in BufReader::new(stdin).lines() {
            let line = line.unwrap();
            let tokens = lex(&line);
            let expr = parse(tokens);
            let val = eval(&expr, &mut context).unwrap();
            println!("  = {}", context.display(val));
        }
    }
}
