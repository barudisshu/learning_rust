use std::cmp::Ordering;

///
fn main() {
    let mut arr = [4, 8, 1, 10, 0, 45, 12, 7];
    arr.sort_by(|a, b| {(&-*a).cmp(&-*b)});
    print!("{:?}", arr);
}