use std::{collections::HashMap, process};

use self::pager::Pager;

mod pager;

pub struct DB {
    tables: HashMap<String, Table>,
    pub headers: Headers,
    pager: Pager,
}

impl DB {
    pub fn new() -> Self {
        let pager = Pager::new().unwrap_or_else(|err| {
            eprintln!("ERROR: Failed to initialise pager {err}");
            process::exit(1);
        });
        Self {
            tables: HashMap::new(),
            headers: Headers,
            pager,
        }
    }

    pub fn create_table(&mut self, table: Table) {
        self.pager.add_table(table.name);
    }

    pub fn get_tables(&self) -> Vec<(&str, u32)> {
        self.pager.get_tables()
    }

    pub fn get_rows(&self, table: &str) -> Vec<Row> {
        let table = self.find_table(table);
        Vec::new()
    }

    fn find_table(&self, name: &str) -> Option<Table> {
        let page = self.pager.find_page(name);
        None
    }
}

pub struct Headers;

#[derive(Debug)]
enum DataType {
    VarChar,
    Int,
    Float,
    Bool,
}

#[derive(Debug)]
pub struct Column {
    data_type: DataType,
    name: String,
}

#[derive(Debug)]
pub struct Table {
    name: String,
    columns: Vec<Column>,
    page_num: u32,
    rows: Vec<Row>,
}

#[derive(Debug)]
pub struct Row {
    id: u32,
    cols: Vec<Column>,
}
