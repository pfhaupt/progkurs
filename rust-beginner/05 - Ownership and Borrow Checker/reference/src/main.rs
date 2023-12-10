#![allow(unused)]

fn main() {
    let mut v1: Vec<i32> = vec![1; 100_000_000];
    let v2: &mut Vec<i32> = &mut v1;
    v2.push(1);
    println!("{}", v1.len());
}
