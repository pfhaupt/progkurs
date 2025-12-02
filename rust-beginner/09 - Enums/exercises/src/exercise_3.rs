enum Number {
    Int(i32),
    Float(f32),
    Invalid,
}

fn add_numbers(a: Number, b: Number) -> Number {
    match (a, b) {
        (Number::Int(a), Number::Int(b)) => Number::Int(a + b),
        (Number::Float(a), Number::Float(b)) => Number::Float(a + b),
        (Number::Int(a), Number::Float(b)) => Number::Invalid,
        (Number::Float(a), Number::Int(b)) => Number::Invalid,
        // Missing cases for Number::Invalid
    }
}

pub fn main() {
    let one = Number::Int(1);
    let two = Number::Int(2);
    let result = add_numbers(one, two);
    match result {
        Number::Int(value) => println!("1 + 2 = {}", value),
        Number::Float(value) => println!("1 + 2 = {}", value),
        Number::Invalid => println!("Invalid"),
    }
}
