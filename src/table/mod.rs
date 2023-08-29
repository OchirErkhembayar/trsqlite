use crate::{
    consts::{
        EMAIL_OFFSET, EMAIL_SIZE, ID_OFFSET, ID_SIZE, PAGE_SIZE, ROWS_PER_PAGE, ROW_SIZE,
        TABLE_MAX_PAGES, USERNAME_OFFSET, USERNAME_SIZE,
    },
    parsing::Row,
};

#[derive(Debug)]
pub enum ExecuteError {
    TableFull,
}

#[derive(Debug)]
pub struct Table {
    pub num_rows: usize,
    pages: Vec<Vec<u8>>,
}

impl Table {
    pub fn new() -> Self {
        Self {
            num_rows: 0,
            pages: Vec::new(),
        }
    }

    pub fn row_slot(&mut self, row_num: usize) -> (usize, usize) {
        let page_num = row_num / ROWS_PER_PAGE;
        if self.pages.get(page_num).is_none() {
            self.pages.push(Vec::with_capacity(PAGE_SIZE));
        }
        let row_offset = row_num % ROWS_PER_PAGE;
        let byte_offset = row_offset * ROW_SIZE;
        (page_num, byte_offset)
    }

    pub fn get_row(&self, page_num: usize, byte_offset: usize) -> Row {
        let id_bytes = &mut [0; ID_SIZE];
        let username_bytes = &mut [0; USERNAME_SIZE];
        let email_bytes = &mut [0; EMAIL_SIZE];

        let id_bytes_slice =
            &self.pages[page_num][(byte_offset + ID_OFFSET)..(byte_offset + ID_OFFSET + ID_SIZE)];
        let username_bytes_slice = &self.pages[page_num]
            [(byte_offset + USERNAME_OFFSET)..(byte_offset + USERNAME_OFFSET + USERNAME_SIZE)];
        let email_bytes_slice = &self.pages[page_num]
            [(byte_offset + EMAIL_OFFSET)..(byte_offset + EMAIL_OFFSET + EMAIL_SIZE)];

        id_bytes.copy_from_slice(id_bytes_slice);
        username_bytes.copy_from_slice(username_bytes_slice);
        email_bytes.copy_from_slice(email_bytes_slice);
        let id = u32::from_ne_bytes(*id_bytes);
        Row {
            id,
            username: *username_bytes,
            email: *email_bytes,
        }
    }

    pub fn insert_row(&mut self, row: &Row) -> Result<(), ExecuteError> {
        if self.pages.len() > TABLE_MAX_PAGES {
            return Err(ExecuteError::TableFull);
        }
        let (page_num, _) = self.row_slot(self.num_rows);
        self.pages[page_num].extend_from_slice(&row.id.to_ne_bytes());
        self.pages[page_num].extend_from_slice(&row.username);
        self.pages[page_num].extend_from_slice(&row.email);
        self.num_rows += 1;
        Ok(())
    }
}

mod tests {}
