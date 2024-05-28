fn main() {
    let a: i32 = 5;
    let b: i32 = 10;
    println!("a equals b: {}", a == b);
    println!("a does not equal b: {}", a != b);
    println!("a is less than b: {}", a < b); 
    println!("a is less than or equal b: {}", a <= b);
    println!("a is greater than b: {}", a > b); 
    println!("a is greater than or equal b: {}", a >= b);
    println!("a is positive: {}", a.is_positive());

    let c: i32 = 12;
    let d: i32 = 20;
    if c < d {
        println!("c is smaller than d!");
    } else {
        println!("c is not smaller than d!");
    }
    if c == d {
        println!("c is equal to d!");
    }
}
