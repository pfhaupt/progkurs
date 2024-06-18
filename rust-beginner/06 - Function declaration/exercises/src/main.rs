#![allow(unused)]
fn is_prime(candidate: u32) -> bool {
    if candidate < 2 { return false; }
    for number in 2..candidate {
        if candidate % number == 0 {
            return false;
        }
    }
    return true;
}
fn looop(start: i32, end: i32, mut counter: i32) {
    if counter <= start {
        counter = start;
        return;
    }
    while counter < end {
        println!("{}", counter);
        counter += 1;
    }
    // Compiler error
    // return counter;
}
fn min(a: i32, b: i32) -> i32 {
    if a < b {
        return a;
    } else {
        return b;
    }
}
// Doesn't compile
// fn square(x: i32) -> i64 {
//     return x * x;
// }

fn main() {
    println!("Hello, world!");
}
