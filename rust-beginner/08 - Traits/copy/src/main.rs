#![allow(unused)]
#[derive(Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 1, y: 2 };
    let p1 = p;
    println!("p.x = {}", p.x);
    println!("p1.x = {}", p1.x);
    println!("Hello, world!");
}
