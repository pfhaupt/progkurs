#![allow(unused)]

fn string_slice() {
    let original = "Hello, World!";
    let slice = &original[0..5];
    println!("slice = `{}`", slice);
}

fn other_slices() {
    let array = [15, 20, 25];
    let vector = vec![10, 15, 20];
    let slice_arr = &array[0..2];
    let slice_vec = &vector[1..3];
    takes_slice(slice_arr);
    takes_slice(slice_vec);
    takes_slice(&[5, 20, 35]);
}

fn takes_slice(slice: &[i32]) {
    println!("length: {}", slice.len());
    println!("elems: {:?}", slice);
}

fn main() {
    string_slice();
    other_slices();
}
