use std::mem::size_of_val;

///
fn main() {
    let mut a: &str = "";
    let mut b: &str = "0123456789";
    let mut c: &str = "abcdè";
    print!("{} {} {}",
        size_of_val(a),
        size_of_val(b),
        size_of_val(c)
    );
}