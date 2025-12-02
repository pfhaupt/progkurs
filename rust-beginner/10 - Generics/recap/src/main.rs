#[allow(unused)]
enum Breed {
    Labrador,
    Rottweiler,
    ShibaInu,
}

fn main1() {
    let dog = Breed::ShibaInu;
    match dog {
        Breed::Labrador => println!("It's a labrador"),
        Breed::Rottweiler => println!("It's a Rottweiler"),
        Breed::ShibaInu => println!("It's a ShibaInu"),
    }
}

fn is_even(num: i32) -> bool {
    num % 2 == 0
}
fn main2() {
    let value = 3;
    match value {
        _ if value < 0 => println!("{} is negative", value),
        _ if is_even(value) => println!("{} is even", value),
        e => println!("{} is odd and greater than zero", e),
    }
}

struct Person {
    name: &'static str,
    age: u8,
}
fn main3() {
    let people = vec![
        Person { name: "Alice", age: 20 },
        Person { name: "Bob", age: 30 },
        Person { name: "Carol", age: 40 },
    ];
    for Person { name, age } in people {
        println!("Name: {}, Age: {}", name, age);
    }
}

enum Number {
    Integer(i32),
    Float(f32),
}
fn main() {
    let numbers = vec![
        Number::Integer(3),
        Number::Float(10.12),
        Number::Integer(5),
    ];
    for number in &numbers {
        match number {
            Number::Integer(n) => println!("Integer: {}", n),
            Number::Float(n) => println!("Float: {}", n),
        }
    }
}