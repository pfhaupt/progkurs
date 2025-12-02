fn ex1() {
    let a: i32 = 10;
    let b: &i32 = &a;
    println!("{}", *b);
}
fn ex2() {
    let a: u8 = 12;
    let r1: &u8 = &a;
    let r2: &u8 = &a;
    println!("r1 = {}", r1);
    println!("r2 = {}", r2);
}
fn main() {
    let mut a: i16 = 420;
    let b: &mut i16 = &mut a;
    *b = 1337;
    println!("a = {}", a);
}
