use std::mem::*;

///
fn main() {
    print!("{} ", min(&[23, 17]));
    print!("{} ", min(&vec![55, 22, 33, 44]));
}

fn min(arr: &[i32]) -> i32 {
    // Let's assume 'arr' is not empty.
    let mut minimum = arr[0];
    for i in 1..arr.len() {
        if arr[i] < minimum { minimum = arr[i]; }
    }
    minimum
}