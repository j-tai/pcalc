use std::env;
use std::io;
use std::io::{BufRead, BufReader};

use pcalc::{eval, lex, parse, Context, Error, Span};

fn show_err(err: Error, span: Span) {
    for _ in 1..span.start {
        eprint!(" ");
    }
    for _ in span.start..=span.end {
        eprint!("^");
    }
    eprintln!();
    eprintln!("{}: {}", err, span);
}

fn run_expr(expr: &str, ctx: &mut Context) {
    if expr.is_empty() {
        return;
    }
    let tokens = lex(&expr, None);
    let expr = match parse(tokens) {
        Ok(e) => e,
        Err((err, span)) => {
            show_err(err, span);
            return;
        }
    };
    let val = match eval(&expr, ctx) {
        Ok(e) => e,
        Err((err, span)) => {
            show_err(err, span);
            return;
        }
    };
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
