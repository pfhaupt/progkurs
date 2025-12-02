use std::f64::consts::PI;

fn main() {
    let a = 0.1;
    let b = 0.2;
    let c = a + b;
    println!("a + b == {}", a + b);
    println!("a + b == 0.3: {}", a + b == 0.3);
    println!("    c == 0.3: {}", c == 0.3);
    println!("c - b == a  : {}", c - b == a);
    let approx = f64::abs((a + b) - c) < f64::EPSILON;
    println!("close enough: {}", approx);

    let mass = 5.972168e27; // g
    let radius: f64 = 637_100_000.0; // cm
    let volume = 4.0 / 3.0 * PI * radius.powi(3);
    let density = mass / volume; // g/cm^3
    println!("Earth's density: {:?}", density);
}
