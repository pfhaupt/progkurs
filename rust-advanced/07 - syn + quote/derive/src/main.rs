#![allow(unused)]
use traits::TypeInfo;

#[derive(TypeInfo)]
struct Custom<'a, T> {
    value: T,
    #[log]
    other: &'a str
}
fn main() {
    println!("{:?}", Custom::<u128>::get_info());
}
