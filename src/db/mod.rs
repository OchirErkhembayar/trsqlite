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

pub struct Table {
    name: String,
    page_num: u32,
}

#[derive(Debug)]
pub struct Row;
