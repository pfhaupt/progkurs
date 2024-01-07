struct Person {
    name: String,
    age: u8,
}
struct Dog {
    name: String,
    kind: String,
    age: u8,
}
trait Printable {
    fn format(&self) -> String;
}
impl Printable for Person {
    fn format(&self) -> String {
        format!("name: {}, age: {}", self.name, self.age)
    }
}
impl Printable for Dog {
    fn format(&self) -> String {
        format!("name: {}, kind: {}, age: {} Woof! :)", self.name, self.kind, self.age)
    }
}
fn print_trait(p: &impl Printable) {
    println!("{}", p.format());
}

fn main() {
    let p = Person {
        name: String::from("Paul"),
        age: 19,
    };
    let d = Dog {
        name: String::from("Detlef"),
        kind: String::from("Labrador"),
        age: 3,
    };
    print_trait(&p);
    print_trait(&d);
}
