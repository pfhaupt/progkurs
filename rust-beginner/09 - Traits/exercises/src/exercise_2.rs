use std::fmt::{Debug, Display};
trait Printable: Debug + Display {
    fn print_normal(&self) {
        println!("{}", self);
    }
    fn print_debug(&self) {
        println!("{:?}", self);
    }
}
impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}
impl Printable for Point {}

pub fn main() {
    let p = Point { x: 10, y: 20 };
    p.print_normal();
    p.print_debug();
}
