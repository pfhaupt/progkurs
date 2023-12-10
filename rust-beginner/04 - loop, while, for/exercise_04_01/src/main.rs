// Small spoiler: This warning shows that the counter is independent from `a`
//                so we disable it.
#![allow(unused)]

fn main() {
    let mut counter = 0;
    for a in 0..10 {
        for a in 0..10 {
            counter += 1;
        }
    }
    println!("{}", counter);
}