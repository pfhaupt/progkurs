#![allow(unused)]
struct Point {
    x: i32,
    y: i32,
}
struct Line {
    start: Point,
    end: Point
}

impl Line {
    fn length(&self) -> f32 {
        let x = (self.end.x - self.start.x) as f32;
        let y = (self.end.y - self.start.y) as f32;
        f32::sqrt(x * x + y * y)
    }
}
fn main() {
    let p1 = Point { x: 3, y: 4 };
    let p2 = Point { x: 5, y: 10 };
    let line = Line { start: p1, end: p2 };
    println!("length = {}", line.length());
}
