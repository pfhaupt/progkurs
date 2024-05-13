#![allow(unused)]

fn stack_vs_heap() {
    let array: [i32; 3] = [10, 45, 90];
    let mut vector: Vec<i32> = Vec::new();
    vector.extend(&array);
}

fn vec_showcase() {
    let mut vec: Vec<i32> = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);
    vec.push(4);
    vec.push(5);
    let size: usize = 10;
    let vec: Vec<i32> = vec![5; size];
    // let array: [i32; size] = [5; size]; // Compiler error
}

fn main() {
}
