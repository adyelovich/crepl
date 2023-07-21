mod lexer;
mod parser;
mod interp;

use std::io::{stdout, stdin, Write};
use crate::lexer::Token;
use crate::parser::Expr;
use crate::interp::interp;

fn main() {
    println!("Welcome to crepl v0.0.1 (`int` Build)!");

    loop {
        let mut line = String::new();
        
        let _ = stdout().write(b"> ");
        let _ = stdout().flush();

        match stdin().read_line(&mut line) {
            Ok(0) => {
                let _ = stdout().write(b"\n");
                let _ = stdout().flush();
                break;
            },
            Ok(_) => process_line(&line),
            Err(e) => {
                eprintln!("encoutered I/O error: {}", e);
                break;
            }
        }
    }
}

fn process_line(line: &str) -> () {
    let tokens;
    let exprs;
    
    
    match Token::tokenize(&line) {
        Ok(v) => tokens = v,
        Err(e) => {
            eprintln!("Lex error: {}", e);
            return;
        }
    }

    match Expr::parse_statement(tokens) {
        Ok(v) => exprs = v,
        Err(e) => {
            eprintln!("parse error: {}", e);
            return;
        }
    }

    interp(exprs);
}
