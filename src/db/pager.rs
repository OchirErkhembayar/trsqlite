use std::{collections::BTreeMap, error::Error, fs::File, io::Write, os::unix::prelude::FileExt};

use super::Table;

const PAGE_SIZE: u16 = 4096;

const PAGE_SIZE_OFFSET: usize = 32;
const PAGE_SIZE_LENGTH: usize = 2;

const TEXT_ENCODING_OFFSET: usize = 56;
const TEXT_ENCODING_LENGTH: usize = 4;

const HEADER_LENGTH: usize = 100;

const MASTER_TABLE_OFFSET: usize = HEADER_LENGTH;
const MASTER_TABLE_MAX_SIZE: usize = PAGE_SIZE as usize - MASTER_TABLE_OFFSET;

const MASTER_TABLE_KEY_SIZE: usize = 100;
const MASTER_TABLE_VALUE_SIZE: usize = u32::BITS as usize;

/*
* First 100 bytes are reserved for header data
* Page size is 4096KB
* First page contains header data and master b-tree
* Tables start from page-2 onwards
*
* Master b-tree stored as array of bytes. Terminating with 0s
* 100 bytes for keys (table name)
* 32 bit int representing the page that the table is located at
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
    page_size: u16,
    master_table: BTreeMap<String, u32>,
    pages: Vec<Page>,
}

impl Pager {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let file = match File::options().read(true).open("./db.db") {
            Ok(file) => file,
            Err(_) => {
                let mut file = File::create("./db.db")?;
                file.write_at(&[0; MASTER_TABLE_MAX_SIZE], MASTER_TABLE_OFFSET as u64)?;
                file.write_at(&PAGE_SIZE.to_be_bytes(), PAGE_SIZE_OFFSET as u64)?;
                file.flush()?;
                return Ok(Self {
                    file,
                    page_size: PAGE_SIZE,
                    master_table: BTreeMap::new(),
                    pages: Vec::new(),
                });
            }
        };
        // Get page size
        let mut page_size_bytes = [0; PAGE_SIZE_LENGTH];
        file.read_at(&mut page_size_bytes, PAGE_SIZE_OFFSET as u64)?;
        let page_size = u16::from_be_bytes(page_size_bytes);
        // Get master table
        let mut master_table_bytes = [0; MASTER_TABLE_MAX_SIZE];
        file.read_at(&mut master_table_bytes, MASTER_TABLE_OFFSET as u64)?;
        let mut master_table = BTreeMap::new();
        const CHUNK_SIZE: usize = MASTER_TABLE_KEY_SIZE + MASTER_TABLE_VALUE_SIZE;
        for keyval in master_table_bytes.chunks(CHUNK_SIZE) {
            if keyval == [0; CHUNK_SIZE] {
                break;
            }
            let key = &keyval[0..MASTER_TABLE_KEY_SIZE];
            let value = &keyval[MASTER_TABLE_KEY_SIZE..MASTER_TABLE_VALUE_SIZE];
            master_table.insert(
                String::from_utf8(Vec::from(key))?,
                u32::from_be_bytes(value.try_into().unwrap()),
            );
        }
        Ok(Self {
            file,
            page_size,
            master_table,
            pages: Vec::new(),
        })
    }

    pub fn get_tables(&self) -> Vec<(&str, u32)> {
        self.master_table
            .iter()
            .map(|(k, v)| (k.as_str(), *v))
            .collect()
    }

    pub fn find_page(&self, table_name: &str) -> Vec<u8> {
        let mut tree = BTreeMap::new();
        let bytes = tree.insert('f', "bar");
        Vec::new()
    }
}

struct Page {
    num: u32,
    table: Table,
}
