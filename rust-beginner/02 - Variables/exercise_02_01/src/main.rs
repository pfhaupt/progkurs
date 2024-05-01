fn main() {
    let a: i32 = 0;
    let b: i32 = 0;
    let c: u32 = 0;
    let d: i32 = a + b;
    let e: i32 = b + c;
    let f: u32 = (b as u32) + c;
    println!("{}", f);
}
