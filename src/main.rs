use std::time::Instant;

fn elapsed_ms(t1: Instant, t2: Instant) -> f64 {
    let t = t2 - t1;
    t.as_secs() as f64 * 1000. + t.subsec_nanos() as f64 / 1e6
}
const SIZE: i32 = 40_000;
fn ns_per_op(t1: Instant, t2: Instant) -> f64 {
    elapsed_ms(t1, t2) / SIZE as f64 * 1_000_000.
}

fn main() {
    let arr = [(640, 'T'), (917, 'C'), (412, 'S'), (670, 'T'), (917, 'L')];
}