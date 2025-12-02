#![allow(unused)]

fn square(x: i16) -> i16 {
    return x * x;
}

fn many(mut a: i16, bla: u32, x: f32) {
    a = 5;
    println!("{}", bla);
    println!("{}", x);
}

fn none() {
}

fn returns() -> i32 {
    let a: i32 = 18;
    return a;
}

fn invalid() -> i32 {
    let a: i8 = 12;
    // return a;
    panic!("We can't return a!");
}

fn branch(a: i32) -> i32 {
    if a < 0 {
        println!("No return!");
        panic!("We don't return anything!");
    } else {
        return a;
    }
}

fn returns_invalid(a: i32) -> i32 {
    if a < 0 { return 0; }
    while a >= 0 {
        if a == 5 { return 1; }
        // a is not mutable!
        // a -= 1;
    }
    panic!("What if a is 4??");
}

fn ret(mut a: i32) {
    if a <= 0 { return; }
    while a >= 0 {
        if a == 5 { return; }
        a -= 1;
    }
    println!("Hello!");
}

fn main() {
    println!("Hello, world!");
}
