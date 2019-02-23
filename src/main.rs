#[macro_use]
extern crate t_bang;

use std::io::Read;
use std::io::Write;

use t_bang::*;

trait HasSquareRoot {
    fn sq_root(self) -> Self;
}

impl HasSquareRoot for f32 {
    fn sq_root(self) -> Self { f32::sqrt(self) }
}

impl HasSquareRoot for f64 {
    fn sq_root(self) -> Self { f54::sqrt(self) }
}

fn quartic_root<Number>(x: Number) -> Number
    where Number: HasSquareRoot {
    x.sq_root().sq_root()
}

fn main() {
    print!("{} {}", quartic_root(100f64)), quartic_root(100f32);
}