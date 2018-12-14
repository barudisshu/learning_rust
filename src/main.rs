fn main() {
    fn f1() { print!("1"); fn f3() {print!("3")}; }
    fn main() {
        f1();
        fn f2() { print!("2"); }
        f2(); f1(); f2();
    }
}
