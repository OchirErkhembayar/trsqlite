use interface::repl;

mod vm;
mod db;
mod interface;

fn main() {
    repl::run();
}
