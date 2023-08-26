use std::{
    fmt::{self, Display},
    io::{self, Write},
    process,
};

use parsing::{prepare_stmt, Stmt, StmtType, Row};

mod parsing;

enum MetaCommandError {
    UnrecognisedCommand,
}

impl Display for MetaCommandError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                MetaCommandError::UnrecognisedCommand => "Unrecognised meta command",
            }
        )
    }
}

// Rows stored in pages
// Page stores as many rows as it can fit
// Pages allocated as needed
// Pages stored in an array
struct Table {
    name: String,
    rows: Vec<Row>,
}

fn main() {
    let mut input = String::new();
    loop {
        let input = match get_input(&mut input) {
            Ok(str) => str,
            Err(err) => {
                eprintln!("ERROR: Failed to read input, {err}");
                continue;
            }
        };

        if input[..1].eq_ignore_ascii_case(".") {
            let input = &input[1..];
            if let Err(err) = do_meta_cmd(input) {
                println!("ERROR: {err}");
            }
            continue;
        }

        match prepare_stmt(input) {
            Ok(stmt) => execute_stmt(stmt),
            Err(err) => {
                println!("ERROR: {err}");
            }
        }
    }
}

fn do_meta_cmd(input: &str) -> Result<(), MetaCommandError> {
    if input.eq_ignore_ascii_case("exit") {
        println!("Goodbye");
        process::exit(1);
    }
    Err(MetaCommandError::UnrecognisedCommand)
}

fn execute_stmt(stmt: Stmt) {
    match stmt.stmt_type {
        StmtType::Select => println!("Select stmt {stmt:?}"),
        StmtType::Insert(row) => {

        }
    }
}

fn get_input(input: &mut String) -> Result<&str, io::Error> {
    input.clear();
    print!("db > ");
    io::stdout().flush()?;
    io::stdin().read_line(input)?;
    Ok(input.trim())
}

