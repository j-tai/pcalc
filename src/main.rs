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
    eprintln!("{}: {}", span, err);
}

fn run_expr(expr: &str, ctx: &mut Context, line: u32) {
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
    ctx.vars.insert(format!("ans{}", line), val.clone());
    println!("  ans{:<4} = {}", line, ctx.display(&val));
    if let Some(v) = ctx.vars.get_mut("ans") {
        *v = val;
    } else {
        ctx.vars.insert("ans".to_string(), val);
    }
}

fn main() {
    let mut context = Context::default();

    let mut did_calc = false;
    let mut line_num = 1;
    for s in env::args().skip(1) {
        println!("{}", s);
        run_expr(&s, &mut context, line_num);
        did_calc = true;
        line_num += 1;
    }

    if !did_calc {
        // Run REPL
        let stdin = io::stdin();
        for line in BufReader::new(stdin).lines() {
            let line = line.unwrap();
            run_expr(&line, &mut context, line_num);
            line_num += 1;
        }
    }
}
