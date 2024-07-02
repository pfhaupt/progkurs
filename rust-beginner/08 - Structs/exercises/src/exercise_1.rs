struct Point {
    x: i32,
    y: i32,
    // This causes an error:
    // y: u32
}
pub fn main() {
    let p = Point { x: 1, y: 2 };
    println!("p.x = {}", p.x);
    println!("p.y = {}", p.y);
}
