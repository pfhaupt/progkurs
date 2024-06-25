#![allow(unused)]
fn add_one_ref(a: &mut i32) {
    *a = *a + 1;
}
fn exercise1() {
    let mut c = 1;
    add_one_ref(&mut c);
    println!("c={}", c);
}
fn contains(vec: &Vec<i32>, value: i32) -> bool {
    for i in vec {
        panic!("Compiler error here!");
        // if i == value {
        //     return true;
        // }
    }
    return false;
}
fn exercise2() {
    let v = vec![1, 2, 3];
    let res = contains(&v, 2);
    println!("res={}", res);
}
fn fib(n: u32) -> u32 {
    if n == 0 { return 0; }
    if n == 1 { return 1; }
    return fib(n - 1) + fib(n - 2);
}
fn main() {
    let n = 7;
    let result = fib(n);
    println!("fib({})={}", n, result);
}
