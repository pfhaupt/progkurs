# Solutions for Arrazs and Vectors

## Solution for Exercise 1: Reverse Order Array
```rust
fn main() {
    let squares: [u8; 5] = [1, 4, 9, 16, 25];
    println!(
        "Reversed order: {} {} {} {} {}",
        squares[4],
        squares[3],
        squares[2],
        squares[1],
        squares[0],
        );
}
```

## Solution for Exercise 2: Calculating Array Sum
```rust
fn main() {
    let integers: [u16; 5] = [10, 20, 30, 40, 50];
    let sum: u16 = integers[0]
                 + integers[1]
                 + integers[2]
                 + integers[3]
                 + integers[4];
    println!("Sum: {}", sum);
}
```

## Solution for Exercise 3: Vector Initialization with a Range
```rust
fn main() {
    let vec: Vec<i32> = vec![1, 2, 3, 4, 5];
    println!("{:?}", vec);
}
```

## Solution for Exercise 4: Doubling Vector Elements
```rust
fn main() {
    let mut vec: Vec<i32> = vec![1, 2, 3, 4, 5];
    vec[0] = vec[0] * 2;
    vec[1] = vec[1] * 2;
    vec[2] = vec[2] * 2;
    vec[3] = vec[3] * 2;
    vec[4] = vec[4] * 2;
    println!("{:?}", vec);
}
```

## Solution for Exercise 5: Vector Element Removal
```rust
fn main() {
    let mut vec: Vec<i32> = vec![5, 10, 15, 20, 25];
    // 2 options:
    vec.remove(vec.len() - 1);
    // or:
    // vec.pop();
    println!("{:?}", vec);
}
```

## Solution for Exercise 6: Vector Capacity Exploration
```rust
fn main() {
    let mut vec: Vec<i32> = vec![];
    while vec.len() < 520 {
        // resize when length == capacity
        vec.push(1);
        // capacity always doubles when resized
        println!("Len: {}, capacity: {}", vec.len(), vec.capacity());
    }
}
```

## Solution for Exercise 7: Array of Fixed Size
```rust
fn main() {
    let numbers: [i16; 4] = [12, 42, 9973, -5];
    println!("First: {}", numbers[0]);
    println!("Last: {}", numbers[numbers.len() - 1]);
}
```

## Solution for Exercise 8: Growing a Vector
```rust
fn main() {
    let mut vec: Vec<i32> = Vec::new();
    vec.push(1);
    println!("{:?}", vec);
    vec.push(10);
    println!("{:?}", vec);
    vec.push(100);
    println!("{:?}", vec);
    vec.push(1001);
    println!("{:?}", vec);
}
```

## Solution for Exercise 9: Array of Boolean Values
```rust
fn main() {
    let arr: [bool; 5] = [true, false, true, true, false];
    println!("{:?}", arr);
}
```

## Solution for Exercise 10: Concatenating Two Vectors
```rust
fn main() {
    let mut v1: Vec<i32> = vec![0, 2, 4];
    let v2: Vec<i32> = vec![1, 3, 5];
    // 2 options:
    v1.push(v2[0]);
    v1.push(v2[1]);
    v1.push(v2[2]);
    // or:
    // v1.extend(v2.iter());
    println!("{:?}", v1);
}
```

## Solution for Bonus Challenge: Multidimensional Array
```rust
fn main() {
    let mut seats: [[bool; 5]; 7] = [[false; 5]; 7];
    seats[0][1] = true;
    seats[3][4] = true;
    seats[2][1] = true;
    println!("{:?}", seats);
    // better print using loops:
    // for row in seats {
    //     println!("{:?}", row);
    // }
}
```