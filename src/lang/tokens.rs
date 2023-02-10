#[derive(Debug)]
pub enum TokenType {
    Num,
    Id,
    Let,
    Assign,
    Eq,
    OpenParenthesis,
    ClosedParenthis,
    BinOp,
    Newline,
}

#[derive(Debug)]
pub struct Token {
    value: String,
    tokenType: TokenType,
}

pub fn tokenize(source: &str) -> Result<Vec<Token>, String> {
    source
        .split(&[' ', '\r', '\n'])
        .filter(|c| !c.is_empty())
        .map(str_to_token)
        .collect()
}

fn str_to_token(token: &str) -> Result<Token, String> {
    if token.chars().all(|c| c.is_ascii_digit()) {
        return Ok(Token {
            value: token.to_string(),
            tokenType: TokenType::Num,
        });
    }
    return match token {
        "(" => Ok(Token {
            value: "(".to_string(),
            tokenType: TokenType::OpenParenthesis,
        }),
        ")" => Ok(Token {
            value: ")".to_string(),
            tokenType: TokenType::ClosedParenthis,
        }),
        "=" => Ok(Token {
            value: "=".to_string(),
            tokenType: TokenType::Assign,
        }),
        "==" => Ok(Token {
            value: "==".to_string(),
            tokenType: TokenType::Eq,
        }),
        "+" | "-" | "*" | "/" => Ok(Token {
            value: token.to_string(),
            tokenType: TokenType::BinOp,
        }),
        "let" => Ok(Token {
            value: token.to_string(),
            tokenType: TokenType::Let,
        }),
        "\n" | "\r\n" => Ok(Token {
            value: token.to_string(),
            tokenType: TokenType::Newline,
        }),
        word if word.chars().all(|c| c.is_ascii_alphabetic()) => Ok(Token {
            value: token.to_string(),
            tokenType: TokenType::Id,
        }),
        _ => Err(format!("could not parse token '{}'", token)),
    };
}
