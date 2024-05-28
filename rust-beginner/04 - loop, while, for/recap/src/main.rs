fn main() {
    let multi_arr: [[i32; 4]; 2] = [
        [1, 2, 3, 4],
        [5, 6, 7, 8]
    ];
    // multi_arr[index] returns an Array
    let e: i32 = multi_arr[1][2]; // sets v to 7
    println!("Element at (1, 2) is {}", e);
}
