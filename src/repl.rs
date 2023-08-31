use crate::parsing::{prepare_stmt, Row, Stmt, StmtType};
use crate::table::Table;
use std::{
    fmt::{self, Display},
    io::{self, Write},
    process,
};

enum MetaCommandError {
    UnrecognisedCommand,
}

struct ExecuteResult {
    result_type: ResultType,
}

enum ResultType {
    Select(Vec<Row>),
    Insert,
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

pub fn run() {
    let mut input = String::new();
    let mut table = Table::db_open("./data.db");
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
            if let Err(err) = do_meta_cmd(input, &mut table) {
                println!("ERROR: {err}");
            }
            continue;
        }

        match prepare_stmt(input) {
            Ok(stmt) => {
                if let ResultType::Select(rows) = execute_stmt(stmt, &mut table).result_type {
                    for row in rows.iter() {
                        println!(
                            "id: {} username: {} email: {}",
                            row.id,
                            String::from_utf8(Vec::from(row.username)).unwrap(),
                            String::from_utf8(Vec::from(row.email)).unwrap(),
                        );
                    }
                }
            }
            Err(err) => {
                println!("ERROR: {err}");
            }
        }
    }
}

fn do_meta_cmd(input: &str, table: &mut Table) -> Result<(), MetaCommandError> {
    if input.eq_ignore_ascii_case("exit") {
        table.db_close();
        println!("Goodbye");
        process::exit(1);
    }
    Err(MetaCommandError::UnrecognisedCommand)
}

fn execute_stmt(stmt: Stmt, table: &mut Table) -> ExecuteResult {
    match stmt.stmt_type {
        StmtType::Select => {
            let mut rows = Vec::new();
            for i in 0..table.num_rows {
                let (page_num, byte_offset) = table.row_slot(i);
                rows.push(table.get_row(page_num, byte_offset));
            }
            ExecuteResult {
                result_type: ResultType::Select(rows),
            }
        }
        StmtType::Insert(row) => {
            table.insert_row(&row).expect("fail");
            ExecuteResult {
                result_type: ResultType::Insert,
            }
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

mod tests {

    #[test]
    fn it_can_insert_and_read_a_single_statement() {
        let mut table = Table::db_open("./test");
        let input = "insert 1 foo foo@bar.com";
        let insert_stmt = prepare_stmt(input).unwrap();
        execute_stmt(insert_stmt, &mut table);
        let input = "select";
        let select_stmt = prepare_stmt(input).unwrap();
        let result = execute_stmt(select_stmt, &mut table);
        let insert_stmt = prepare_stmt("insert 1 foo foo@bar.com").unwrap();
        match result.result_type {
            ResultType::Select(rows) => match insert_stmt.stmt_type {
                StmtType::Insert(row) => {
                    assert_eq!(vec![*row], rows);
                }
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
        std::fs::remove_file("./test").expect("Failed to remove test file");
    }
}
