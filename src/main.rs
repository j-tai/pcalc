use std::env;
use std::fs;
use std::io;
use std::io::{BufRead, BufReader};
use std::process::exit;

use pcalc::{eval, lex, parse, Context, Error, Result, Span, Value};

fn show_err(err: Error, span: Span, arrow: bool) {
    if arrow {
        for _ in 1..span.start {
            eprint!(" ");
        }
        for _ in span.start..=span.end {
            eprint!("^");
        }
        eprintln!();
    }
    eprintln!("{}: {}", span, err);
}

fn run_expr(expr: &str, ctx: &mut Context, filename: Option<String>) -> Result<Value> {
    let tokens = lex(&expr, filename);
    let expr = parse(tokens)?;
    eval(&expr, ctx)
}

fn main() {
    let mut ctx = Context::default();

    // Read each file from command-line arguments
    for filename in env::args().skip(1) {
        eprintln!("Loading {}...", filename);
        let s = match fs::read_to_string(&filename) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("pcalc: cannot read {:?}: {}", filename, e);
                exit(1);
            }
        };
        match run_expr(&s, &mut ctx, Some(filename)) {
            Ok(_) => (),
            Err((e, s)) => show_err(e, s, false),
        }
    }
    eprintln!("Welcome to pcalc.");

    // Run REPL
    let stdin = io::stdin();
    let mut line_num = 1_u32;
    for line in BufReader::new(stdin).lines() {
        let line = line.unwrap();
        match run_expr(&line, &mut ctx, None) {
            Ok(v) => {
                let ans_name = format!("ans{}", line_num);
                println!("  ans{:<4} = {}", line_num, ctx.display(&v));
                ctx.vars.insert(ans_name, v.clone());
                if let Some(ans) = ctx.vars.get_mut("ans") {
                    *ans = v;
                } else {
                    ctx.vars.insert("ans".to_string(), v);
                }
                line_num += 1;
            }
            Err((e, s)) => show_err(e, s, true),
        }
    }
}
