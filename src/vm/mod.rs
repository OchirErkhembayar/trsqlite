use crate::db::{DB, Row};

use self::parser::{Stmt, QueryType};

pub mod parser;
pub mod tokenizer;
pub mod code_generator;

pub struct VM {
    db: DB,
}

impl VM {
    pub fn new() -> Self {
        Self {
            db: DB::new(),
        }
    }

    pub fn execute(&mut self, stmt: Stmt) -> ExecuteResult {
        match stmt.query_type {
            QueryType::Select => ExecuteResult::with_rows(self.db.get_rows(stmt.table.as_str())),
            _ => ExecuteResult::without_rows(false),
        }
    }
}

#[derive(Debug)]
pub struct ExecuteResult {
    rows: Option<Vec<Row>>,
    success: bool
}

impl ExecuteResult {
    fn with_rows(rows: Vec<Row>) -> Self {
        Self {
            rows: Some(rows),
            success: true,
        }
    }

    fn without_rows(success: bool) -> Self {
        Self {
            rows: None,
            success,
        }
    }
}

