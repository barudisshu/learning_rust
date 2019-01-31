#[macro_use] extern crate t_bang;
use t_bang::*;

fn main() {
    print_nth_char("€èe", 3);
}

fn print_nth_char(s: &str, mut n: u32) {
    let mut iter: std::str::Chars = s.chars();
    loop {
        let item: Option<char> = iter.next();
        match item {
            Some(c) => if n == 1 { print!("{}", c); },
            None => { break; },
        }
        n -= 1;
    }
}