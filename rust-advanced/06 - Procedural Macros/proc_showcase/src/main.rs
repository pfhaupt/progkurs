#![allow(unused)]
use showcase::function_like;
use showcase::create_struct;

function_like!(1, 2);

create_struct!(Person age u8 name String);

pub trait OurTrait {
    fn greet(&self);
}

use showcase::log_call;
use showcase::DeriveTrait;
#[derive(DeriveTrait)]
struct Dog;
#[derive(DeriveTrait)]
struct Cat;
#[log_call]
enum Animal {
    Dog(Dog),
    Cat(Cat),
}
#[log_call(greet="yes")]
fn hello() {
    println!("Hello!");
}
fn main() {
    Dog.greet();
    Cat.greet();
}
