// Rating 2.5/3 - Reddish Purple
// Topic: Types
// Questions:
// a) Does this compile?
// b) What type does `a` have?
// c) Does line 11 change the type of `a`?
fn main() {
    let a = 0;
    let b: i32 = a;
    let arr: [i32; 2] = [a, b];
    let d = a as usize;
    let e = arr[d];
} 
