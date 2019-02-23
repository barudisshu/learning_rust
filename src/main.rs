#[macro_use]
extern crate t_bang;

use std::io::Read;
use std::io::Write;

use t_bang::*;

trait LettersCount {
    fn letters_count(&self, ch: char) -> usize;
}
impl LettersCount for str {
    fn letters_count(&self, ch: char) -> usize {
        self.chars().filter(|c| *c == ch).count()
    }
}
fn main() {
    print!("{} ", "".letters_count('a'));
    print!("{} ", "ddd".letters_count('a'));
    print!("{} ", "ddd".letters_count('d'));
    print!("{} ", "foobarbaz".letters_count('a'));
}