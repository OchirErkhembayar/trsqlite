//! Constants used throughout the program

/// Size of each page set to 4KB
pub const PAGE_SIZE: usize = 4096;

/// Maximum number of rows per page
pub const ROWS_PER_PAGE: usize = PAGE_SIZE / ROW_SIZE;

/// Maximum number of pages a table can contain
pub const TABLE_MAX_PAGES: usize = 100;

/// Size of the ID field
pub const ID_SIZE: usize = 4;

/// Size of username field
pub const USERNAME_SIZE: usize = 32;

/// Size of email field
pub const EMAIL_SIZE: usize = 255;

/// Size of a single row (currently fixed to one type of row)
pub const ROW_SIZE: usize = USERNAME_SIZE + EMAIL_SIZE + ID_SIZE;

pub const ID_OFFSET: usize = 0;

pub const USERNAME_OFFSET:usize = ID_OFFSET + ID_SIZE;

pub const EMAIL_OFFSET: usize = USERNAME_OFFSET;
