use std::{
    fmt::{self, Display},
    io::{self, Write},
    process,
};
use parsing::{prepare_stmt, Stmt, StmtType, Row};
use table::Table;

mod parsing;
mod table;
mod consts;
mod vm;

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


fn main() {
    let mut input = String::new();
    let mut table = Table::new();
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
            Ok(stmt) => execute_stmt(stmt, &mut table),
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

fn execute_stmt(stmt: Stmt, table: &mut Table) {
    match stmt.stmt_type {
        StmtType::Select => {
            let mut rows = Vec::new();
            for i in 0..table.num_rows {
                let (page_num, byte_offset) = table.row_slot(i);
                println!("{page_num}, {byte_offset}");
                rows.push(table.get_row(page_num, byte_offset));
            }
            for row in rows.iter() {
                println!("id: {} username: {} email: {}", 
                    row.id, String::from_utf8(Vec::from(row.username)).unwrap(), String::from_utf8(Vec::from(row.email)).unwrap(),
                );
            }
        },
        StmtType::Insert(row) => {
            table.insert_row(&row).expect("fail");
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

