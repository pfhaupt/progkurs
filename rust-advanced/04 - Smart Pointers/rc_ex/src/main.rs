#![allow(unused)]

use std::{cell::RefCell, rc::Rc};
struct OurData {
    data: i64,
}
fn main() {
    let original = OurData { data: 15 };
    let rc_orig = Rc::new(original);
    {
        let other = Rc::clone(&rc_orig);
        println!("Inside: {}", Rc::strong_count(&rc_orig));
    }
    println!("Outside: {}", Rc::strong_count(&rc_orig));
    memory_leak();
}

struct Edge {
    start: Rc<RefCell<Node>>,
    end: Rc<RefCell<Node>>,
}
struct Node {
    edges: Vec<Rc<Edge>>,
}
fn memory_leak() {
    let mut n1 = Rc::new(RefCell::new(Node { edges: Vec::new() }));
    let n2 = Rc::new(RefCell::new(Node { edges: Vec::new() }));
    for _ in 0..100 {
        let e1 = Edge {
            start: Rc::clone(&n1),
            end: Rc::clone(&n2)
        };
        n1.borrow_mut().edges.push(Rc::new(e1));
    }
    drop(n1);
    println!("{}", Rc::strong_count(&n2));
}
