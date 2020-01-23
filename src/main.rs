use std::env;
use std::io;
use std::io::{BufRead, BufReader};

use pcalc::{eval, lex, parse, Context};

fn run_expr(expr: &str, ctx: &mut Context) {
    if expr.is_empty() {
        return;
    }
    let tokens = lex(&expr, None);
    let expr = match parse(tokens) {
        Ok(e) => e,
        Err((err, span)) => {
            eprintln!("{}: {}", span, err);
            return;
        }
    };
    let val = eval(&expr, ctx).unwrap();
    println!("  = {}", ctx.display(val));
}

fn main() {
    let mut context = Context::default();

    let mut did_calc = false;
    for s in env::args().skip(1) {
        println!("{}", s);
        run_expr(&s, &mut context);
        did_calc = true;
    }

    if !did_calc {
        // Run REPL
        let stdin = io::stdin();
        for line in BufReader::new(stdin).lines() {
            let line = line.unwrap();
            run_expr(&line, &mut context);
        }
    }
}
