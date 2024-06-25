#![allow(unused)]
fn add(a: i32, b: i32) -> i32 {
    return a + b;
}

fn print_vec(vec: Vec<i32>) {
    println!("Vector: {:?}", vec);
}
fn create_vec() -> Vec<i32> {
    return vec![1, 2, 3];
}

fn add_vec(vec: Vec<i64>, num: i32) -> i64 {
    let mut prod: i64 = 1;
    for i in vec {
        prod *= i;
    }
    // return prod + num; // Compiler error
    panic!("Can't return!")
}

fn main() {
    println!("Hello, world!");
}
