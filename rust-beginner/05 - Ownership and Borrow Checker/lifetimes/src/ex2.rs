use rand::{self, Rng};
fn rng() -> rand::rngs::ThreadRng {
    rand::thread_rng()
}
#[allow(unused)]
pub fn main() {
    let a = 12;
    let mut b = &a;
    if rng().gen_bool(0.5) {
        let c = 40;
        // Compiler error if not commented out
        // b = &c;
    }
    println!("{}", *b);
}
