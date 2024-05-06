
fn example_1() {
    let a = vec![1, 2];
    let b = &a;
    println!("{:?}", *b);
}
struct A { a: [i32; 5000] }
fn get(a: &A) {
    println!("a.a is {:?}", a.a);
}
fn example_2() {
    let a = A { a: [20; 5000] };
    get(&a);
}
struct B { a: &A }
fn example_3() {
    let a = A { a: [20; 5000] };
    let b = B { a: &a };
    get(b.a);
}
pub fn main() {
    example_1();
    example_2();
    example_3();
}
