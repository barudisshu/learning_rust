fn main() {
    let mut a = "Hello".to_string();
    let mut b = a;
    print!("{}, ", b);
    a = "world".to_string();
    print!("{}!", a);
    b = a;
}
