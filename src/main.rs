struct S<'a, T: 'a> { b: &'a T }

fn main() {
    let s1 = S { b: &true };
    let s2 = S { b: &s1 };
    let S { b: r1 } = s1;
    let S { b: &S { b: r2 } } = s2;
    println!("{} {} {} {}", s1.b, s2.b.b, r1, r2);
    println!("{:p} {:p} {:p} {:p}", s1.b, s2.b.b, r1, r2);
}
