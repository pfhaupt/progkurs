#![allow(unused)]

fn add(a: i32, b: i32) -> i32 {
    return a + b;
}

fn no_ret(a: i32, b: i32) {
    println!("a={}, b={}", a, b);
}
fn no_args() -> i32 {
    return 1;
}
fn fib(n: u32) -> u32 {
    if n == 0 { return 0; }
    if n == 1 { return 1; }
    return fib(n - 1) + fib(n - 2);
}
fn main() {
    let n: u32 = 7;
    let result: u32 = fib(n);
    println!("fib({})={}", n, result);
}
fn mut_ref_arg(a: &mut i32) {
    *a = *a + 1;
}
fn main4() {
    let mut c: i32 = 1;
    mut_ref_arg(&mut c);
    println!("c={}", c);
}
fn ret_ref_arg(a: &mut i32) -> &i32 {
    *a = *a + 1;
    return a;
}
fn main3() {
    // this will cause an error
    // let mut c: i32 = 1;
    // let rc: &mut i32 = &mut c;
    // let c1: &i32 = ret_ref_arg(rc);
    // let c2: &i32 = ret_ref_arg(rc);
    // println!("c1={}", c1);
}
fn contains(vec: &Vec<i32>, value: i32) -> bool {
    for i in vec {
        if *i == value {
            return true;
        }
    }
    return false;
}
fn main2() {
    let v: Vec<i32> = vec![1, 2, 3];
    let result = contains(&v, 2);
    println!("result={}", result);
}
fn main1() {
    let a: i32 = 1;
    let b: i32 = 2;
    let c: i32 = f(a, b) + 10;
    println!("c={}", c);
    if g(a, b) {
        println!("a==b");
    }
    // this will cause a stack overflow
    // dont_call(1);
    let result = factorial(5);
    println!("factorial(5)={}", result);
    let v = vec![1, 2, 3];
    moved_vec(v);
    // v is moved, so this will cause an error
    // println!("v={:?}", v);
    let v = vec![1, 2, 3];
    borrow_vec(&v);
    // no problem, v is still valid
    println!("v={:?}", v);
    let mut v = vec![1, 2, 3];
    println!("before v={:?}", v);
    mut_borrow_vec(&mut v);
    println!("after v={:?}", v);
    let mut v = vec![1, 2, 3];
    mut_borrow_vec_rec(&mut v);
    // let x: &i32 = ret_ref();
    // println!("x={}", x);
}
// this will cause an error
// fn ret_ref() -> &'static i32 {
//     let x: i32 = 1;
//     return &x;
// }
fn mut_borrow_vec_rec(v: &mut Vec<i32>) {
    v.push(1);
    if v.len() < 10 {
        let v2 = v;
        mut_borrow_vec_rec(v2);
        v2.push(2);
    }
}
fn mut_borrow_vec(v: &mut Vec<i32>) {
    v.push(1);
}
fn borrow_vec(v: &Vec<i32>) {
    println!("borrowed the vector");
}
fn moved_vec(v: Vec<i32>) {
    println!("moved the vector");
}
fn factorial(n: i32) -> i32 {
    if n == 0 {
        return 1;
    }
    return n * factorial(n - 1);
}
fn f(x: i32, y: i32) -> i32 {
    return x + y;
}
fn g(x: i32, y: i32) -> bool { return x == y; }
#[allow(unconditional_recursion)] // Hehe
fn dont_call(x: i32) -> i32 { return dont_call(x); }
fn call_this() {
    println!("called call_this()");
}
