#![allow(unused)]

fn random_bool() -> bool {
    use rand::{thread_rng, Rng};
    thread_rng().gen_bool(0.5)
}

fn lifetime_violation() {
    let r;
    {
        let x = 12;
        r = &x;
    }
    // println!("{}", *r); // Compiler error
}

fn lifetime_memory() {
    let a = 12;
    let mut b = 20;
    let mut r = &b;
    if random_bool() {
        r = &a;
        b = 20;
    }
    println!("{}", *r);
}

fn first<'v, T>(
    v1: &'v Vec<T>, v2: &'v Vec<T>
) -> &'v T {
    if random_bool() { &v1[0] }
    else             { &v2[0] }
}
fn lifetime_func() {
    let v1 = vec![5u8, 10];
    let v2 = vec![3, 5];
    let res = first(&v1, &v2);
    println!("res = {}", *res);
}

fn main() {
    // lifetime_memory();
    lifetime_func();
}