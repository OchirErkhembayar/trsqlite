use crate::{
    consts::{
        EMAIL_OFFSET, EMAIL_SIZE, ID_OFFSET, ID_SIZE, PAGE_SIZE, ROWS_PER_PAGE, ROW_SIZE,
        TABLE_MAX_PAGES, USERNAME_OFFSET, USERNAME_SIZE,
    },
    parsing::Row,
};

use self::pager::Pager;

mod pager;

#[derive(Debug)]
pub enum ExecuteError {
    TableFull,
}

#[derive(Debug)]
pub struct Table {
    pub num_rows: usize,
    pager: Pager,
    row_num: usize,
    end_of_table: bool,
}

impl Table {
    pub fn db_open(file_name: &str) -> Self {
        let pager = Pager::new(file_name);
        let num_rows = pager.file_length / ROW_SIZE;

        Self {
            num_rows,
            pager,
            row_num: num_rows,
            end_of_table: true,
        }
    }

    pub fn db_close(&mut self) {
        let num_full_pages = self.num_rows / ROWS_PER_PAGE;

        for i in 0..num_full_pages {
            let page = self.pager.pages.get(i);
            if page.is_some_and(|p| p.is_some()) {
                self.pager.flush(i, PAGE_SIZE);
            }
        }

        let num_additional_rows = self.num_rows % ROWS_PER_PAGE;
        if num_additional_rows > 0 {
            let page_num = num_full_pages;
            if self.pager.pages.get(page_num).is_some_and(|p| p.is_some()) {
                self.pager.flush(page_num, PAGE_SIZE);
            }
        }
    }

    pub fn cursor_val(&mut self) -> (usize, usize) {
        let page_num = self.row_num / ROWS_PER_PAGE;
        let row_offset = self.row_num % ROWS_PER_PAGE;
        let byte_offset = row_offset * ROW_SIZE;
        (page_num, byte_offset)
    }

    pub fn get_row(&mut self, page_num: usize, byte_offset: usize) -> Row {
        let id_bytes = &mut [0; ID_SIZE];
        let username_bytes = &mut [0; USERNAME_SIZE];
        let email_bytes = &mut [0; EMAIL_SIZE];

        let page = &self.pager.get_page(page_num);
        let id_bytes_slice = &page[(byte_offset + ID_OFFSET)..(byte_offset + ID_OFFSET + ID_SIZE)];
        let username_bytes_slice =
            &page[(byte_offset + USERNAME_OFFSET)..(byte_offset + USERNAME_OFFSET + USERNAME_SIZE)];
        let email_bytes_slice =
            &page[(byte_offset + EMAIL_OFFSET)..(byte_offset + EMAIL_OFFSET + EMAIL_SIZE)];

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
        if self.pager.pages.len() > TABLE_MAX_PAGES {
            return Err(ExecuteError::TableFull);
        }
        self.cursor_to_end();
        let (page_num, _) = self.cursor_val();
        self.pager.append_to_page(page_num, &row.id.to_ne_bytes());
        self.pager.append_to_page(page_num, &row.username);
        self.pager.append_to_page(page_num, &row.email);
        self.num_rows += 1;
        Ok(())
    }

    pub fn cursor_to_start(&mut self) {
        self.row_num = 0;
        self.end_of_table = self.num_rows == 0;
    }

    pub fn cursor_to_end(&mut self) {
        self.row_num = self.num_rows;
        self.end_of_table = true;
    }

    pub fn advance_cursor(&mut self) {
        self.row_num += 1;
        if self.row_num >= self.num_rows {
            self.end_of_table = true;
        }
    }
}

mod tests {}
