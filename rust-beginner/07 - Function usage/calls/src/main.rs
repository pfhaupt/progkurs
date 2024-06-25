#![allow(unused)]

fn call_this() {
    println!("called call_this()!");
}
fn example1() {
    call_this();
}
fn print_arg(vec: Vec<i32>) {
    println!("{:?}", vec);
}
fn example2() {
    let v: Vec<u8> = vec![1, 2, 3];
    // print_arg(v); // Compiler error
}
fn get_vec() -> Vec<i32> {
    return vec![1, 2, 3];
}
fn example3() {
    let result = get_vec();
    println!("{:?}", result);
}
fn add(a: i32, b: i32) -> i32 {
    return a + b;
}
fn example4() {
    let x = 14;
    let y = 17;
    let z = add(x, y);
    println!("z={}", z);
}
fn sub(a: i32, b: i32) -> i32 {
    return a - b;
}
fn example5() {
    if sub(10, 5) == 3 {
        println!("Something is wrong!");
    }
}
fn print_number(n: i32) {
    println!("{}", n);
}
fn add_one(n: i32) -> i32 {
    return n + 1;
}
fn example6() {
    print_number(add_one(5));
}
fn factorial(n: i32) -> i32 {
    if n <= 1 {
        return 1;
    } else {
        return n * factorial(n - 1);
    }
}
fn example7() {
    let n = 10;
    let result = factorial(n);
    println!("{}! = {}", n, result);
}
fn print_vec(arg: &Vec<i32>) {
    println!("{:?}", arg);
}
fn example8() {
    let v = vec![1, 2, 3];
    print_vec(&v);
    println!("{:?}", v);
}
fn modify_vec(arg: &mut Vec<i32>) {
    arg.push(1);
    if arg.len() < 10 {
        modify_vec(arg);
        arg.push(2);
    }
}
fn example9() {
    let mut vec = vec![1];
    modify_vec(&mut vec);
    println!("{:?}", vec);
}
fn return_ref<'a>() -> &'a i32 {
    let number = 15;
    // return &number;
    panic!("We can't return a reference here!")
}
fn other_func() {
    let x = 12;
}
fn example10() {
    let num = return_ref();
    other_func();
}
fn element_of_vec(vec: &Vec<i32>) -> &i32 {
    return &vec[0]
}
fn example11() {
    let v = vec![10, 1, 2];
    let elem = element_of_vec(&v);
    println!("{}", *elem);
}
fn main() {
    example1();
    example2();
    example3();
    example4();
    example5();
    example6();
    example7();
    example8();
    example9();
    // example10(); // Would panic
    example11();
}
