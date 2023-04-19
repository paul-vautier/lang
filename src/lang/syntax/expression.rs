use std::process::Output;

use pepser::{
    errors::ParserError,
    impls::{none_of, sequence, ws},
    traits::{discard, opt, sep_by, value, wrapped, Input, ParseResult, Parser},
};

macro_rules! bin_op{
    ($function:ident, ($first_seq:expr, $first_binop:expr) $(,($seq:expr, $binop:expr))*) => {
        {
            let or_combinator = value($first_binop, sequence($first_seq));

            $(
                let or_combinator = or_combinator.or(value($binop, sequence($seq)));
            )*

            $function.and(
                opt(
                    wrapped(
                        ws(),
                        or_combinator,
                        ws(),
                        ).and($function)))
                .map(|(expr, opt)| {
                    opt.map_or(expr.clone(), |(binop, other_expr)| {
                        Expr::Binary(binop, Box::new(expr), Box::new(other_expr))
                    })
                })
        }
    };
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

#[derive(Clone, Debug)]
pub enum UnaryOperator {
    Neg,
    Bang,
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

pub fn parse_language<'a>(input: &'a str) -> ParseResult<&'a str, Vec<Expr>> {
    wrapped(ws(), sep_by(non_empty(expression), sequence(";")), ws()).parse(input)
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
    bin_op!(logic_and, ("||", BinaryOperator::Or)).parse(input)
}

pub fn logic_and<'a>(input: &'a str) -> ParseResult<&'a str, Expr> {
    bin_op!(equality, ("&&", BinaryOperator::Or)).parse(input)
}

pub fn equality<'a>(input: &'a str) -> ParseResult<&'a str, Expr> {
    bin_op!(
        comparison,
        ("==", BinaryOperator::Eq),
        ("!==", BinaryOperator::Neq)
    )
    .parse(input)
}

pub fn comparison<'a>(input: &'a str) -> ParseResult<&'a str, Expr> {
    bin_op!(
        term,
        ("<", BinaryOperator::Lt),
        ("<=", BinaryOperator::Leq),
        (">", BinaryOperator::Gt),
        (">=", BinaryOperator::Geq)
    )
    .parse(input)
}

pub fn term<'a>(input: &'a str) -> ParseResult<&'a str, Expr> {
    bin_op!(
        factor,
        ("-", BinaryOperator::Sub),
        ("+", BinaryOperator::Add)
    )
    .parse(input)
}

pub fn factor<'a>(input: &'a str) -> ParseResult<&'a str, Expr> {
    bin_op!(
        unary,
        ("/", BinaryOperator::Div),
        ("*", BinaryOperator::Mul)
    )
    .parse(input)
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
            .map(|(primary, args)| Expr::Call(Box::new(primary), args.unwrap_or(vec![]))),
        ws(),
    )
    .parse(input)
}

pub fn primary<'a>(input: &'a str) -> ParseResult<&'a str, Expr> {
    wrapped(
        ws(),
        value(Expr::Bool(true), sequence("true"))
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
