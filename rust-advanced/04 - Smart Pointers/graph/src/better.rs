use std::rc::Rc;
use std::cell::RefCell;
struct Graph {
    edges: Vec<Rc<RefCell<Edge>>>,
    nodes: Vec<Rc<RefCell<Node>>>,
}
struct Edge {
    start: Rc<RefCell<Node>>,
    end: Rc<RefCell<Node>>,
}
struct Node {
    id: usize,
    edges: Vec<Rc<RefCell<Edge>>>,
}
impl Edge {
    fn new_rc(start: &Rc<RefCell<Node>>, end: &Rc<RefCell<Node>>)
    -> Rc<RefCell<Self>> {
        let e = Edge {
            start: Rc::clone(start),
            end: Rc::clone(end)
        };
        let re = Rc::new(RefCell::new(e));
        start.borrow_mut().edges.push(Rc::clone(&re));
        end.borrow_mut().edges.push(Rc::clone(&re));
        re
    }
}
impl Node {
    fn new_rc(id: usize) -> Rc<RefCell<Self>> {
        let n = Self {
            id,
            edges: Vec::new()
        };
        Rc::new(RefCell::new(n))
    }
}
impl Drop for Node {
    fn drop(&mut self) {
        println!("Dropping node with id={}", self.id);
    }
}
pub fn main() {
    let mut n1 = Node::new_rc(0);
    let mut n2 = Node::new_rc(1);
    let mut e1 = Edge::new_rc(&n1, &n2);
}
