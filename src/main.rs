fn main() {
    let k = 20;
    f1(k + 4);
    f2(30);
}

fn f1(x1: i32) {
    let y1 = 2 + x1;
}

fn f2(x2: i32) {
    f1(x2 + 7);
}

