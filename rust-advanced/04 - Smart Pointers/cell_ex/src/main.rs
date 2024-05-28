#![allow(unused)]
use std::cell::{Cell, RefCell};

impl std::fmt::Display for Data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{0}", self.data)
    }
}
struct Data {
    data: i32
}
fn ref_cell() {
    let orig = Data { data: 15 };
    let rc_orig = RefCell::new(orig);
    {
        let borrow = rc_orig.borrow();
        println!("The data is {}", borrow);
    }
    let mut mut_borrow = rc_orig.borrow_mut();
    mut_borrow.data = 100;
    {
        let borrow = rc_orig.borrow();
        println!("The data is {}", borrow);
    }
}

fn main() {
    cell();
    ref_cell();
}

fn cell() {
    let a = Cell::new(5);
    a.set(12);
    println!("a = {}", a.get());
}