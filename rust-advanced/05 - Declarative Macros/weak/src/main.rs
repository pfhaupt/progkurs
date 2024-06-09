#![allow(unused)]
use std::{cell::RefCell, fmt::Display, rc::{Rc, Weak}};

impl<T: Display> Data<T> {
    fn new(value: T) -> Self {
        Self {
            value,
            next: None
        }
    }
}
impl<T: Display> Drop for Data<T> {
    fn drop(&mut self) {
        println!("Dropped {}", self.value);
    }
}
type Wrap<T> = Weak<RefCell<Data<T>>>;
struct Data<T: Display> {
    value: T,
    next: Option<Wrap<T>>,
}
fn rc_main() {
    let d1 = Data::new(5);
    let rc = Rc::new(RefCell::new(d1));
    rc.borrow_mut().next = Some(Rc::downgrade(&rc));
    println!("count={}", Rc::strong_count(&rc));
}

fn weak_main() {
    let rc = Rc::new(5);
    let wc = Rc::downgrade(&rc);
    drop(rc);
    if let Some(data) = Weak::upgrade(&wc) {
        println!("Data has not been dropped: {}", data);
    } else {
        println!("Data was dropped :(");
    }
}

impl<T: Display> Tree<T> {
    fn new(data: T) -> Self {
        Self {
            data,
            parent: None,
            left: None,
            right: None
        }
    }
}
impl<T: Display> Drop for Tree<T> {
    fn drop(&mut self) {
        println!("Dropped {}", self.data);
    }
}
struct Tree<T: Display> {
    data: T,
    parent: Option<Weak<RefCell<Tree<T>>>>,
    left: Option<Rc<RefCell<Tree<T>>>>,
    right: Option<Rc<RefCell<Tree<T>>>>,
}
fn set_left<T: Display>(parent: Rc<RefCell<Tree<T>>>, left: Rc<RefCell<Tree<T>>>) {
    parent.borrow_mut().left = Some(Rc::clone(&left));
    left.borrow_mut().parent = Some(Rc::downgrade(&parent));
}
fn main() {
    let t1 = Rc::new(RefCell::new(Tree::new(5)));
    let t2 = Rc::new(RefCell::new(Tree::new(10)));
    let t3 = Rc::new(RefCell::new(Tree::new(8)));
    set_left(Rc::clone(&t1), Rc::clone(&t2));
    set_left(Rc::clone(&t3), Rc::clone(&t2));
    println!("t1.strong={}", Rc::strong_count(&t1));
    println!("t2.strong={}", Rc::strong_count(&t2));
    println!("t3.strong={}", Rc::strong_count(&t3));
}
