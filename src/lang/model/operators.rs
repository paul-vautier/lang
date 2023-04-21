use std::fmt::Display;

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

impl Display for UnaryOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UnaryOperator::Neg => write!(f, "-"),
            UnaryOperator::Bang => write!(f, "!"),
        }
    }
}
