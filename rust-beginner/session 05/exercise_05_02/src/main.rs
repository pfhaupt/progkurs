#![allow(unused)]

fn main() {
    let mut vector: Vec<i32> = vec![1, 2];
    let v1: &mut Vec<i32> = &mut vector;
    let v2: &Vec<i32> = &vector;
    v1.push(1);
    println!("{:?}", vector);
}
