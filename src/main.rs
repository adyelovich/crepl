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
            Ok(0) => break,
            Ok(_) => interp(Expr::parse(Token::tokenize(&line))),
            Err(e) => {
                eprintln!("encoutered error: {}", e);
                break;
            }
        }
    }
}
