#![allow(unused)]

use std::mem;
struct BinaryTree<T> {
    value: T,
    left: Option<Box<BinaryTree<T>>>,
    right: Option<Box<BinaryTree<T>>>,
}
struct Person {
    name: &'static str,
    age: u8
}
struct BigStruct {
    much_data: [i32; 50_000]
}
fn takes_ref(data: &BigStruct) {
    
}
fn move_data(data: Box<BigStruct>) -> Box<BigStruct> {
    // A lot of work, pew
    data
}
fn main() {
    let person = Person {
        name: "Peter",
        age: 27
    };
    let boxed = Box::new(person);

    println!("{}", mem::size_of::<BinaryTree<i32>>());

    let mut big = Box::new(BigStruct {
        much_data: [0; 50_000]
    });
    takes_ref(&big);
    for _ in 0..100_000 {
        big = move_data(big);
    }
}
