#![allow(unused)]

#[derive(Debug, PartialEq)]
enum Shape {
    Circle,
    Square,
    Rectangle,
}
fn area(shape: Shape) -> &'static str {
    if shape == Shape::Circle { "pi * r^2" }
    else if shape == Shape::Square { "a^2" }
    else if shape == Shape::Rectangle { "a * b" }
    else { "Well, can you tell me what shape is this? :^)" }
}

struct StuffWithShape {
    shape: Shape,
}
trait Area {
    fn area(&self) -> &'static str;
}
impl Area for Shape {
    fn area(&self) -> &'static str {
        if self == &Shape::Circle { "pi * r^2" }
        else if self == &Shape::Square { "a^2" }
        else if self == &Shape::Rectangle { "a * b" }
        else { unreachable!(":^)") }
    }
}
impl Area for StuffWithShape {
    fn area(&self) -> &'static str {
        self.shape.area()
    }
}

pub fn main() {
    let circle = Shape::Circle;
    let square = Shape::Square;
    let rectangle = Shape::Rectangle;

    let stuff_with_circle = StuffWithShape { shape: circle };
    let stuff_with_square = StuffWithShape { shape: square };
    let stuff_with_rectangle = StuffWithShape { shape: rectangle };

    println!("Area of circle: {}", stuff_with_circle.area());
    println!("Area of square: {}", stuff_with_square.area());
    println!("Area of rectangle: {}", stuff_with_rectangle.area());

    println!(""); // for better readability
}