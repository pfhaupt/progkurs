#![allow(unused)]

fn e1() {
    let a = 20;
    // a = 10;
    let mut b: u8 = 12;
    // b = -29;
}

fn e2() {
    let a = 330;
    let b = a + 26;
    let c = b as u8;
    println!("c = {}", c);
}

fn main() {
    let min_i8: i8 = i8::MIN;
    let max_u128: u128 = u128::MAX;
    let u64_bits: u32 = u64::BITS;
}
