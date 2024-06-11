pub fn main() {
    let mut a: i32 = 0;
    if a == 0 {
        let b: &mut i32 = &mut a;
        *b = 10;
    }
    println!("{}", a);
}
