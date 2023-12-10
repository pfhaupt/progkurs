// Rating 1/3 - Yellow
// Topic: Type Inference
// Question: What type does `a` have?
fn main() {
    let a = 5;
    let mut b = a * a;
    let c = [1, 2usize];
    b += c[2];
    println!("{} {} {:?}", a, b, c);
}