#![allow(unused)]

struct Tree {
    value: i32,
    left: Box<Tree>,
    right: Box<Tree>
}

struct Point {
    x: i32,
    y: i32,
}
struct Line {
    points: Vec<Point>,
}
impl Point {
    fn new(x: i32, y: i32) -> Self {
        return Self { x, y };
    }
    fn distance(&self, p2: &Point) -> f64 {
        let dx = (p2.x - self.x) as f64;
        let dy = (p2.y - self.y) as f64;
        let xs = dx * dx;
        let ys = dy * dy;
        f64::sqrt(xs + ys)
    }
    fn update(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }
    fn zero() -> Self {
        Self { x: 0, y: 0 }
    }
    fn x(mut self, x: i32) -> Self {
        self.x = x;
        self
    }
    fn y(mut self, y: i32) -> Self {
        self.y = y;
        self
    }
}
fn main() {
    let p1 = Point::zero().x(3).y(4);
    println!("p1.x = {}", p1.x);
    println!("p1.y = {}", p1.y);
}
fn mut_method() {
    let mut p1 = Point::new(1, 2);
    println!("before: {}", p1.x);
    p1.update(3, 4);
    println!("after:  {}", p1.x);
}
fn calculate_distance() {
    let p1 = Point::new(1, 2);
    let p2 = Point::new(3, 4);
    let d = p1.distance(&p2);
    println!("distance = {}", d);
}
fn new_structs() {
    let p1 = Point::new(1, 2);
    let p2 = Point::new(3, 4);
    let line = Line::new(&p1, &p2);
}

impl Line {
    fn new(start: &Point, end: &Point) -> Self {
        panic!("Still an unsolved problem :^)")
    }
}
fn create_line(start: &Point, end: &Point) -> Line {
    panic!("Too much effort to figure this out :^)")
}
fn create_points() {
    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 3, y: 4 };
    // v-- Compiler error --v
    // let line = Line { points: vec![p1, p2] };
    println!("p1.x = {}", p1.x);
    println!("p2.y = {}", p2.y);
}