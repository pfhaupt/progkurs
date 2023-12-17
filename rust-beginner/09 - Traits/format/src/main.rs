#[derive(Debug)]
struct Person {
    name: String,
    age: u16,
    height: u16,
}
impl std::fmt::Display for Person {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Name: {}, Age: {}, Height: {}",
            self.name, self.age, self.height
        )
    }
}
impl std::ops::Add for Person {
    type Output = Person;
    fn add(self, other: Person) -> Person {
        Person {
            name: format!("{}-{}", self.name, other.name),
            age: self.age + other.age,
            height: self.height + other.height,
        }
    }
}
fn main() {
    let person1 = Person {
        name: String::from("John"),
        age: 32,
        height: 180,
    };
    println!("{}", person1);
    println!("{:?}", person1);
    println!("{:#?}", person1);
    println!("{p}", p = person1);
    let person2 = Person {
        name: String::from("Jane"),
        age: 28,
        height: 160,
    };
    let person3 = person1 + person2;
    println!("{}", person3);
}
