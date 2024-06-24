#![allow(unused)]
#![allow(non_camel_case_types)]

use macros::inject;

#[inject(replace(u8, i8))]
struct Person {
    age: u8,
    name: &'static str
}
struct Cat;
struct Dog;
#[inject(replace(Cat, i32))]
enum Animal {
    Cat(Cat),
    Dog(Dog)
}
#[inject(
    payload="meow!",
    replace(mut),
    replace(const, let mut),
    replace(i32, u16),
)]
fn main() {
    let mut person: Person = Person {
        age: 24,
        name: "Max"
    };
    // person.age = 12;
    const a: i32 = 1;
    a = 300;
    println!("{a}");
    println!("{a - 5}");
}
