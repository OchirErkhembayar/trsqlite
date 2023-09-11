use self::pager::Pager;

mod pager;

pub struct DB {
    tables: Vec<Table>,
    pub headers: Headers,
    pager: Pager,
}

impl DB {
    pub fn new() -> Self {
        Self {
            tables: Vec::with_capacity(1),
            headers: Headers,
            pager: Pager::new().unwrap(),
        }
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

pub struct Table;

#[derive(Debug)]
pub struct Row;
