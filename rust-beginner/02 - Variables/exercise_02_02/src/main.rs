#![allow(unused)]

fn main() {
    let a = 0;
    let b: i32 = a;
    let arr: [i32; 2] = [a, b];
    let d = a as usize;
    let e = arr[d];
    println!("{}", e);
} 
