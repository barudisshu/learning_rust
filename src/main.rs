#[macro_use] extern crate t_bang;
use t_bang::*;

fn main() {
    let s = "abc012è€";
    println!("{:?}", t!(s));
    for i in 0..s.len() {
        println!("{}: {}", i, s.as_bytes()[i]);
    }
}