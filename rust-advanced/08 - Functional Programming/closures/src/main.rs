#![allow(unused)]
fn print(arg: i32) {
    println!("arg is {}", arg);
}
fn call_func(func: fn(i32), arg: i32) {
    func(arg);
}
fn filter<T>(elems: Vec<T>, predicate: fn(&T) -> bool) -> Vec<T> {
    let mut new_elems = vec![];
    for e in elems {
        if predicate(&e) {
            new_elems.push(e);
        }
    }
    new_elems
}
fn is_even(n: &i32) -> bool { n % 2 == 0 }
fn named_functions() {
    let vec = vec![1, 2, 3, 4, 5];
    println!("original: {:?}", vec);
    let even_square = map(filter(vec, is_even), square);
    println!("modified: {:?}", even_square);
}
fn map<T, E>(elems: Vec<T>, func: fn(&T) -> E) -> Vec<E> {
    let mut new_elems = Vec::with_capacity(elems.len());
    for e in elems {
        new_elems.push(func(&e));
    }
    new_elems
}
fn square(n: &i32) -> i32 { n * n }

fn square_closure() {
    let square = |n: i32| n * n;
    let n = (|n|n*n)(5);
    println!("n={}", n);
}
fn closure_simple() {
    let n = 7;
    let add_n = |x| x + n;
    let start = 3;
    let result = add_n(start);
    println!("{}+{}={}", start, n, result);
}
fn fn_showcase() {
    let mut s = String::from("Big Data");
    let add_size_of_s = |x| x + s.len();
    println!("{}", add_size_of_s(5));
}
fn fn_mut_showcase() {
    let mut s = String::from("Big Data");
    let mut push_to_str = |what: &str| {
        s.push_str(what)
    };
    push_to_str(", now expanded!");
    println!("{}", s);
}
fn fn_once_showcase() {
    let s = String::from("Big Data");
    let move_str = || {
        let x = s;
        println!("{}", x);
    };
    move_str();
    // println!("{}", s);
}
fn multiply_by(num: i32) -> impl Fn(i32) -> i32 {
    move |n| n * num
}
fn closure_builder() {
    let result = multiply_by(5)(multiply_by(7)(3));
    println!("{}", result);
}
fn filter_2<T, F>(elems: Vec<T>, mut predicate: F) -> Vec<T>
where F: FnMut(&T) -> bool {
    let mut new_elems = vec![];
    for e in elems {
        if predicate(&e) {
            new_elems.push(e);
        }
    }
    new_elems
}
fn main() {
    let vec = vec![6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
    let evens = filter_2(vec.clone(), |n| n % 2 == 0);
    let odds = filter_2(vec.clone(), |n| n % 2 == 1);
    let mut not_prime = vec![];
    let primes = filter_2(vec.clone(), |n| {
        for i in 2..*n {
            if n % i == 0 {
                not_prime.push(*n);
                return false;
            }
        }
        true
    });
    println!("original:  {:?}", vec);
    println!("even:      {:?}", evens);
    println!("odds:      {:?}", odds);
    println!("not_prime: {:?}", not_prime);
    println!("primes:    {:?}", primes);
}
