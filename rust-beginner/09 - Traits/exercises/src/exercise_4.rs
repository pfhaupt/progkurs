trait Forgettable {
    fn forget(&self);
}
struct Thing { name: String }
impl Forgettable for Thing {
    fn forget(&self) {
        println!("I'm forgetting {}", self.name);
    }
}
pub fn main() {
    let thing = Thing { name: String::from("my thing") };
    thing.forget();
    let thing2 = Thing { name: String::from("my other thing") };
    thing2.forget();
}
