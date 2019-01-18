use std::cmp::Ordering;

///
fn main() {
    print!(
        "{}",
        (|v: &Vec<i32>| {
            let mut sum = 0;
            for i in 0..v.len() {
                sum += v[i];
            }
            sum
        })(&vec![11, 22, 34]));
}