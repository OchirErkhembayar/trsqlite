use std::{
    fmt::Display,
    io::{self, Write},
    process,
};

use crate::vm::{parser::parse_stmt, tokenizer::tokenize, VM};

pub fn run() {
    println!(
        "Toopsqlite version 0.1
Enter \".help\" for usage hints.
Connected to a transient in-memory database.
Use \".open FILENAME\" to reopen a persistent database
"
    );
    let mut stdout = io::stdout();
    stdout.flush().unwrap_or_else(|err| {
        eprintln!("ERROR: Something went wrong: {err}");
        process::exit(1);
    });

    let mut buffer = String::new();

    let stdin = io::stdin();
    let mut vm = VM::new();
    loop {
        buffer.clear();
        print!("\ntsqplite> ");
        stdout.flush().unwrap_or_else(|err| {
            eprintln!("ERROR: Something went wrong: {err}");
            process::exit(1);
        });

        match stdin.read_line(&mut buffer) {
            Ok(_) => (),
            Err(err) => {
                eprintln!("ERROR: Failed to read line because: {err}\nPlease try again.");
                continue;
            }
        }

        if buffer.trim().is_empty() {
            eprintln!("Dude, type something");
            continue;
        }

        if buffer.trim().starts_with(".") {
            match execute_meta_cmd(&buffer.trim()[1..], &mut vm) {
                Ok(_) => (),
                Err(err) => eprintln!("ERROR: {err}\nPlease try again."),
            }
            continue;
        }

        let tokens = match tokenize(&mut buffer.trim().chars().peekable()) {
            Ok(tokens) => tokens,
            Err(err) => {
                eprintln!("ERROR: {err}");
                continue;
            }
        };
        println!("Tokens: {tokens:?}");
        let stmt = match parse_stmt(&mut tokens.iter().peekable()) {
            Ok(stmt) => stmt,
            Err(err) => {
                eprintln!("ERROR: {err}");
                continue;
            }
        };
        println!("{stmt:?}");
        let result = vm.execute(stmt);
        println!("{result:?}");
    }
}

enum MetaCmdError {
    UnrecognizedCmd,
}

impl Display for MetaCmdError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                MetaCmdError::UnrecognizedCmd => "Unrecognised command",
            }
        )
    }
}

fn execute_meta_cmd(cmd: &str, vm: &mut VM) -> Result<(), MetaCmdError> {
    match cmd {
        "exit" => {
            println!("Goodbye!");
            process::exit(0);
        }
        "help" => {
            println!(
                "Available commands:
.exit - exit the program
.help - see this
.tables - a list of all the tables
"
            );
            Ok(())
        }
        "tables" => {
            let tables = vm.get_tables();
            if tables.is_empty() {
                println!("No tables");
            }
            for (name, page) in tables {
                println!("Name: {name} Page: {page}");
            }
            Ok(())
        }
        _ => Err(MetaCmdError::UnrecognizedCmd),
    }
}
