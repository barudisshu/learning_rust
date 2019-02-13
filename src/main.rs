#[macro_use]
extern crate t_bang;

use t_bang::*;

fn main() {
    print!("[{:?}]", std::env::var("abcd"));
    std::env::set_var("abcd", "This is the value");
    print!(" [{:?}]", std::env::var("abcd"));
}

