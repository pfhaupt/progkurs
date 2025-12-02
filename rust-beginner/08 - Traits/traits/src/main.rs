#![allow(unused)]

use std::process::Output;
struct Foo {}
impl std::fmt::Display for Foo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "We have implemented the Display trait for Foo!")
    }
}
fn main3() {
    let foo = Foo {};
    println!("{}", foo);
    // println!("{:?}", foo);
    // println!("{:#?}", foo);
}

trait Geometry {
    fn print_area(&self) {
        println!("area={}", self.area());
    }
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
    println!("area = {}", rect.area());
    println!("perimeter = {}", rect.perimeter());
    calculate_geometry(&rect);
    let vector = vec![1, 2, 3];
    // Below line will not compile
    // because Vec<T> does not implement the Geometry trait
    // calculate_geometry(&vector);
}

// More advanced capabilities of traits below

#[derive(Debug, PartialEq, Clone, Copy)]
struct Vector2D {
    x: f64,
    y: f64
}
// Generic implementation for every type T which is copyable
// where f64 * T = f64 is defined we also define Vector * T
impl<T: Copy> std::ops::Mul<T> for Vector2D
where f64: std::ops::Mul<T, Output = f64> {
    type Output = Self;
    fn mul(self, rhs: T) -> Self::Output {
        Self { x: self.x * rhs, y: self.y * rhs }
    }
}
// Implementation of Vector2D + Vector2D
impl std::ops::Add for Vector2D {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}
fn main() {
    let v = Vector2D { x: 1.0, y: 2.0 };
    assert!(v * 10.0 == Vector2D { x: 10.0, y: 20.0 });
    assert!(v * &10.0 == Vector2D { x: 10.0, y: 20.0 });
    assert!(v + v == Vector2D { x: 2.0, y: 4.0 });
}
