#![allow(unused)]
pub fn main() {
    let mut r;
    {
        let x = vec![2];
        r = &x;
        println!("{:?}", *r);
    }
    {
        let y = vec![2];
        r = &y;
        println!("{:?}", *r);
    }
}
