use std::{fs::File, os::unix::prelude::FileExt, process};

use crate::consts::{PAGE_SIZE, TABLE_MAX_PAGES};

#[derive(Debug)]
pub struct Pager {
    pub file: File,
    pub file_length: usize,
    pub pages: Vec<Option<Vec<u8>>>,
}

impl Pager {
    pub fn new(file_path: &str) -> Self {
        let file = File::options()
            .read(true)
            .write(true)
            .open(file_path)
            .unwrap_or_else(|_| File::create(file_path).expect("Failed to create file"));
        let mut pages = Vec::with_capacity(TABLE_MAX_PAGES);
        for _ in 0..TABLE_MAX_PAGES {
            pages.push(None);
        }
        Self {
            file_length: file
                .metadata()
                .expect("Failed to read file meta data")
                .len() as usize,
            file,
            pages,
        }
    }

    pub fn get_page(&mut self, page_num: usize) -> &Vec<u8> {
        if page_num > TABLE_MAX_PAGES {
            eprintln!("ERROR: Tried to access page number {page_num} which is out of bounds");
            process::exit(1);
        }
        if self.pages[page_num].is_none() {
            // Cache miss. Load from file
            let mut num_pages = self.file_length / PAGE_SIZE;
            if self.file_length % PAGE_SIZE != 0 {
                num_pages += 1;
            }

            if page_num <= num_pages {
                let buf_size = if page_num * PAGE_SIZE <= self.file_length {
                    self.file_length - page_num * PAGE_SIZE
                } else {
                    PAGE_SIZE
                };
                let mut page = vec![0; buf_size];
                self.file
                    .read_exact_at(&mut page, (page_num * PAGE_SIZE) as u64)
                    .expect("Failed to read from file");
                self.pages[page_num] = Some(page);
            }
        }
        self.pages.get(page_num).as_ref().unwrap().as_ref().unwrap()
    }

    pub fn flush(&mut self, page_num: usize, page_size: usize) {
        let page = &self.pages.get(page_num);
        if page.is_none() || page.is_some_and(|p| p.is_none()) {
            panic!("Tried to flush a non existent page");
        }
        println!(
            "Page length: {}",
            page.as_ref().unwrap().as_ref().unwrap().len()
        );
        self.file
            .write_at(
                page.as_ref().unwrap().as_ref().unwrap(),
                (page_num * page_size) as u64,
            )
            .expect("Failed to flush page");
        self.pages[page_num].as_mut().unwrap().clear();
    }

    pub fn append_to_page(&mut self, page_num: usize, data: &[u8]) {
        self.pages[page_num]
            .as_mut()
            .unwrap()
            .extend_from_slice(data);
    }
}
