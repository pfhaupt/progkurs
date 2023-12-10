fn main() {
    // Declaration
    let multi_arr: [[i32; 3]; 3] = [
        [1, 2, 3],
        [4, 5, 6],
        [7, 8, 9]
    ];
    // multi_arr[index] returns an Array
    let v: i32 = multi_arr[1][2]; // sets v to 6
    println!("element at (1, 2) is {}", v);
}
