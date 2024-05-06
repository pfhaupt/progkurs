fn f(v: &Vec<i32>) -> &i32 {
    &v[0]
}
fn e1() {
    let v = vec![1, 2, 3];
    let first = f(&v);
    println!("{}", *first);
}
fn g<'a>(v1: &'a Vec<i32>, v2: &'a Vec<i32>) -> &'a i32 {
    if v1.len() > v2.len() {
        &v1[0]
    } else {
        &v2[0]
    }
}
fn e2() {
    let v1 = vec![1, 2, 3];
    let v2 = vec![3, 4];
    let first = g(&v1, &v2);
    println!("{}", *first);
}
fn g2<'v1, 'v2, 'r>(v1: &'v1 Vec<i32>, v2: &'v2 Vec<i32>) -> &'r i32
where
    'v1: 'r,
    'v2: 'r,
{
    if v1.len() > v2.len() {
        &v1[0]
    } else {
        &v2[0]
    }
}
fn a(outer: &Vec<i32>) {
    let inner = vec![3, 4];
    let first = g2(outer, &inner);
    println!("{}", *first);
}
fn e3() {
    let orig = vec![1, 2, 3];
    a(&orig);
}
pub fn main() {
    e1();
    e2();
    e3();
}
