#![allow(unused)]

// forbidden in Rust
// fn no_overload() {}
// fn no_overload(x: i32) {}

// fn forbidden(x: i32) {}
// fn main() {
//     let n: i32 = 10;
//     forbidden(x: n);
//     forbidden(x = n);
//     forbidden(n);
// }

fn add(n: i32, m: i32) -> i32 {
    return n + m;
}
fn main() {
    let n: i32 = 10;
    let m: i32 = 20;
    let sum: i32 = add(n, m);
    println!("{a} + {} = {}", m, sum, a=n);
}
