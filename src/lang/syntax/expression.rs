use std::{fmt::Display, process::Output, vec};

use pepser::{
    errors::ParserError,
    impls::{sequence, ws},
    traits::{discard, opt, sep_by, value, wrapped, Input, ParseResult, Parser},
};

macro_rules! bin_op{
    ($function:ident, $input:ident, ($first_seq:expr, $first_binop:expr) $(,($seq:expr, $binop:expr))*) => {
        {
            let or_combinator = value($first_binop, sequence($first_seq));

            $(
                let or_combinator = or_combinator.or(value($binop, sequence($seq)));
            )*

            let (ipt, initial) = $function.parse($input)?;
            let (ipt, vector) = wrapped(
                ws(),
                or_combinator,
                ws(),
            ).and($function)
            .many()
            .parse(ipt)?;

            Ok((ipt, fold_expressions(initial, vector)))
        }
    };
}

fn fold_expressions(initial: Expr, vector: Vec<(BinaryOperator, Expr)>) -> Expr {
    vector.into_iter().fold(initial, |acc, (op, expr)| {
        Expr::Binary(op, Box::new(acc), Box::new(expr))
    })
}
use super::lexical_grammar::{identifier, num_literal, string_literal};

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

#[derive(Clone, Debug)]
pub enum UnaryOperator {
    Neg,
    Bang,
}

impl Display for UnaryOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UnaryOperator::Neg => write!(f, "-"),
            UnaryOperator::Bang => write!(f, "!"),
        }
    }
}
#[derive(Clone, Debug)]
pub enum BinaryOperator {
    Mul,
    Add,
    Div,
    Sub,
    Or,
    Eq,
    Neq,
    Leq,
    Lt,
    Geq,
    Gt,
}
impl Display for BinaryOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BinaryOperator::Mul => write!(f, "*"),
            BinaryOperator::Add => write!(f, "+"),
            BinaryOperator::Div => write!(f, "/"),
            BinaryOperator::Sub => write!(f, "-"),
            BinaryOperator::Or => write!(f, "||"),
            BinaryOperator::Eq => write!(f, "=="),
            BinaryOperator::Neq => write!(f, "!=="),
            BinaryOperator::Leq => write!(f, "<="),
            BinaryOperator::Lt => write!(f, "<"),
            BinaryOperator::Geq => write!(f, ">="),
            BinaryOperator::Gt => write!(f, ">"),
        }
    }
}
pub fn parse_language<'a>(input: &'a str) -> ParseResult<&'a str, Vec<Expr>> {
    wrapped(ws(), sep_by(expression, sequence(";")), ws()).parse(input)
}

pub fn expression<'a>(input: &'a str) -> ParseResult<&'a str, Expr> {
    assignement.parse(input)
}

pub fn assignement<'a>(input: &'a str) -> ParseResult<&'a str, Expr> {
    identifier
        .and(discard(wrapped(ws(), sequence("="), ws()), assignement))
        .map(|(id, expr)| Expr::Assignement(id, Box::new(expr)))
        .or(logic_or)
        .parse(input)
}

pub fn logic_or<'a>(input: &'a str) -> ParseResult<&'a str, Expr> {
    bin_op!(logic_and, input, ("||", BinaryOperator::Or))
}

pub fn logic_and<'a>(input: &'a str) -> ParseResult<&'a str, Expr> {
    bin_op!(equality, input, ("&&", BinaryOperator::Or))
}

pub fn equality<'a>(input: &'a str) -> ParseResult<&'a str, Expr> {
    bin_op!(
        comparison,
        input,
        ("==", BinaryOperator::Eq),
        ("!==", BinaryOperator::Neq)
    )
}

pub fn comparison<'a>(input: &'a str) -> ParseResult<&'a str, Expr> {
    bin_op!(
        term,
        input,
        ("<", BinaryOperator::Lt),
        ("<=", BinaryOperator::Leq),
        (">", BinaryOperator::Gt),
        (">=", BinaryOperator::Geq)
    )
}

pub fn term<'a>(input: &'a str) -> ParseResult<&'a str, Expr> {
    bin_op!(
        factor,
        input,
        ("-", BinaryOperator::Sub),
        ("+", BinaryOperator::Add)
    )
}

pub fn factor<'a>(input: &'a str) -> ParseResult<&'a str, Expr> {
    bin_op!(
        unary,
        input,
        ("/", BinaryOperator::Div),
        ("*", BinaryOperator::Mul)
    )
}

pub fn unary<'a>(input: &'a str) -> ParseResult<&'a str, Expr> {
    wrapped(
        ws(),
        value(UnaryOperator::Bang, sequence("!"))
            .or(value(UnaryOperator::Neg, sequence("-")))
            .and(unary)
            .map(|(operator, expr)| Expr::Unary(operator, Box::new(expr)))
            .or(call),
        ws(),
    )
    .parse(input)
}

pub fn call<'a>(input: &'a str) -> ParseResult<&'a str, Expr> {
    wrapped(
        ws(),
        primary
            .and(opt(discard(
                wrapped(ws(), sequence(":>"), ws()),
                sep_by(expression, wrapped(ws(), sequence("::"), ws())),
            )))
            .map(|(primary, args_opt)| {
                args_opt.map_or(primary.clone(), |args: Vec<Expr>| {
                    Expr::Call(Box::new(primary), args)
                })
            }),
        ws(),
    )
    .parse(input)
}

pub fn primary<'a>(input: &'a str) -> ParseResult<&'a str, Expr> {
    wrapped(
        ws(),
        wrapped(sequence("("), expression, sequence(")"))
            .or(value(Expr::Bool(true), sequence("true")))
            .or(value(Expr::Bool(false), sequence("false")))
            .or(num_literal)
            .or(string_literal)
            .or(identifier.map(Expr::Ident))
            .or(expression),
        ws(),
    )
    .parse(input)
}

pub fn non_empty<P, I: Input>(mut parser: P) -> impl FnMut(I) -> ParseResult<I, P::Output>
where
    P: Parser<I>,
{
    move |input: I| {
        if input.input_len() == 0 {
            return Err(ParserError::new(
                0,
                pepser::errors::ErrorSource::Many,
                "empty",
            ));
        }

        parser.parse(input)
    }
}
