fn main() {
    let n: u32 = 10;
    let mut result: u32 = 1;
    for i in 1..=n {
        result *= i;
    }
    println!("  {}! is {}", n, result);
}
