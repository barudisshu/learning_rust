#[macro_use]
extern crate t_bang;

use t_bang::*;

fn main() {
    let arr = [66, -8, 43, 19, 0, -32];
    let v = arr
    .iter()
    .filter(|x| **x > 0)
    .map(|x| *x * 2)
    .collect::<Vec<_>>();
    print!("{:?}", v);    
    print!("{:?}", t!(v));
}

