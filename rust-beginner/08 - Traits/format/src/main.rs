#[derive(Debug)]
struct Person { name: String, age: u16, height: u16 }
impl std::fmt::Display for Person {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Name: {}, Age: {}, Height: {}", self.name, self.age, self.height)
    }
}
fn main() {
    let person = Person {
        name: String::from("John"),
        age: 32,
        height: 180,
    };
    let as_str = format!("{}", person);
    println!("{}", as_str);
    println!("{:?}", person);
    println!("{:#?}", person);
    println!("{p}", p = person);
}
