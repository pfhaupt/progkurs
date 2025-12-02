use std::ops::{Add, Sub};
struct Number<T> {
    value: T,
}
impl<T> Number<T>
where
    T: Add<Output = T> + Sub<Output = T> + Copy,
{
    fn new(value: T) -> Self {
        Number { value }
    }
    fn add(&self, other: &Self) -> Self {
        Number::new(self.value + other.value)
    }
    fn sub(&self, other: &Self) -> Self {
        Number::new(self.value - other.value)
    }
}
pub fn main() {
    let a = Number::new(1);
    let b = Number::new(2);
    let c = a.add(&b);
    let d = a.sub(&b);
    println!("{} + {} = {}", a.value, b.value, c.value);
    println!("{} - {} = {}", a.value, b.value, d.value);
}
