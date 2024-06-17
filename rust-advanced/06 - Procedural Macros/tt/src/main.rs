// https://blog.cloudflare.com/writing-complex-macros-in-rust-reverse-polish-notation

macro_rules! rpn {
    ([ $b:expr, $a:expr $(, $stack:expr)* ] + $($rest:tt)*) => {
        rpn!([ $a + $b $(, $stack)* ] $($rest)*)
    };
    ([ $b:expr, $a:expr $(, $stack:expr)* ] - $($rest:tt)*) => {
        rpn!([ $a - $b $(, $stack)* ] $($rest)*)
    };
    ([ $b:expr, $a:expr $(, $stack:expr)* ] * $($rest:tt)*) => {
        rpn!([ $a * $b $(, $stack)* ] $($rest)*)
    };
    ([ $b:expr, $a:expr $(, $stack:expr)* ] / $($rest:tt)*) => {
        rpn!([ $a / $b $(, $stack)* ] $($rest)*)
    };
    ([ $($stack:expr),* ] $num:literal $($rest:tt)*) => {
       rpn!([ $num $(, $stack)* ] $($rest)*) 
    };
    ([$result:expr]) => { $result };
    ($($t:tt)*) => { rpn!([] $($t)*) }
}
fn main() {
    let a = rpn!(2 3 + 4 *); // expect: 20
    let b = rpn!(15 7 1 1 + - / 3 * 2 1 1 + + -); // expect: 5
    println!("a = {:?}", a);
    println!("b = {:?}", b);
}
