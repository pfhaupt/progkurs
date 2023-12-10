# Loops

## Solution for Exercise 1: Basic Loop
```rust
fn main() {
    let mut count = 5;
    loop {
        if count == 0 {
            break;
        }
        println!("Rust is fun!");
        count -= 1;
    }
}
```

## Solution for Exercise 2: While Loop with Counter
```rust
fn main() {
    let mut counter = 1;
    while counter <= 5 {
        println!("{}", counter);
        counter += 1;
    }
}
```

## Solution for Exercise 3: For Loop Over a Range
```rust
fn main() {
    for n in 1..=10 {
        println!("{}", n);
    }
}
```

## Solution for Exercise 4: Loop with Break
```rust
fn main() {
    let mut counter = 1;
    loop {
        println!("{}", counter);
        if counter == 10 {
            break;
        }
        counter += 1;
    }
}
```

## Solution for Exercise 5: Nested Loops
```rust
fn main() {
    for i in 0..3 {
        for j in 0..5 {
            println!("i={} j={}", i, j);
        }
    }
}
```

## Solution for Exercise 6: Summation with a For Loop
```rust
fn main() {
    let mut sum = 0;
    for i in 1..=100 {
        sum += i;
    }
    println!("{}", sum);
}
```

## Solution for Exercise 7: Reversing an Array with a Loop
```rust
fn main() {
    let mut arr = [1, 2, 3, 4, 5];
    for index in 0..(arr.len()/2) {
        let t = arr[index];
        arr[index] = arr[arr.len() - index - 1];
        arr[arr.len() - index - 1] = t;
    }
    println!("{:?}", arr);
}
```

## Solution for Exercise 8: Finding a Number with While Loop
```rust
fn main() {
    let arr = [1, 15, 3, 4];
    let mut index = 0;
    let mut found = false;
    while index < arr.len() {
        if arr[index] == 15 {
            println!("found at index {}", index);
            found = true;
            break;
        }
        index += 1;
    }
    if !found {
        let mut sum = 0;
        for elem in arr {
            sum += elem;
        }
        println!("sum={}", sum);
    }
}
```

## Solution for Exercise 9: Loop Control with Continue
```rust
fn main() {
    for i in 1..20 {
        if i % 2 == 0 {
            continue;
        }
        println!("{}", i);
    }
}
```

## Solution for Exercise 10: Infinite Loop with User Break
```rust
fn main() {
    let mut counter = 0;
    loop {
        if counter == 42 {
            break;
        }
        counter += 1;
    }
}
```

## Solution for Exercise 11: Loop Through a Vector
```rust
fn main() {
    let vector = vec![1, 2, 3, 10];
    for elem in vector {
        println!("{}", elem);
    }
}
```

## Solution for Exercise 12: Countdown with While Loop
```rust
fn main() {
    let mut counter = 10;
    while counter >= 1 {
        println!("{}", counter);
        counter -= 1;
    }
}
```

## Solution for Exercise 13: Nested Loop to Print a Pattern
```rust
fn main() {
    // square
    for _ in 0..10 {
        for _ in 0..10 {
            print!("#");
        }
        println!();
    }
    println!("-------------");
    // triangle
    for i in 0..10 {
        for _ in 0..i {
            print!("#");
        }
        println!();
    }
}
```

## Solution for Exercise 14: Looping Through a Multidimensional Array
```rust
fn main() {
    let arr = [
        [1, 2, 3],
        [4, 5, 6],
        [7, 8, 9]
    ];
    for row in arr {
        for elem in row {
            println!("{}", elem);
        }
    }
}
```

## Solution for Exercise 15: Factorial with a For Loop
```rust
fn main() {
    let n = 5;
    let mut result = 1;
    for i in 1..=n {
        result *= i;
    }
    println!("{}", result);
}
```

## Solution for Bonus Challenge: Prime Numbers
```rust
fn main() {
    let mut prime_counter = 0;
    let mut candidate = 2;
    while prime_counter != 10 {
        let mut is_prime = true;
        // naive, trial by division
        for n in 2..candidate {
            if candidate % n == 0 {
                is_prime = false;
                break;
            }
        }
        if is_prime {
            println!("{} is prime!", candidate);
            prime_counter += 1;
        }
        candidate += 1;
    }
    // Advanced:
    // Keep track of found primes
    // only check found primes
    /*
    let mut primes = vec![];
    let mut candidate = 2;
    while primes.len() != 10 {
        let mut is_prime = true;
        for found in &primes {
            if candidate % *found == 0 {
                is_prime = false;
                break;
            }
        }
        if is_prime {
            println!("{} is prime!", candidate);
            primes.push(candidate);
        }
        candidate += 1;
    }
    */
}
```
