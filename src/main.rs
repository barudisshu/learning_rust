use std::time::Instant;

fn elapsed_ms(t1: Instant, t2: Instant) -> f64 {
    let t = t2 - t1;
    t.as_secs() as f64 * 1000. + t.subsec_nanos() as f64 / 1e6
}

fn main() {
    let a = [48, 18, 20, 35, 17, 13, 39, 12, 42, 33, 29, 27, 50, 16];
    let mut v = std::collections::BinaryHeap::<i32>::new();
    for i in 0..a.len() / 2 {
        v.push(a[i * 2]);
        v.push(a[i * 2 + 1]);
        print!("{} ", v.pop().unwrap());
    }
    while ! v.is_empty() {
        print!("{} ", v.pop().unwrap());
    }
}