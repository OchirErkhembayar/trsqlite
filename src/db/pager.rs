use std::{
    collections::BTreeMap,
    fs::File,
    io::{self, Read, Seek, SeekFrom},
    os::unix::prelude::FileExt,
};

use super::Table;

const PAGE_SIZE: u32 = 4096;

const PAGE_SIZE_OFFSET: usize = 0;
const PAGE_SIZE_LENGTH: usize = 2;
const MASTER_TABLE_OFFSET: usize = 2;
const MASTER_TABLE_SIZE: usize = 3996;

/*
* First 100 bytes are reserved for header data
* Page size is 4096KB
* First page contains header data and master b-tree
* Tables start from page-2 onwards
* Starting with just a single table with fixed columns already implemented
*/

/*
* Master table
* Keys are table names
* Values are pointing to pages
*
* Page
* Each page will have a header notifying you whether or not its
*/

pub struct Pager {
    file: File,
    headers: Headers,
}

impl Pager {
    pub fn new() -> Result<Self, io::Error> {
        let mut file = match File::options().read(true).open("./db.db") {
            Ok(file) => file,
            Err(_) => {
                let mut file = File::create("./db.db")?;
                let headers = Headers {
                    page_size: PAGE_SIZE,
                    master_table: BTreeMap::new(),
                };
                file.write_at(&(4096 as u32).to_be_bytes(), PAGE_SIZE_OFFSET as u64);
                file.write_at(&[0; MASTER_TABLE_SIZE], MASTER_TABLE_OFFSET as u64);
                return Ok(Self { file, headers });
            }
        };
        // Read the headers. First 100 bytes
        file.seek(SeekFrom::Start(PAGE_SIZE_OFFSET as u64));
        let mut page_size_bytes = [0; 4];
        file.read_exact(&mut page_size_bytes)?;
        let page_size = u32::from_be_bytes(page_size_bytes);
        file.seek(SeekFrom::Start(MASTER_TABLE_OFFSET as u64));
        let mut master_table_bytes = [0; MASTER_TABLE_SIZE];
        let headers = Headers {
            page_size,
            master_table: BTreeMap::new(),
        };
        Ok(Self { file, headers })
    }

    pub fn find_page(&self, table_name: &str) -> Vec<u8> {
        let mut tree = BTreeMap::new();
        let bytes = tree.insert('f', "bar");
        Vec::new()
    }
}

struct Headers {
    page_size: u32,
    master_table: BTreeMap<String, Table>,
}
