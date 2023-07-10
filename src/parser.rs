/*
In C, there are both expressions and statements.
A statement must have a semicolon, and optionally an expression.

I think at the repl, I am thinking that it should print the result of the
expression if I just write an expression, but also do the same for a statement.
Maybe only one expression per line, and then allow multiple statements?


I am thinking that I should return a list of statements to allow multiple
statements in one line.
*/

use crate::lexer::Token;
use std::collections::VecDeque;

#[derive(PartialEq, Debug)]
pub enum Expr {
    Empty,
    CInt(i32),
}

/* So far an arbitrary number of statements are allowed, followed by
   an optional expression */
impl Expr {
    pub fn parse(mut tokens: VecDeque<Token>) -> Vec<Expr> {
        let mut statements: Vec<Expr> = Vec::new();

        while tokens.len() > 0 {
            if let Some(e) = Self::build_expr(&mut tokens) {
                statements.push(e);
            }
        }
        
        statements
    }

    fn build_expr(tokens: &mut VecDeque<Token>) -> Option<Expr> {
        let mut expr = Expr::Empty;
        
        loop {
            if let Some(token) = tokens.pop_front() {
                match token {
                    Token::TokInt(i) => expr = Expr::CInt(i),
                    Token::TokSemicolon => break,
                }
            } else {
                break;
            }
        }

        if expr == Expr::Empty { None } else { Some(expr) }
    }
}


#[cfg(test)]
mod tests {
    use std::collections::VecDeque;
    use crate::lexer::Token::*;
    use crate::parser::Expr;

    #[test]
    fn parse_nothing() {
        assert_eq![Expr::parse(VecDeque::from([])), vec![]];
    }
    
    #[test]
    fn parse_int32_semicolon() {
        assert_eq![Expr::parse(VecDeque::from([TokInt(10), TokSemicolon])),
                   vec![Expr::CInt(10)]];
    }

    #[test]
    fn parse_int32_nosemicolon() {
        assert_eq![Expr::parse(VecDeque::from([TokInt(-42)])), vec![Expr::CInt(-42)]];
    }

    #[test]
    fn parse_int32_one_semicolon() {
        assert_eq![Expr::parse(VecDeque::from([TokInt(24), TokSemicolon, TokInt(-77)])),
                   vec![Expr::CInt(24), Expr::CInt(-77)]];
    }

    #[test]
    fn parse_int32_many() {
        assert_eq![Expr::parse(VecDeque::from([TokInt(10), TokSemicolon,
                                    TokInt(-42), TokSemicolon,
                                    TokInt(3333), TokSemicolon,
                                    TokSemicolon,
                                    TokSemicolon])),
                   vec![Expr::CInt(10), Expr::CInt(-42), Expr::CInt(3333)]];
    }
    
}
