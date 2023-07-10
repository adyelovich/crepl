use crate::parser::Expr;

pub fn interp(exprs: Vec<Expr>) -> () {
    for expr in exprs {
        match expr {
            Expr::CInt(i) => println!("{}", i),
            Expr::Empty => panic!("encoutered Empty expression while interpreting!"),
        }
    }
}
