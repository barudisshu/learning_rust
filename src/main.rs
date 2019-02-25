#[macro_use]
extern crate t_bang;

use std::io::Read;
use std::io::Write;

use t_bang::*;

struct Pair(u32, u32);
impl std::iter::Iterator for Pair {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

fn main() {
    let mut a = Pair(23u32, 34u32);
    print!("{:?}", a.next());
}