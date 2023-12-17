trait Animal {
    fn get_name(&self) -> &String;
    fn make_sound(&self);
}
struct Cat { name: String }
struct Dog { name: String }
impl Animal for Cat {
    fn get_name(&self) -> &String { &self.name }
    fn make_sound(&self) { println!("Meow!"); }
}
impl Animal for Dog {
    fn get_name(&self) -> &String { &self.name }
    fn make_sound(&self) { println!("Woof!"); }
}
pub fn main() {
    let cat = Cat { name: String::from("Misty") };
    cat.make_sound();
    let dog = Dog { name: String::from("Rusty") };
    dog.make_sound();
}
