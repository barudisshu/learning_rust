fn main() {
    let mut arr = [5, -4, 9, 0, -7, -1, 3, 5, 3, 1];
    double_negatives(&mut arr);
    println!("{:?}", arr);

    let mut a = 3;
    plus(&mut a);
    println!("{:?}", a);
}
fn double_negatives(a: &mut [i32; 10]) {
    for i in 0..10 {
        if a[i] < 0 { a[i] *= 2; }
    }
}
fn plus(a: &mut i32) {
    let factor:&i32 = &2;
    *a += factor
}
