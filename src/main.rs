fn main() {
    let x = 4.;
    print_double(x);
    print!("{}", x);
}

fn print_double(mut x: f64) {
    x *= 2.;
    print!("{} ", x);
}