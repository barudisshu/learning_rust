use std::panic;

fn main() {
    let v: Option<i16> = Some(11);
    let w: Result<i32, String> = Ok(22);
    let x: Option<Result<i32, String>> = Some(Ok(33));
    print!("{}, {}, {:?}, ", v.unwrap(), w.unwrap(), x.unwrap());
}

