use std::mem::*;

///
fn main() {
    let mut s = "".to_string();
    for _ in 0..10 {
        println!("{:?} \t{} \t{}",
                 s.as_ptr(), s.capacity(), s.len());
        s.push('a');
    }
    println!("{:?} \t{} \t{}: \t{}",
             s.as_ptr(), s.capacity(), s.len(), s);
}