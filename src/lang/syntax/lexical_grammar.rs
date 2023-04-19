use pepser::{
    impls::{any, none_of, sequence, take_while},
    traits::{discard, opt, parse_if, value, wrapped, ParseResult, Parser},
};

const IDENTIFIER_CHARSET: &str = "qwertyuiopasdfghjklzxcvbnm_QWERTYUIOPASDFGHJKLZXCVBNM";

const RESERVED_WORD: &[&str] = &["if", "else", ""];

use super::Expr;

#[rustfmt::skip]
pub fn num_literal<'a>(input: &'a str) -> ParseResult<&'a str, Expr> {
    opt(sequence("-"))
        .map(|opt| if opt.is_some() { -1 } else { 1 })
        .and(integral_part)
        .and(decimal_part)
        .and(exponent)
        .map(|(((sign, integral), decimal), exponent)| Expr::Value(calculate_number(sign, integral, decimal, exponent)))
        .parse(input)
}

fn calculate_number(sign: i64, integral: u64, decimal: f64, exponent: i32) -> f64 {
    (sign as f64 * (integral as f64 + decimal)).powi(exponent)
}
#[rustfmt::skip]
fn integral_part<'a>(input: &'a str) -> ParseResult<&'a str, u64> {
    sequence("0")
                .or(digits)
                .map(str::parse::<u64>)
                .map(Result::unwrap).parse(input)
}

#[rustfmt::skip]
fn decimal_part<'a>(input: &'a str) -> ParseResult<&'a str, f64> {
    parse_if(sequence("."), digits).map(|opt| {
        opt.map(|double_str| format!("0.{}", double_str).parse::<f64>())
            .map(Result::unwrap)
            .unwrap_or(0.0)
    }).parse(input)
}

#[rustfmt::skip]
fn exponent<'a>(input: &'a str) -> ParseResult<&'a str, i32> {
    opt(discard(any("eE"), 
    opt(
            value(-1, sequence("-")).or(value(1 as i32, sequence("+")
            ))).map(|opt| opt.unwrap_or(1))
        ).and(digits).map(|(a, b)| a * b.parse::<i32>().unwrap())
    ).map(|opt| opt.unwrap_or(1))
    .parse(input)
}

pub fn digits<'a>(input: &'a str) -> ParseResult<&'a str, &'a str> {
    take_while(|c| c.is_digit(10)).parse(input)
}

#[rustfmt::skip]
pub fn identifier<'a>(input: &'a str) -> ParseResult<&'a str, String> {
    any(IDENTIFIER_CHARSET)
        .and(
            opt(
                any(IDENTIFIER_CHARSET)
                .or(digits)
                .many()
                .map(Vec::into_iter)
                .map(Iterator::collect::<String>)
            ).map(|opt| opt.unwrap_or("".to_string()))
        )
        .map(|(c, rest)| format!("{}{}", c, rest))
        .parse(input)
}

pub fn string_literal<'a>(input: &'a str) -> ParseResult<&'a str, Expr> {
    wrapped(
        sequence("\""),
        none_of("\"\\")
            .or(sequence("\\\"").map(|_| "\""))
            .many()
            .map(|vec| Expr::Str(vec.into_iter().collect::<String>())),
        sequence("\""),
    )
    .parse(input)
}
