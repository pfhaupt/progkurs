# Ownership, References and Borrow Checking

## Solution for Exercise 1: Basic References
```rust
fn main() {
    let a: i32 = 0;
    let b: &i32 = &a;
    println!("a={}", *b);
}
```
## Solution for Exercise 2: Multiple Immutable References
```rust
fn main() {
    let a: i32 = 0;
    let r1: &i32 = &a;
    let r2: &i32 = &a;
    println!("r1={}", r1);
    println!("r2={}", r2);
}
```
## Solution for Exercise 3: Mutable Reference Modification
```rust
fn main() {
    let mut a: i32 = 0;
    let ra: &mut i32 = &mut a;
    *ra = 20;
    println!("a={}", a);
}
```
## Solution for Exercise 4: Ownership Transfer
```rust
fn main() {
    let v: Vec<i32> = vec![1, 2, 3];
    {
        // v is moved to here
        let v1: Vec<i32> = v;
    }
    // can't use v here
    // println!("v={:?}", v);
}
```
## Solution for Exercise 5: Immutable and Mutable References
```rust
fn main() {
    let mut a: i32 = 0;
    let ia: &i32 = &a;
    println!("ia={}", ia);
    let ma: &mut i32 = &mut a;
    *ma = 12;
    // Using ia again causes compiler error
    // println!("ia={}", ia);
}
```
## Solution for Exercise 6: Vector Modification
```rust
fn main() {
    let mut v: Vec<i32> = vec![1];
    while v.len() < 100 {
        let elem: i32 = v[v.len() - 1];
        v.push(elem + 1);
    }
    for elem in &mut v {
        *elem *= *elem;
    }
    println!("v={:?}", v);
}
```
## Solution for Exercise 7: References of References
```rust
fn main() {
    let a: i32 = 0;
    let ra: &i32 = &a;
    println!("ra={}", ra);
    let rra: &&i32 = &ra;
    println!("rra={}", rra);
}
```
## Solution for Exercise 8: Borrowing Elements
```rust
fn main() {
    let mut v: Vec<i32> = vec![1, 2, 3];
    let elem: &mut i32 = &mut v[1];
    *elem *= 2;
    println!("v={:?}", v);
}
```
## Solution for Exercise 9: Reference Loop
```rust
fn main() {
    let a: i32 = 5;
    let mut r: &i32 = &a;
    for _ in 0..10 {
        // Assign a reference to itself
        // Deref coercion makes this line unnecessary, this effectively does nothing
        r = &r;
    }
    // Here, r still has type &i32
    let r1 = &r;
    // r1 has type &&i32
    println!("{}", r1);
}
```
## Solution for Exercise 10: Array References
```rust
fn main() {
    let mut arr: [bool; 5] = [false, true, true, false, false];
    println!("arr={:?}", arr);
    for elem in &mut arr {
        *elem = false;
    }
    println!("arr={:?}", arr);
    for elem in &mut arr {
        *elem = true;
    }
    println!("arr={:?}", arr);
}
```
## Solution for Bonus Challenge: Advanced Vector Manipulation with References
```rust
fn main() {
    let mut v: Vec<i32> = Vec::with_capacity(100);
    for i in 1..=100 {
        v.push(i);
    }

    // Find indices of elements divisible by 2 or 3
    // Store indices in a vector to avoid index shift
    // and double mutable borrow
    let mut indices: Vec<usize> = Vec::new();
    // Iterate over elements of rv
    // rv.iter() returns an iterator over the elements of rv
    // enumerate() returns a tuple (index, element)
    // i is the index, elem is the element
    let rv: &mut Vec<i32> = &mut v;
    for (i, elem) in rv.iter().enumerate() {
        if *elem % 2 == 0 || *elem % 3 == 0 {
            // if we just did rv.remove(i) here, we would get a double mutable borrow
            indices.push(i);
        }
    }
    // Remove elements in reverse order to avoid index shift
    for i in indices.iter().rev() {
        rv.remove(*i);
    }
    
    // Print elements not divisible by 5 or 7
    for elem in &v {
        if *elem % 5 != 0 && *elem % 7 != 0 {
            println!("elem={}", elem);
        }
    }
    println!("v={:?}", v);
}
```