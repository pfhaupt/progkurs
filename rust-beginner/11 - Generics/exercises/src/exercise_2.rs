#![allow(unused)]
struct A {
    i: i32
}
#[derive(Debug)]
struct List<A> {
    data: Vec<A>,
}
impl<A> List<A> {
    fn new() -> Self {
        List { data: Vec::new() }
    }
    fn push_a(&mut self, elem: A) {
        self.data.push(elem);
    }
}
pub fn main() {
    let mut list = List::new();
    for i in 0..10 {
        list.push_a(i);
    }
    println!("{:?}", list);
}
