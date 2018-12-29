fn main() {
    let a: i16 = f::<i16>('a', 37, 41);
    let b: f64 = f::<f64>('b', 37.2, 41.1);
    println!("{} {}", a, b);
}

// Library code
fn f<T>(ch: char, num1: T, num2: T) -> T {
    if ch == 'a' { num1 }
    else { num2 }
}