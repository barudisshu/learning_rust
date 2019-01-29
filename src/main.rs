#[macro_use] extern crate t_bang;
use t_bang::*;

fn main() {
    let arr = [11, 22, 33, 44];
    let n = 2;
    let sr1 = &arr[..n];
    let sr2 = &arr[n..];
    let sr3 = &sr1[1..];
    print!("{:?} {:?} {:?}", t!(sr1), sr2, t!(sr3));
}