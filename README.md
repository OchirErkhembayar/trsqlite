<<<<<<< HEAD
# File Format

## Database Header
The first 100 bytes of the database file.
Contains the page size and number of pages + more

## Pages
Size: 4096 bytes
Numbered, starting from 1
### Types
1. The lock-byte page
2. A freelist page
    - A freelist trunk page
    - A freelist leaf page
3. A b-tree page
    - A table b-tree interior page
    - A table b-tree leaf page
    - An index b-tree interior page
    - An index b-tree leaf page
4. A payload overflow page
5. A pointer map page

All reads and writes start at a page boundary
Reads are usually an integer number of pages in size (except when the database is first opened, the first 100 bytes are read)

Before any modification occurs, the original content is written to the rollback journal. If a transaction is interrupted or needs to be rollde back, the rollback journal can then be used to restore the database.
Freelist pages bear no information which needs to be restored on rollback so they don't require this.

### Lock byte page
Skip it

### Free-list page
A linked list of pages which are not in use (maybe the data inside them was deleted)

### B-Tree pages
Index B-Trees and Table B-Trees
Both are either an interior page or a leaf page

#### Table B-Tree
A leaf page contains keys and values which have data
An interior page contains K keys with K+1 pointers to child b-tree pages
    - The "pointer" is just a u32 page number of the child page


=======
# trsqlite
## An SQLite clone in Rust that i'm writing for personal educational purposes

Following a tutorial (in C) and attempting to follow in Rust.
https://cstack.github.io/db_tutorial/
>>>>>>> d16061e9ff3c5b183b4af616efce0b9344b1c82d
