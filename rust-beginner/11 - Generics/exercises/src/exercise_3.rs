
trait From<T> {
    fn from_t(a: T) -> Self;
}
struct Number {
    value: i32,
}
impl From<i32> for Number {
    fn from_t(value: i32) -> Self {
        Number { value }
    }
}
impl From<f32> for Number {
    fn from_t(value: f32) -> Self {
        Number { value: value as i32 }
    }
}
pub fn main() {
    let a = Number::from_t(1);
    let b = Number::from_t(2.0);
    let c = Number::from_t("Hallo!");
    println!("{} + {} = {}", a.value, b.value, a.value + b.value);
}
