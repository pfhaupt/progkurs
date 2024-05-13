#![allow(unused)]

fn main() {
    let x1: i32 = 0;
    let x2: i32 = 17;
    let x3: i32 = 39;
    let x4: i32 = -129;
    let x5: i32 = 41;
    let xs: [i32; 5] = [0, 17, 39, -129, 41];
    // let xs: [i32; 5] = [0, 17, 39, -129, 41, 19]; // Compiler error
    let ys = [x1, x2];
    // let big: [i32; 1000] = [12, 1000]; // Compiler error
    let xy: [[i32; 3]; 3] = [
        [1, 2, 3],
        [4, 5, 6],
        // [4, 5, 6u8],  // Compiler error
        [7, 8, 9]
    ];
    let vec: Vec<i32> = Vec::new();
    println!("{}", x1);
    println!("{}", x2);
    println!("{}", x3);
    println!("{}", x4);
    println!("{}", x5);
}
