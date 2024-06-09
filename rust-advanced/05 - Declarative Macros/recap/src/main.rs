#![allow(unused)]

use std::rc::Rc;
fn func(rc: Rc<u64>, depth: u8) {
    println!("Function: {}", Rc::strong_count(&rc));
    if depth < 3 {
        func(Rc::clone(&rc), depth + 1);
    }
}
fn rc_main() {
    let data = Rc::new(5u64);
    {
        let rc = Rc::clone(&data);
        println!("Block: {}", Rc::strong_count(&rc));
    }
    func(Rc::clone(&data), 0);
    println!("After: {}", Rc::strong_count(&data));
}
use std::cell::RefCell;
fn ref_main() {
    let data = RefCell::new(42u64);
    {
        let brw = data.borrow();
        println!("Before: {}", brw);
    }
    {
        let mut brw_mut = data.borrow_mut();
        *brw_mut = 100;
        println!("Changed!");
    }
    {
        let brw = data.borrow();
        println!("After: {}", brw);
    }
}

fn change(rc: Rc<RefCell<u64>>, val: u64) {
    let mut brw_mut = rc.borrow_mut();
    *brw_mut = val;
}
fn print(rc: Rc<RefCell<u64>>) {
    let brw = rc.borrow();
    println!("Current: {}", brw);
}
fn reset(rc: Rc<RefCell<u64>>) {
    change(Rc::clone(&rc), 0);
    print(Rc::clone(&rc));
}
fn main() {
    let data = Rc::new(RefCell::new(5u64));
    print(Rc::clone(&data));
    change(Rc::clone(&data), 42);
    print(Rc::clone(&data));
    reset(Rc::clone(&data));
}
