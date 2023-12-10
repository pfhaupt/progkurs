# Solutions for Basic Types and Variables in Rust

## Solution for Exercise 1: Initialize and print an Integer Variable
```rust
fn main() {
    let age = 20;
    println!("The value of age is: {}", age);
    // age = 21; // Uncommenting this will cause a compile-time error.
}
```

## Solution for Exercise 2: Working with mutable Variables
```rust
fn main() {
    let mut counter = 0;
    counter += 1;
    println!("The value of counter is: {}", counter);
}
```

## Solution for Exercise 3: Type Inference and explicit types
```rust
fn main() {
    let x: i32 = 10;
    let y = 20; // y will have [default] type i32 due to type inference
    println!("The type of y is inferred to be i32");
}
```

## Solution for Exercise 4: Overflow Behavior
```rust
fn main() {
    let mut a: u8 = 255;
    // This will panic in debug mode
    // This will wrap around in release mode
    a += 1;
    println!("The value of a is: {}", a);  // Will print 0
}
```

## Solution for Exercise 5: Basic arithmetics
```rust
fn main() {
    let x = 10;
    let y = 3;
    let z = x / y; // Integer division rounds towards 0
    println!("The value of z is: {}", z);  // Will print 3
}
```

## Solution for Exercise 6: The size of Integer Types
```rust
fn main() {
    let mut small: i8 = 120;
    let large: i128 = 50000;
    // small = 130; // This will cause a compile-time error due to overflow.
}
```

## Solution for Exercise 7: Negative Numbers
```rust
fn main() {
    let positive: u8 = 50;
    // let negative: u8 = -10; // This will cause a compile-time error.
    println!("To store negative numbers, use a signed integer type like i8, i16, i32, etc.");
}
```

## Solution for Exercise 8: Max and Min values of an Integer Type
```rust
fn main() {
    println!("Maximum value for u8: {}", u8::MAX);  // 255
    println!("Minimum value for i8: {}", i8::MIN);  // -128
}
```

## Solution for Exercise 9: Integer Conversion
```rust
fn main() {
    let x: i32 = 1000;
    let y: i8 = x as i8; // Overflow, resulting value would be truncated.
    println!("Value of y: {}", y);
    // Bonus: There's no way of putting 10 bits into 8 bits, you will always lose information. Use a bigger type (f.e. `i16` instead of `i8`) instead.
}
```

## Solution for Exercise 10: Default Values
```rust
fn main() {
    // let x: i32; // Will cause a compile-time error when trying to use `x`.
    let x: i32 = 0; // Initialize with a default value to avoid compile-time error.
    println!("Value of x: {}", x);
    
    // Bonus: Use Option:
    let mut y: Option<i32> = None;
    println!("{:?}", y); // Prints None
    // ... much later
    y = Some(5);
    println!("{:?}", y); // Prints Some(5)
    println!("{}", y.unwrap()); // Prints 5
}
```
