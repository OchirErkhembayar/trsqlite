use std::iter::Peekable;
use super::tokenizer::{Token, TokenType};

fn match_keyword(lexeme: &str) -> Option<QueryType> {
    match lexeme {
        "select" => Some(QueryType::Select),
        "insert" => Some(QueryType::Insert),
        "create" => Some(QueryType::Create),
        "update" => Some(QueryType::Update),
        _ => None,
    }
}

#[derive(Debug)]
pub struct Stmt {
    pub query_type: QueryType,
    pub table: String,
}

impl Stmt {
    fn new(query_type: QueryType) -> Self {
        Self { 
            query_type,
            table: "foo".to_string(),
        }
    }

    fn with_table(query_type: QueryType, table: String) -> Self {
        Self {
            query_type,
            table,
        }
    }
}

#[derive(Debug)]
pub enum QueryType {
    Select,
    Insert,
    Create,
    Update,
}

pub fn parse_stmt(tokens: &mut Peekable<std::slice::Iter<'_, Token>>) -> Result<Stmt, SyntaxError> {
    let t = tokens.next();
    if t.is_none() {
        return Err(SyntaxError);
    }
    
    let t = t.unwrap();

    let query_type = match t.token_type {
        TokenType::Ident => {
            match match_keyword(t.get_lexeme()) {
                Some(keyword) => keyword,
                None => return Err(SyntaxError),
            }
        }
        _ => return Err(SyntaxError),
    };

    match query_type {
        QueryType::Select => parse_select(tokens),
        _ => return Err(SyntaxError),
    }
}

fn parse_select(tokens: &mut Peekable<std::slice::Iter<'_, Token>>) -> Result<Stmt, SyntaxError> {
    // [fields|*] from [table_name]
    while let Some(_) = tokens.next_if(|t| t.token_type != TokenType::Ident || t.lexeme.clone().is_some_and(|l| l != "from")) {
        // Going to add this later. For now we just return all.
    }
    match tokens.next() {
        Some(t) => {
            if let Some(lexeme) = t.lexeme.clone() {
                if lexeme != "from" {
                    return Err(SyntaxError);
                }
            }
        }
        None => return Err(SyntaxError),
    };
    let table = match tokens.next() {
        Some(token) => match token.token_type {
            TokenType::Ident => token.lexeme.clone().expect("Ident token without lexeme"),
            _ => return Err(SyntaxError),
        }
        None => return Err(SyntaxError),
    };
    Ok(Stmt::with_table(QueryType::Select, table))
}

#[derive(Debug)]
pub struct SyntaxError;

impl std::fmt::Display for SyntaxError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Syntax error")
    }
}
