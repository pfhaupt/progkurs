#![allow(unused)]
// Disable the below warning to see how the compiler complains about the
// floating point literals in the match arms. :^)
// The compiler does not like it at all. :D
#![allow(illegal_floating_point_literal_pattern)]

#[derive(Debug)]
enum Shape {
    Circle(f64),
    Square(f64, f64),
    Rectangle(f64, f64),
    Triangle(f64, f64, f64),
}

fn area_formula(shape: &Shape) -> &'static str {
    match shape {
        Shape::Circle(_) => "pi * r^2",
        Shape::Square(_, _) => "a^2",
        Shape::Rectangle(5.0, _) => "5 * b",
        Shape::Rectangle(_, _) => "a * b",
        Shape::Triangle(_, _, _) => "sqrt(s(s-a)(s-b)(s-c)) where s = (a+b+c)/2",
        e => todo!("We can't get the formula for the area of {e:?} yet!")
    }
}

fn area_calculation(shape: &Shape) -> f64 {
    match shape {
        Shape::Circle(r) => std::f64::consts::PI * r * r,
        Shape::Square(a, _) => a * a,
        Shape::Rectangle(a, b) => a * b,
        Shape::Triangle(a, b, c) => {
            let s = (a + b + c) / 2.0;
            (s * (s - a) * (s - b) * (s - c)).sqrt()
        },
        e => todo!("We can't calculate the area of {e:?} yet!")
    }
}

fn print_circle_descr_match(shape: &Shape) {
    match shape {
        Shape::Circle(r) => {
            let area = area_calculation(shape);
            match area {
                0.0..=5.0 =>
                    println!("Small circle with radius {} and area {}.", r, area),
                5.0..=10.0 =>
                    println!("Medium circle with radius {} and area {}.", r, area),
                10.0..=std::f64::INFINITY =>
                    println!("Big circle with radius {} and area {}.", r, area),
                _ => println!("Circle with radius {}.", r),
            }
        },
        _ => println!("Not a circle."),
    }
}

fn print_circle_descr_if_let(shape: &Shape) {
    if let Shape::Circle(r) = shape {
        let area = area_calculation(shape);
        if area <= 5.0 {
            println!("Small circle with radius {} and area {}.", r, area);
        } else if area <= 10.0 {
            println!("Medium circle with radius {} and area {}.", r, area);
        } else {
            println!("Big circle with radius {} and area {}.", r, area);
        }
    } else {
        println!("Not a circle.");
    }
}

fn print_circle_descr_let(shape: &Shape) {
    let Shape::Circle(r) = shape else {
        println!("Not a circle.");
        return;
    };
    assert!(matches!(shape, Shape::Circle(_)), "This is ALWAYS a circle.");
    let area = area_calculation(shape);
    match area {
        0.0..=5.0 =>
            println!("Small circle with radius {} and area {}.", r, area),
        5.0..=10.0 =>
            println!("Medium circle with radius {} and area {}.", r, area),
        10.0..=std::f64::INFINITY =>
            println!("Big circle with radius {} and area {}.", r, area),
        _ => println!("Circle with radius {}.", r),
    }
}

fn modify_shape(shape: &mut Shape) {
    match shape {
        Shape::Circle(r) => *r = 2.0 * *r,
        Shape::Square(a, b) => {
            *a = 2.0 * *a;
            *b = 2.0 * *b;
        },
        Shape::Rectangle(a, b) => {
            *a = 2.0 * *a;
            *b = 2.0 * *b;
        },
        Shape::Triangle(a, b, c) => {
            *a = 2.0 * *a;
            *b = 2.0 * *b;
            *c = 2.0 * *c;
        },
    }
}

pub fn main() {
    let circle = Shape::Circle(3.0);
    let zero_circle = Shape::Circle(0.0);
    let square = Shape::Square(3.0, 3.0);
    let rectangle_5 = Shape::Rectangle(5.0, 3.0);
    let rectangle = Shape::Rectangle(3.0, 4.0);
    let triangle = Shape::Triangle(3.0, 4.0, 5.0);

    println!("Area of circle: {}", area_formula(&circle));
    println!("Area of square: {}", area_formula(&square));
    println!("Area of rectangle: {}", area_formula(&rectangle_5));
    println!("Area of rectangle: {}", area_formula(&rectangle));
    println!("Area of triangle: {}", area_formula(&triangle));
    println!(""); // for better readability

    println!("Area of circle: {}", area_calculation(&circle));
    println!("Area of square: {}", area_calculation(&square));
    println!("Area of rectangle: {}", area_calculation(&rectangle_5));
    println!("Area of rectangle: {}", area_calculation(&rectangle));
    println!("Area of triangle: {}", area_calculation(&triangle));
    println!(""); // for better readability

    print_circle_descr_match(&circle);
    print_circle_descr_match(&zero_circle);
    print_circle_descr_match(&square);
    println!(""); // for better readability

    print_circle_descr_if_let(&circle);
    print_circle_descr_if_let(&zero_circle);
    print_circle_descr_if_let(&square);
    println!(""); // for better readability

    print_circle_descr_let(&circle);
    print_circle_descr_let(&zero_circle);
    print_circle_descr_let(&square);
    println!(""); // for better readability

    let mut circle = Shape::Circle(3.0);
    let mut square = Shape::Square(3.0, 3.0);
    let mut rectangle = Shape::Rectangle(3.0, 4.0);
    let mut triangle = Shape::Triangle(3.0, 4.0, 5.0);

    println!("Before: Area of circle: {}", area_calculation(&circle));
    println!("Before: Area of square: {}", area_calculation(&square));
    println!("Before: Area of rectangle: {}", area_calculation(&rectangle));
    println!("Before: Area of triangle: {}", area_calculation(&triangle));
    println!(""); // for better readability

    modify_shape(&mut circle);
    modify_shape(&mut square);
    modify_shape(&mut rectangle);
    modify_shape(&mut triangle);

    println!("After: Area of circle: {}", area_calculation(&circle));
    println!("After: Area of square: {}", area_calculation(&square));
    println!("After: Area of rectangle: {}", area_calculation(&rectangle));
    println!("After: Area of triangle: {}", area_calculation(&triangle));
    println!(""); // for better readability
}
