# Functions
## Usage
To get started, you'll need a working Rust environment. Open up your go-to editor and follow the instructions in each exercise. The difficulty ratings are there to give you an idea of what to expect, based on what you've learned.

## Goal
The goal of these exercises is to familiarize you with Rust's function syntax. You'll learn to spot syntax errors, and learn how to fix and declare functions.

## Exercise 1: Function with parameters
```
Rating: 1/3 (Yellow)
Task:
1. You're given the following code snippet.
fn func(a: i8, b: i32) -> i32 {
    return a + b;
}
2. Fix the function so that it compiles.
Hint:
a. The return type `i32` is correct.
```

## Exercise 2: Function with Return
```
Rating: 1/3 (Yellow)
Task:
1. You're given the following code snippet.
fn if_sum(a: i32, b: i32, c: i32) {
    if a + b == c {
        return c;
    } else if b + c == a {
        return a;
    }
    return 0;
}
2. Fix the function so that it compiles.
Hint:
a. The expected return type is i32.
```

## Exercise 3: Function with Syntax Errors
```
Rating: 2/3 (Red)
Task:
1. You're given the following code snippet.
fn sum(a: i32, b) - i32 {
    a + b;
}
2. Fix the syntax errors.
Hint:
a. Parameters are similar to variables in that they need a type.
b. The function is expected to return the sum of a and b.
```

## Exercise 4: Function with Vector parameter
```
Rating: 2/3 (Red)
Task:
1. You're given the following code snippet.
fn sum_of_odd_elements(vec: Vec<u32>) -> u32 {
    // ...
}
2. Implement this function.
Hint:
a. The function is expected to return the sum of all odd elements in the vector. For example, if the vector is
> vec![1, 5, 10, 12]
then the function should return 1 + 5 = 6.
```

## Exercise 5: Function with Vectors
```
Rating: 3/3 (Purple)
Task:
1. You're given the following code snippet.
fn compare(vec1: Vec<i32>, vec2: Vec<i32>) -> bool {
    // return true if:
    // - both vectors have the same element count
    // - at every index, both elements have the same value
    // false otherwise
}
2. Implement this function.
```

## Exercise 6: Calculate Average
```
Rating: 1/3 (Yellow)
Task:
1. You're given the following code snippet.
fn calculate_average(numbers: Vec<i32>) -> f32 {
    let sum = 0;
    for num in numbers {
        sum += num;
    }
    return sum / numbers.len();
}
2. Fix the function so that it correctly calculates and returns the average of the numbers in the vector.
Hint:
a. Pay attention to how the sum is being calculated and the types involved in the division.
b. Rust requires explicit type conversion in certain cases. Make sure the types used in the division operation are compatible for calculating a float average.
```

## Exercise 7: Primality Test, revisited
```
Rating: 3/3 (Purple)
Task:
1. You're given the following code snippet.
fn is_prime(p: u32) -> bool {
    if p < 2 {
        return true;
    }
    for c in 2..=p {
        if c % p == 0 {
            return true;
        }
    }
    return false;
}
2. The function is currently broken. Fix it so it returns the correct results.
Hint:
a. There are both syntactical and logical errors in this code.
```

## Exercise 8: Recursive Function
```
Rating: 3/3 (Purple)
Task:
1. Write a function that calculates the factorial of a number without using any loops.
Hint:
a. The mathematical definition of Factorial may help you.
b. To sum the first n Integers recursively, you'd write it like this:
fn sum(n: u32) -> u32 {
    if n == 0 { return 0; }
    // sum is current number plus previous sum
    return n + sum(n - 1);
}
```

## Exercise 9: Modularity Test
```
Rating: 2/3 (Red)
Task:
1. Write a function that fullfills the following requirements:
- It takes one parameter of type i32
- It returns a boolean
- The function returns true if the parameter is either:
    - negative
    - divisible by 137
    - the square of an integer
```

## Exercise 10: Vector Product
```
Rating: 2/3 (Red)
Task:
1. Write a function that fullfills the following requirements:
- It takes one parameter of type Vec<i64>
- It takes one parameter of type i64
- It returns an i64
- It adds the second parameter to the product of all vector elements, and returns that sum.
```