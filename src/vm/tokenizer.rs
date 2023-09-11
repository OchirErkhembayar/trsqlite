use std::{iter::Peekable, str::Chars};

use super::parser::SyntaxError;

#[derive(Debug, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: Option<String>,
}

impl Token {
    fn with_lexeme(token_type: TokenType, lexeme: String) -> Self {
        Self { token_type, lexeme: Some(lexeme) }
    }

    fn without_lexeme(token_type: TokenType) -> Self {
        Self { token_type, lexeme: None }
    }

    pub fn get_lexeme(&self) -> &str {
        self.lexeme.as_ref().unwrap().as_str()
    }
}

#[derive(Debug, PartialEq)]
pub enum TokenType {
    Ident,
    Comma,
    LeftParen,
    RightParen,
    Equals,
    Star,
    SemiColon,
}

pub fn tokenize(chars: &mut Peekable<Chars>) -> Result<Vec<Token>, SyntaxError> {
    let mut tokens = Vec::new();
    while let Some(char) = chars.next() {
        let token = match char {
            ' ' => continue,
            ',' => Token::without_lexeme(TokenType::Comma),
            '(' => Token::without_lexeme(TokenType::LeftParen),
            ')' => Token::without_lexeme(TokenType::RightParen),
            '=' => Token::without_lexeme(TokenType::Equals),
            '*' => Token::without_lexeme(TokenType::Star),
            ';' => Token::without_lexeme(TokenType::SemiColon),
            'A'..='Z' | 'a'..='z' | '0'..='9' => {
                let mut lexeme = String::from(char);

                while let Some(char) = chars.next_if(|c| c.is_alphanumeric()) {
                    lexeme.push(char);
                }

                Token::with_lexeme(TokenType::Ident, lexeme)
            }
            _ => return Err(SyntaxError),
        };
        tokens.push(token);
    }
    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenizer() {
        let input = "input foo, bar;";
        let tokens = tokenize(&mut input.chars().peekable());
        assert_eq!(vec![
            Token::with_lexeme(TokenType::Ident, "input".to_string()),
            Token::with_lexeme(TokenType::Ident, "foo".to_string()),
            Token::without_lexeme(TokenType::Comma),
            Token::with_lexeme(TokenType::Ident, "bar".to_string()),
            Token::without_lexeme(TokenType::SemiColon),
        ], tokens.unwrap());
    }
}
