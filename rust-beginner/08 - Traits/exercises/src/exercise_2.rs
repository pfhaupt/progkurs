use std::fmt::{Debug, Display};
trait Printable: Debug + Display {
    fn print_normal(&self) {
        println!("{}", self);
    }
    fn print_debug(&self) {
        println!("{:?}", self);
    }
}
struct Point {
    x: i32,
    y: i32,
}
// Compiler error, Point does not implement Display and Debug
// impl Printable for Point {}

pub fn main() {
    let p = Point { x: 10, y: 20 };
    // p.print_normal();
    // p.print_debug();
}
