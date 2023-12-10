#![allow(unused)]

struct Point {
    x: i32,
    y: i32,
}
impl Point {
    fn new(x: i32, y: i32) -> Self {
        return Self { x, y };
    }
    fn distance(&self, other: &Point) -> f64 {
        let dx = (other.x - self.x);
        let dy = (other.y - self.y);
        let x_squared = (dx * dx) as f64;
        let y_squared = (dy * dy) as f64;
        return f64::sqrt(x_squared + y_squared);
    }
    fn update(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }
    fn nom(self) {
        println!("nomnom, I ate self");
    }
}
fn main() {
    let mut p1 = Point::new(1, 2);
    p1.nom();
    println!("p1.x = {}", p1.x);
    // let p2 = Point::new(3, 4);
    // let d = p1.distance(&p2);
    // println!("d = {}", d);
    // let p1 = Point { x: 1, y: 2 };
    // let p2 = Point { x: 3, y: 4 };
    // println!("p1.x = {}", p1.x);
    // println!("p2.y = {}", p2.y);
    // let p1 = Point { x: 1, y: 2 };
    // let p2 = p1;
    // println!("p1.x = {}", p1.x);
    // println!("p2.y = {}", p2.y);
}
