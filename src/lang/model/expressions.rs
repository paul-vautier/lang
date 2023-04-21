use std::fmt::Display;

use super::operators::{BinaryOperator, UnaryOperator};

#[derive(Debug, Clone)]
pub enum Expr {
    Str(String),
    Value(f64),
    Bool(bool),
    Assignement(String, Box<Expr>),
    Binary(BinaryOperator, Box<Expr>, Box<Expr>),
    Ident(String),
    Unary(UnaryOperator, Box<Expr>),
    Call(Box<Expr>, Vec<Expr>),
}

impl Expr {}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Str(str) => write!(f, "{}", str),
            Expr::Value(val) => write!(f, "{}", val),
            Expr::Bool(boolean) => write!(f, "{}", boolean),
            Expr::Assignement(str, expr) => write!(f, "{} = {}", str, expr),
            Expr::Binary(operator, first, sec) => write!(f, "({} {} {})", first, operator, sec),
            Expr::Ident(str) => write!(f, "{}", str),
            Expr::Unary(op, expr) => write!(f, "{}{}", op, expr),
            Expr::Call(expr, args) => write!(
                f,
                "{} :> ({})",
                expr,
                args.iter()
                    .fold(String::new(), |acc, arg| format!("{} {} ", acc, arg))
            ),
        }
    }
}
