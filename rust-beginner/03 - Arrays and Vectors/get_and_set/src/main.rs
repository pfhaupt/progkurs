#![allow(unused)]
fn get() {
    let arr: [i32; 5] = [10, 20, 30, 40, 50];
    let element: i32 = arr[1];
    println!("arr element = {}", element);

    let vec: Vec<i32> = vec![10, 20, 30, 40, 50];
    let element: i32 = vec[1];
    println!("vec element = {}", element);
}

fn set() {
    let mut arr: [i32; 5] = [10, 20, 30, 40, 50];
    arr[3] = 60;

    let mut vec: Vec<i32> = vec![10, 20, 30, 40, 50];
    vec[3] = 60;
}

fn get_and_set() {
    let mut arr: [i32; 5] = [10, 20, 30, 40, 50];
    arr[3] = 100;
    arr[4] = arr[1] + arr[2];
    arr[3] = arr[0] + arr[3];
    println!("{:?}", arr);
}

fn methods() {
    let mut arr: [i32; 1] = [1];
    arr.is_empty();
    arr.fill(5);
    arr.contains(&5);
    arr.len();
    arr.sort();
    // ... and many more

    let mut vec: Vec<i32> = vec![1];
    // Vectors can do everything arrays can do
    vec.extend(&arr);
    vec.push(5);
    vec.remove(0);
    vec.insert(0, 5);
    // ... and many more
}

fn main() {
    get_and_set();
}
