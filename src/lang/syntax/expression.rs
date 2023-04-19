use std::process::Output;

use pepser::{
    impls::{none_of, sequence, ws},
    traits::{discard, opt, value, wrapped, ParseResult, Parser},
};

use super::lexical_grammar::{identifier, num_literal, string_literal};

#[derive(Debug, Clone)]
pub enum Expr {
    Str(String),
    Value(f64),
    Assignement(String, Box<Expr>),
    Binary(BinaryOperator, Box<Expr>, Box<Expr>),
    Or,
    And,
    Eq,
    Comp,
    Term,
    Factor,
    Unary,
    Call,
    Primary,
}
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
    And,
}

pub fn next<'a>(input: &'a str) -> ParseResult<&'a str, Expr> {
    ws().map(|_| Expr::Call).parse(input)
}

pub fn parse_language<'a>(input: &'a str) -> ParseResult<&'a str, Expr> {
    wrapped(ws(), expression, ws()).parse(input)
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

#[rustfmt::skip]
pub fn logic_or<'a>(
    input: &'a str
) -> ParseResult<&'a str, Expr>
{
    logic_and.and(
        opt(
            wrapped(
                ws(),
                value(BinaryOperator::Or, sequence("||")),
                ws(),
                ).and(logic_and)))
        .map(|(expr, opt)| {
            opt.map_or(expr.clone(), |(binop, other_expr)| {
                Expr::Binary(binop, Box::new(expr), Box::new(other_expr))
            })
        })
        .parse(input)
}

#[rustfmt::skip]
pub fn logic_and<'a>(
    input: &'a str
) -> ParseResult<&'a str, Expr>
{
    equality.and(
        opt(
            wrapped(
                ws(),
                value(BinaryOperator::Or, sequence("||")),
                ws(),
                ).and(equality)))
        .map(|(expr, opt)| {
            opt.map_or(expr.clone(), |(binop, other_expr)| {
                Expr::Binary(binop, Box::new(expr), Box::new(other_expr))
            })
        })
        .parse(input)
}

#[rustfmt::skip]
pub fn equality<'a>(
    input: &'a str
) -> ParseResult<&'a str, Expr>
{
    comparison.and(
        opt(
            wrapped(
                ws(),
                value(BinaryOperator::Or, sequence("||")),
                ws(),
                ).and(comparison)))
        .map(|(expr, opt)| {
            opt.map_or(expr.clone(), |(binop, other_expr)| {
                Expr::Binary(binop, Box::new(expr), Box::new(other_expr))
            })
        })
        .parse(input)
}

#[rustfmt::skip]
pub fn comparison<'a>(
    input: &'a str
) -> ParseResult<&'a str, Expr>
{
    term.and(
        opt(
            wrapped(
                ws(),
                value(BinaryOperator::Or, sequence("||")),
                ws(),
                ).and(term)))
        .map(|(expr, opt)| {
            opt.map_or(expr.clone(), |(binop, other_expr)| {
                Expr::Binary(binop, Box::new(expr), Box::new(other_expr))
            })
        })
        .parse(input)
}

#[rustfmt::skip]
pub fn term<'a>(
    input: &'a str
) -> ParseResult<&'a str, Expr>
{
    equality.and(
        opt(
            wrapped(
                ws(),
                value(BinaryOperator::Or, sequence("||")),
                ws(),
                ).and(equality)))
        .map(|(expr, opt)| {
            opt.map_or(expr.clone(), |(binop, other_expr)| {
                Expr::Binary(binop, Box::new(expr), Box::new(other_expr))
            })
        })
        .parse(input)
}

#[rustfmt::skip]
pub fn factor<'a>(
    input: &'a str
) -> ParseResult<&'a str, Expr>
{
    equality.and(
        opt(
            wrapped(
                ws(),
                value(BinaryOperator::Or, sequence("||")),
                ws(),
                ).and(equality)))
        .map(|(expr, opt)| {
            opt.map_or(expr.clone(), |(binop, other_expr)| {
                Expr::Binary(binop, Box::new(expr), Box::new(other_expr))
            })
        })
        .parse(input)
}
