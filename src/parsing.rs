use std::{
    fmt::{self, Display},
};
use crate::consts::{USERNAME_SIZE, EMAIL_SIZE};

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
pub struct Row {
    pub id: u32,
    pub username: [u8; USERNAME_SIZE],
    pub email: [u8; EMAIL_SIZE],
}

#[derive(Debug, PartialEq)]
pub enum StmtType {
    Select,
    Insert(Box<Row>),
}

impl Stmt {
    fn select() -> Self {
        Self {
            stmt_type: StmtType::Select,
        }
    }

    fn insert(row: Row) -> Self {
        Self {
            stmt_type: StmtType::Insert(Box::new(row)),
        }
    }
}

pub fn prepare_stmt(input: &str) -> Result<Stmt, PrepareError> {
    let len = input.len();
    if len >= 6 && input[0..6].eq_ignore_ascii_case("select") {
        return Ok(Stmt::select());
    }

    if len >= 6 && input[0..6].eq_ignore_ascii_case("insert") {
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
        let username_str = match items.next() {
            Some(item) => item,
            None => return Err(PrepareError::SyntaxError),
        };
        let email_str = match items.next() {
            Some(item) => item,
            None => return Err(PrepareError::SyntaxError),
        };
        let username_bytes = username_str.as_bytes();
        if username_bytes.len() > USERNAME_SIZE {
            return Err(PrepareError::SyntaxError);
        }
        let mut username: [u8; USERNAME_SIZE] = [0; USERNAME_SIZE];
        for (i, v) in username_bytes.iter().enumerate() {
            username[i] = *v;
        }
        let email_bytes = email_str.as_bytes();
        if email_bytes.len() > EMAIL_SIZE {
            return Err(PrepareError::SyntaxError);
        }
        let mut email: [u8; EMAIL_SIZE] = [0; EMAIL_SIZE];
        for (i, v) in email_bytes.iter().enumerate() {
            email[i] = *v;
        }
        let row = Row {
            id,
            username,
            email,
        };
        return Ok(Stmt::insert(row));
    }

    Err(PrepareError::UnrecognisedStmt)
}

mod tests {
    use super::*;

    #[test]
    fn prepare_insert_stmt() {
        let input = "insert 1 a b";
        let stmt = prepare_stmt(input);
        let stmt = stmt.expect("Oh no");
        let username_bytes = "a".as_bytes();
        let mut username: [u8; USERNAME_SIZE] = [0; USERNAME_SIZE];
        for (i, v) in username_bytes.iter().enumerate() {
            username[i] = *v;
        }
        let email_bytes = "b".as_bytes();
        let mut email: [u8; EMAIL_SIZE] = [0; EMAIL_SIZE];
        for (i, v) in email_bytes.iter().enumerate() {
            email[i] = *v;
        }
        let row = Box::new(Row {
            id: 1,
            username,
            email,
        });
        assert_eq!(32, row.username.len());
        assert_eq!(255, row.email.len());
        assert_eq!(StmtType::Insert(row), stmt.stmt_type);
    }

    #[test]
    fn username_too_long() {
        let input = "insert 1 OWJEFOIWJEFPOIWAEFCNJOIWAJFOIWJFOIWJEFJEWIFJWOIEJFIWEJFEWJFIFOIWIFEJWOIJFIWJEOIFJWOIFJWIOEFOIWGZGOIESROIGNZ;GSREJGZS;LJGWZEJF;OIZJEWZS'JRT;OIDZJG;OIERHG;OISZJGR;OISRGIZSJRGZ;OIDJG;OIEHG;JROIEOG;IJOIESAJYOIEJG;AWRJTPEARJGSE;OIRH;OIESRHJ;ESOIRGJRAJGESORHJTRHJOIE`RJG;OIESOIHJTRSIPJSOIROWIoiwjfoiwjef;awoienf;jwejaenfcwajeofwjeoifjweoijfiwojfoiwjfjwoeijfwoiejfwoiejfoiwjefoiwefjwoejfwiefjoiwjfeiwofejwoiejfoiwjfoiwoiejfoiewjJFEOIWJOIOIjoJWOIJFOIWJFEOIWJOISDJFOIEWFOIWJFNZVOIOIRESJGOIERJGDSFGHESOIJGPJESGSIEGLDSHOIGSREONGCJGNOIESNJGCOIRESNJGC;SOIJRENCOIRESJGCN;ESOINJGC;OIRESJNGC;OIRESNCG b";
        let stmt = prepare_stmt(input);
        match stmt {
            Ok(_) => panic!("Should be error"),
            Err(err) => assert_eq!(err, PrepareError::SyntaxError),
        }
    }

    #[test]
    fn email_too_long() {
        let input = "insert 1 a FOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOFOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOFOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOFOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOFOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOFOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOFOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOFOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOFOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOFOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOFOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOFOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOFOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOO";
        let stmt = prepare_stmt(input);
        match stmt {
            Ok(_) => panic!("Should be error"),
            Err(err) => assert_eq!(err, PrepareError::SyntaxError),
        }
    }
}
