#[macro_use] extern crate t_bang;

use t_bang::*;
use std::io::Write;

fn main() {
    //ILLEGAL: std::io::stdout().write("Hi").unwrap();
    //ILLEGAL: std::io::stdout().write(String::from("Hi")).unwrap();
    std::io::stdout().write("Hello ".as_bytes()).unwrap();
    std::io::stdout().write(String::from("world").as_bytes()).unwrap();
}