use std::fmt::{Display, self};

pub enum PrepareError {
    UnrecognisedStmt,
    SyntaxError,
}

impl Display for PrepareError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                PrepareError::UnrecognisedStmt => "Unrecognised statement",
                PrepareError::SyntaxError => "Syntax error",
            }
        )
    }
}

#[derive(Debug)]
pub struct Stmt {
    pub stmt_type: StmtType,
}

#[derive(Debug)]
pub struct Row {
    id: u32,
    username: String,
    email: String,
}

#[derive(Debug)]
pub enum StmtType {
    Select,
    Insert(Row),
}

impl Stmt {
    fn select() -> Self {
        Self {
            stmt_type: StmtType::Select,
        }
    }

    fn insert(row: Row) -> Self {
        Self {
            stmt_type: StmtType::Insert(row),
        }
    }
}

pub fn prepare_stmt(input: &str) -> Result<Stmt, PrepareError> {
    let len = input.len();
    if len > 6 && input[0..6].eq_ignore_ascii_case("select") {
        return Ok(Stmt::select());
    }

    if len > 6 && input[0..6].eq_ignore_ascii_case("insert") {
        let mut items = input.split_whitespace();
        items.next();
        let next_item = match items.next() {
            Some(item) => item,
            None => return Err(PrepareError::SyntaxError),
        };
        let id: u32 = match next_item.parse() {
            Ok(id) => id,
            Err(_) => return Err(PrepareError::SyntaxError),
        };
        let username = match items.next() {
            Some(item) => item,
            None => return Err(PrepareError::SyntaxError),
        };
        let email = match items.next() {
            Some(item) => item,
            None => return Err(PrepareError::SyntaxError),
        };
        let row = Row {
            id,
            username: username.to_string(),
            email: email.to_string(),
        };
        return Ok(Stmt::insert(row));
    }

    Err(PrepareError::UnrecognisedStmt)
}
