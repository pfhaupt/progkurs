#![allow(unused)]
use std::fmt::Debug;
struct TellMeMoreAbout<T> {
    value: T,
}
trait TellMeMore { fn tell_me_more(&self) -> String; }
impl<T: Debug> TellMeMore for TellMeMoreAbout<T> {
    fn tell_me_more(&self) -> String { format!("{:?}", self) }
}
impl<T: Debug> Debug for TellMeMoreAbout<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "TellMeMoreAbout {{ value: {:?} }}", self.value)
    }
}
fn main() {
    let tell_me_more_about = TellMeMoreAbout { value: 42 };
    println!("{}", tell_me_more_about.tell_me_more());
    let tell_me_more_about = TellMeMoreAbout { value: "Hello World" };
    println!("{}", tell_me_more_about.tell_me_more());
    let tell_me_more_about = TellMeMoreAbout { value: tell_me_more_about };
    println!("{}", tell_me_more_about.tell_me_more());
    let tell_me_more_about = TellMeMoreAbout { value: tell_me_more_about };
    println!("{}", tell_me_more_about.tell_me_more());
}

enum Node<T> {
    None,
    Next(T, Box<Node<T>>),
}
impl<T: std::fmt::Debug> std::fmt::Debug for Node<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Node::None => write!(f, "None"),
            Node::Next(value, next) => write!(f, "Value: ({:?}), Next: [{:?}]", value, next),
        }
    }
}
fn main1() {
    let list: Node<i32> = Node::Next(
        1, Box::new(Node::Next(
            2, Box::new(Node::None))));
    println!("{:?}", list);
    let list: Node<String> = Node::Next(
        "Hello".to_string(), Box::new(Node::Next(
            "World".to_string(), Box::new(Node::None))));
    println!("{:?}", list);
    let list: Node<Node<String>> = Node::Next(
        Node::Next(
            "Hello".to_string(), Box::new(Node::None)),
        Box::new(Node::Next(
            Node::Next(
                "World".to_string(), Box::new(Node::None)),
            Box::new(Node::None))));
    println!("{:?}", list);
}
