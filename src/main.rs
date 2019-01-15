use std::mem::*;

///
/// 部分组合类型的情况会出现结构填充(padding)，造成内存浪费。
fn main() {
    let mut v = vec![0; 0];
    let mut prev_capacity = std::usize::MAX;
    for i in 0..1_000 {
        let cap = v.capacity();
        if cap != prev_capacity {
            println!("{} {} {}", i, v.len(), cap);
            prev_capacity = cap;
        }
        v.push(1);
    }
}