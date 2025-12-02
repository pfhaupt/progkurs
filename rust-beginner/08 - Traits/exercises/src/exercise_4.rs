trait Forgettable {
    fn forget(&self);
}
struct Thing { name: String }
impl Thing {
    fn new(name: &str) -> Self {
        Self { name: name.to_string() }
    }
}
impl Forgettable for Thing {
    fn forget(&self) {
        println!("I'm forgetting {}", self.name);
    }
}
impl Forgettable for String {
    fn forget(&self) {
        println!("I'm forgetting {}", self);
    }
}
impl Forgettable for i32 {
    fn forget(&self) {
        println!("I'm forgetting {}", self);
    }
}
pub fn main() {
    let vector: Vec<Box<dyn Forgettable>> = vec![
        Box::new(String::from("hello")),
        Box::new(Thing::new("my thing")),
        Box::new(39),
    ];
    for elem in &vector {
        elem.forget();
    }
}
