
enum Element {
    Next(String, Box<Element>),
    None,
}

pub fn main() {
    let mut list = Element::None;
    list = Element::Next("Hello".to_string(), Box::new(list));
    list = Element::Next("World".to_string(), Box::new(list));
    list = Element::Next("!".to_string(), Box::new(list));
    let mut current = &list;
    while let Element::Next(value, next) = current {
        println!("{}", value);
        current = next;
    }
}