/*
In C, there are both expressions and statements.
A statement must have a semicolon, and optionally an expression.

I think at the repl, I am thinking that it should print the result of the
expression if I just write an expression, but also do the same for a statement.
Maybe only one expression per line, and then allow multiple statements?


I am thinking that I should return a list of statements to allow multiple
statements in one line.

In the 330 code there was only one statement, so it was easy to keep on parsing,
here I am thinking that we should instead parse a bunch of statements and eat
things along the way (use a VecDeque), and then each time we hit a semicolon,
or end of statement, we go again. If there is still something in the
VecDeque by the end then treat it as a statement, and then if there is 
something else after that, panic.

*/

use crate::lexer::Token;
use std::collections::VecDeque;

#[derive(PartialEq, Debug)]
pub enum Expr {
    Empty,
    CInt(i32),
}

/*
#[derive(PartialEq, Debug)]
struct Error<'a> {
    kind: ErrorKind,
    msg: &'a str,
}

#[derive(PartialEq, Debug)]
enum ErrorKind {
    MultExpr,
}

impl Error {
    pub fn new(kind: ErrorKind, msg: &str) -> Error {
        Error {
            kind,
            msg,
        }
    }
}
*/
/* So far an arbitrary number of statements are allowed, followed by
   an optional expression */
impl Expr {
    pub fn parse_statement(mut tokens: VecDeque<Token>) -> Vec<Expr> {
        //let mut ret: Option<Error> = None;
        let mut statements: Vec<Expr> = Vec::new();

        while tokens.len() > 0 {
            if let Some(e) = Self::parse_expr(&mut tokens) {
                if let Some(front) = tokens.front() {
                    match front {
                        Token::TokSemicolon => {
                            tokens.pop_front();
                            statements.push(e);
                        },
                        _ => {
                            if tokens.len() == 0 {
                                statements.push(e);
                            } else {
                                panic!("too many expressions given!");
                            }
                        }
                    }
                } else {
                    if tokens.len() == 0 {
                        statements.push(e);
                    } else {
                        panic!("too many expressions given!");
                    }
                }
            } else {
                tokens.pop_front();
            }
        }

        statements
    }
    
    // fn parse_expr(mut tokens: VecDeque<Token>) -> Vec<Expr> {
    //     let mut statements: Vec<Expr> = Vec::new();

    //     while tokens.len() > 0 {
    //         if let Some(e) = Self::build_expr(&mut tokens) {
    //             statements.push(e);
    //         }
    //     }
        
    //     statements
    // }

    fn parse_expr(tokens: &mut VecDeque<Token>) -> Option<Expr> {
        let mut expr = Expr::Empty;

        if let Some(token) = tokens.pop_front() {
            match token {
                Token::TokInt(i) => expr = Expr::CInt(i),
                Token::TokSemicolon => {
                    tokens.push_front(token);
                }
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
        assert_eq![Expr::parse_statement(VecDeque::from([])), Ok(vec![])];
    }
    
    #[test]
    fn parse_int32_semicolon() {
        assert_eq![Expr::parse_statement(VecDeque::from([TokInt(10), TokSemicolon])),
                   Ok(vec![Expr::CInt(10)])];
    }

    #[test]
    fn parse_int32_nosemicolon() {
        assert_eq![Expr::parse_statement(VecDeque::from([TokInt(-42)])), Ok(vec![Expr::CInt(-42)])];
    }

    #[test]
    fn parse_int32_one_semicolon() {
        assert_eq![Expr::parse_statement(VecDeque::from([TokInt(24), TokSemicolon, TokInt(-77)])),
                   Ok(vec![Expr::CInt(24), Expr::CInt(-77)])];
    }

    #[test]
    fn parse_int32_many() {
        assert_eq![Expr::parse_statement(VecDeque::from([TokInt(10), TokSemicolon,
                                    TokInt(-42), TokSemicolon,
                                    TokInt(3333), TokSemicolon,
                                    TokSemicolon,
                                    TokSemicolon])),
                   Ok(vec![Expr::CInt(10), Expr::CInt(-42), Expr::CInt(3333)])];
    }

    #[test]
    #[should_panic(expected = "too many")]
    fn parse_int32_expr_many() {
        Expr::parse_statement(VecDeque::from([TokInt(10), TokInt(12)]));
    }
    
}
