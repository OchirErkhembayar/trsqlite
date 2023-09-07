use std::iter::Peekable;
use super::tokenizer::{Token, TokenType};

static KEYWORDS: [&str; 2] = ["select", "insert"];

fn match_keyword(lexeme: &str) -> Option<StmtType> {
    match lexeme {
        "select" => Some(StmtType::Select),
        "insert" => Some(StmtType::Insert),
        _ => None,
    }
}

#[derive(Debug)]
pub struct Stmt {
    stmt_type: StmtType,
}

impl Stmt {
    fn new(stmt_type: StmtType) -> Self {
        Self { stmt_type }
    }
}

#[derive(Debug)]
pub enum StmtType {
    Select,
    Insert,
}

#[derive(Debug)]
pub struct SelectStmt {
    table: String,
}

impl SelectStmt {
    fn new (table: String) -> Self {
        Self { table }
    }
}

pub fn parse_stmt(tokens: &mut Peekable<std::slice::Iter<'_, Token>>) -> Result<Stmt, SyntaxError> {
    let t = tokens.next();
    if t.is_none() {
        return Err(SyntaxError);
    }
    let t = t.unwrap();

    match t.token_type {
        TokenType::Ident => {
            let stmt_type = match match_keyword(t.get_lexeme()) {
                Some(keyword) => keyword,
                None => return Err(SyntaxError),
            };
            Ok(Stmt::new(stmt_type))
        }
        _ => Err(SyntaxError),
    }
}

#[derive(Debug)]
pub struct SyntaxError;

impl std::fmt::Display for SyntaxError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Syntax error")
    }
}
