fn mutable_slice() {
    let mut arr = [1, 2, 3, 4];
    let slice_mut = &mut arr[1..2];
    // let slice = &arr[3..4];
    slice_mut[0] = 1;
    println!("{:?}", arr);
}

pub fn main() {
    let arr: [i32; 5] = [10, 20, 30, 40, 50];
    let slice: &[i32] = &arr[0..2];
    println!("Subarray: {:?}", slice);

    let vec = vec![5, 2, 3];
    let slice: &[u8] = &vec[1..];
    println!("Subvector: {:?}", slice);

    let s = "Hello World!";
    println!("{}", s.replace(&['l', 'r'], "c"));
    
    mutable_slice();

    takes_slice(&arr);
    takes_slice(&arr[2..4]);
    takes_slice(&[1, 2, 3]);
    takes_slice(&vec);
    takes_slice(&vec[..]);
}

fn takes_slice<T: std::fmt::Debug>(slice: &[T]) {
    println!("Slice has size {}", slice.len());
    println!("Slice content: {:?}", slice);
}
