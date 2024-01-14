#![allow(unused)]
use std::fmt::Display;
enum StringElement {
    Next(String, Box<StringElement>),
    None,
}
enum I32Element {
    Next(i32, Box<I32Element>),
    None,
}
enum BoolElement {
    Next(bool, Box<BoolElement>),
    None,
}
enum Element<T> {
    Next(T, Box<Element<T>>),
    None,
}
impl<T: Display + Clone> Element<T> {
    fn print_list(&self) {
        let mut element = self;
        while let Element::Next(value, next) = element {
            println!("The value is: {}", value);
            element = next;
        }
    }
    fn reverse(&self) -> Element<T> {
        let mut element = self;
        let mut result = Element::None;
        while let Element::Next(value, next) = element {
            result = Element::Next((*value).clone(), Box::new(result));
            element = next;
        }
        result
    }
}
fn print_list<A: Display>(element: &Element<A>) {
    let mut element = element;
    while let Element::Next(value, next) = element {
        println!("The value is: {}", value);
        element = next;
    }
}
fn print_i32_list(element: &Element<i32>) {
    let mut element = element;
    while let Element::Next(value, next) = element {
        println!("The value is: {}", value);
        element = next;
    }
}
fn main() {
    let element_i32 = Element::Next(
        1,
        Box::new(Element::Next(2, Box::new(Element::None)))
    );
    let element_string = Element::Next(
        "Hello".to_string(),
        Box::new(Element::Next("World".to_string(), Box::new(Element::None))),
    );
    let element_bool = Element::Next(
        true,
        Box::new(Element::Next(false, Box::new(Element::None))),
    );
    print_list(&element_i32);
    print_list(&element_string);
    print_list(&element_bool);
    // print_i32_list(&element_i32);
    // print_i32_list(&element_string);
    // print_i32_list(&element_bool);
    element_i32.print_list();
    element_string.print_list();
    element_bool.print_list();
    let reversed_element_i32 = element_i32.reverse();
    let reversed_element_string = element_string.reverse();
    let reversed_element_bool = element_bool.reverse();
    reversed_element_i32.print_list();
    reversed_element_string.print_list();
    reversed_element_bool.print_list();
}
