#![allow(unused)]
use print::log_variable;
#[derive(Debug)]
struct Foo;
impl Foo {
    #[log_variable]
    fn new() -> Self {
        let new_foo = Foo;
        new_foo
    }
}
#[log_variable]
fn main() {
    let a: i32 = 5;
    let b = a + 4;
    let c = b.abs();
    let d;
    d = 1;
    {
        let f = a + b + c;
    }
    let foo = Foo::new();
}
