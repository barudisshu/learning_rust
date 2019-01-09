use std::mem::*;

///
/// 部分组合类型的情况会出现结构填充(padding)，造成内存浪费。
fn main() {
    print!("{} {} {} {} {} {}",
           size_of_val(&[0i16; 80]),
           size_of_val(&(0i16, 0i64)),
           size_of_val(&[(0i16, 0i64); 100]),
           size_of_val(&E1::_E1a),
           size_of_val(&E2::_E2a),
           size_of_val(&vec![(0i16, 0i64); 100]));
}

enum E1 {
    _E1a,
    _E1b,
}

enum E2 {
    _E2a,
    _E2b(f64),
}

