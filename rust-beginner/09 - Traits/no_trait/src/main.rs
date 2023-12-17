#![allow(unused)]
struct Foo {}
impl std::fmt::Display for Foo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "We have implemented the Display trait for Foo!")
    }
}
fn main() {
    let foo = Foo {};
    let s: &str = "123";
    println!("{}", foo);
    // println!("{:?}", foo);
    // println!("{:#?}", foo);
}

trait Geometry {
    fn area(&self) -> f64;
    fn perimeter(&self) -> f64;
}
struct Rectangle {
    width: f64,
    height: f64,
}
impl Geometry for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
    fn perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    }
}
fn calculate_geometry(obj: &impl Geometry) {
    println!("Area: {}", obj.area());
    println!("Perimeter: {}", obj.perimeter());
}
fn main2() {
    let rect = Rectangle { width: 10.0, height: 20.0 };
    calculate_geometry(&rect);
    let vector = vec![1, 2, 3];
    // Below line will not compile
    // because Vec<T> does not implement the Geometry trait
    // calculate_geometry(&vector);
}