#[allow(non_snake_case)]
#[allow(unused)]
fn somethingElse() {
    let a = 420;
}
#[allow(unused)]
fn f<'a>() -> &'a i32 {
    let x = 10;
    // return &x;
    panic!("We can't return anything from here!!")
}
pub fn main() {
    let hehe = f();
    println!("{}", *hehe);
    somethingElse();
    println!("{}", *hehe);
}
