struct S<'a> {
    _b: bool,
    _ri: &'a i32,
}

fn create_s(ri: &i32) -> S {
    S {
        _b: true,
        _ri: ri,
    }
}

fn main() {
    let x: i32 = 12;
    let _y: S;
    _y = create_s(&x);
}
