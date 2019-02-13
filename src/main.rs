#[macro_use]
extern crate t_bang;

use t_bang::*;

fn main() {
    let v = [66, -8, 43, 19, 0, -31]
        .iter()
        .filter(|x| { print!("F{} ", x); **x > 0 })
        .map(|x| { print!("M{} ", x); *x * 2 })
        .collect::<Vec<_>>();
    print!("{:?}", v);
}

