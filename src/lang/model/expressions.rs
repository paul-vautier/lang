use std::{borrow::Borrow, fmt::Display, mem};

use super::{
    env::Env,
    operators::{BinaryOperator, UnaryOperator},
};
#[derive(Clone)]
pub enum Value {
    NoVal,
    Number(f64),
    Boolean(bool),
    String(String),
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::NoVal => write!(f, "()"),
            Value::Number(n) => write!(f, "{}", n),
            Value::Boolean(b) => write!(f, "{}", b),
            Value::String(s) => write!(f, "{}", s),
        }
    }
}

impl Value {
    pub fn apply_binop(self, operator: BinaryOperator, other: Value) -> Result<Value, InterpError> {
        if let Value::NoVal = self {
            return Err(InterpError::value(InterpErrorReason::InvalidBinop(
                self, other, operator,
            )));
        }
        if let Value::NoVal = other {
            return Err(InterpError::value(InterpErrorReason::InvalidBinop(
                self, other, operator,
            )));
        }
        Ok(match (self.clone(), other.clone(), operator.clone()) {
            (Value::Number(first), Value::Number(second), BinaryOperator::Mul) => {
                Value::Number(first * second)
            }
            (Value::Number(first), Value::Number(second), BinaryOperator::Add) => {
                Value::Number(first + second)
            }
            (Value::Number(first), Value::Number(second), BinaryOperator::Div) => {
                Value::Number(first / second)
            }
            (Value::Number(first), Value::Number(second), BinaryOperator::Sub) => {
                Value::Number(first - second)
            }
            (Value::Number(first), Value::Number(second), BinaryOperator::Or) => {
                Value::Number(first * second)
            }
            (Value::Number(first), Value::Number(second), BinaryOperator::Eq) => {
                Value::Boolean(first == second)
            }
            (Value::Number(first), Value::Number(second), BinaryOperator::Neq) => {
                Value::Boolean(first != second)
            }
            (Value::Number(first), Value::Number(second), BinaryOperator::Leq) => {
                Value::Boolean(first <= second)
            }
            (Value::Number(first), Value::Number(second), BinaryOperator::Lt) => {
                Value::Boolean(first < second)
            }
            (Value::Number(first), Value::Number(second), BinaryOperator::Geq) => {
                Value::Boolean(first <= second)
            }
            (Value::Number(first), Value::Number(second), BinaryOperator::Gt) => {
                Value::Boolean(first < second)
            }
            (Value::Number(first), Value::String(second), BinaryOperator::Mul) => {
                Value::String(second.repeat(first as usize))
            }
            (Value::Number(first), Value::String(second), BinaryOperator::Add) => {
                Value::String(format!("{}{}", first, second))
            }
            (Value::Boolean(first), Value::Boolean(second), BinaryOperator::Or) => {
                Value::Boolean(first || second)
            }
            (Value::Boolean(first), Value::Boolean(second), BinaryOperator::Eq) => {
                Value::Boolean(first == second)
            }
            (Value::Boolean(first), Value::Boolean(second), BinaryOperator::Neq) => {
                Value::Boolean(first != second)
            }
            (Value::Boolean(first), Value::Boolean(second), BinaryOperator::Leq) => {
                Value::Boolean(first <= second)
            }
            (Value::Boolean(first), Value::Boolean(second), BinaryOperator::Lt) => {
                Value::Boolean(first < second)
            }
            (Value::Boolean(first), Value::Boolean(second), BinaryOperator::Geq) => {
                Value::Boolean(first >= second)
            }
            (Value::Boolean(first), Value::Boolean(second), BinaryOperator::Gt) => {
                Value::Boolean(first > second)
            }
            (Value::String(first), Value::Number(second), BinaryOperator::Mul) => {
                Value::String(first.repeat(second as usize))
            }
            (Value::String(first), Value::Number(second), BinaryOperator::Add) => {
                Value::String(format!("{}{}", first, second))
            }
            (Value::String(first), Value::String(second), BinaryOperator::Add) => {
                Value::String(format!("{}{}", first, second))
            }
            (Value::String(first), Value::String(second), BinaryOperator::Eq) => {
                Value::Boolean(first == second)
            }
            (Value::String(first), Value::String(second), BinaryOperator::Neq) => {
                Value::Boolean(first != second)
            }
            _ => {
                return Err(InterpError::value(InterpErrorReason::InvalidBinop(
                    self, other, operator,
                )))
            }
        })
    }

    pub fn is_truthy(&self) -> Result<bool, InterpError> {
        match self {
            Value::NoVal => Err(InterpError::value(InterpErrorReason::NoValOperation)),
            Value::Number(num) => Ok(*num != 0.0),
            Value::Boolean(bool) => Ok(*bool),
            Value::String(str) => Ok(str.len() != 0),
        }
    }
}

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

impl Expr {
    pub fn interp(&self, env: &mut Env) -> Result<Value, InterpError> {
        match self {
            Expr::Str(str) => Ok(Value::String(str.clone())),
            Expr::Value(f64) => Ok(Value::Number(f64.clone())),
            Expr::Bool(bool) => Ok(Value::Boolean(bool.clone())),
            Expr::Assignement(id, value) => {
                let interpreted = value.interp(env)?;
                if !env.assign(id.clone(), interpreted.clone()) {
                    return Err(InterpError::expr(
                        Expr::Assignement(id.clone(), value.clone()),
                        InterpErrorReason::UndeclaredVar(id.clone()),
                    ));
                }

                return Ok(interpreted);
            }
            Expr::Binary(op, first, sec) => {
                first.interp(env)?.apply_binop(op.clone(), sec.interp(env)?)
            }
            Expr::Ident(id) => env.get_ident_value(id),
            Expr::Unary(_, _) => todo!(),
            Expr::Call(_, _) => todo!(),
        }
    }
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
                args.iter().fold(String::new(), |acc, arg| format!(
                    "{acc}{}{arg}",
                    if acc.is_empty() { "" } else { ", " }
                ))
            ),
        }
    }
}

pub struct InterpError {
    source: InterpErrorSource,
    reason: InterpErrorReason,
}

pub enum InterpErrorSource {
    ExprErr(Expr),
    ValueErr,
}
impl InterpError {
    pub fn expr(source: Expr, reason: InterpErrorReason) -> Self {
        InterpError {
            source: InterpErrorSource::ExprErr(source),
            reason,
        }
    }
    pub fn value(reason: InterpErrorReason) -> Self {
        InterpError {
            source: InterpErrorSource::ValueErr,
            reason,
        }
    }
}
pub enum InterpErrorReason {
    UndeclaredVar(String),
    InvalidUnop(Value),
    InvalidBinop(Value, Value, BinaryOperator),
    NoValOperation,
}
