# Functions
## Usage
To get started, you'll need a working Rust environment. Open up your go-to editor and follow the instructions in each exercise. The difficulty ratings are there to give you an idea of what to expect, based on what you've learned.

## Goal
The goal of these exercises is to familiarize you with Rust's function syntax. You'll learn to spot syntax errors, and learn how to use functions.

## Exercise 1: Even/Odd number
```
Rating: 1/3 (Yellow)
Task:
1. Write a function that fullfills the following requirements:
- It takes one parameter of type u32,
- It returns a boolean,
- The function returns true if the parameter is even, otherwise it returns false.
```
## Exercise 2: Index of smallest element in a Vector
```
Rating: 2/3 (Red)
Task:
1. Write a function that fullfills the following requirements:
- It takes a reference to a Vector of i32 as the parameter,
- It returns an usize,
- The function returns the index of the smallest element in the Vector, or `0` if the Vector is empty.
Examples:
- `f(&vec![1, 2, 3])` returns 0
- `f(&vec![])` returns 0
- `f(&vec![4, -5, 10])` returns 1
```
## Exercise 3: Index of biggest element in a Vector
```
Rating: 2/3 (Red)
Task:
1. Modify the function of Exercise 2 so it does the following:
- It now also takes a second parameter of type boolean,
- If the boolean is true, it returns the biggest element in the Vector, or `0` if the Vector is empty,
- If the boolean is false, it returns the smallest element in the Vector, or `0` if the Vector is empty.
Examples:
- `f(&vec![1, 2, 3], false)` returns 0
- `f(&vec![], true)` returns 0
- `f(&vec![4, -5, 10], true)` returns 2
```
## Exercise 4: Sum of indices
```
Rating: 1/3 (Yellow)
Task:
1. Write a function that fullfills the following requirements:
- It takes a reference to a Vector of i32 as the parameter,
- Using the function declared in Exercise 3, it gets the indices of the smallest and biggest elements in the Vector,
- It returns the sum of both indices.
Examples:
- `f(&vec![1, 2, 3])` returns `smallest + biggest = 0 + 2 = 2`
- `f(&vec![])` returns `smallest + biggest = 0 + 0 = 0`
- `f(&vec![4, -5, 10])` returns `smallest + biggest = 1 + 2 = 3`
```
## Exercise 5: Even sum of indices
```
Rating: 1/3 (Yellow)
Task:
1. Write a function that fullfills the following requirements:
- It takes a reference to a Vector of i32 as the parameter,
- It returns a boolean,
- Using the functions declared in Exercise 1 and 4, it returns true if the sum is even, otherwise false.
Examples:
- `f(&vec![1, 2, 3])` returns `true`
- `f(&vec![])` returns `true`
- `f(&vec![4, -5, 10])` returns `false`
```
## Exercise 6: Sorting a Vector
```
Rating: 2/3 (Red)
Task:
1. Write a function that fullfills the following requirements:
- It takes a Vector of i32 as the parameter,
- It sorts the Vector in ascending order,
- It returns a Vector of i32.
Examples:
- `f(vec![5, 10, 2])` returns `[2, 5, 10]`
- `f(vec![1, 2, 3, 4])` returns `[1, 2, 3, 4]`
- `f(vec![])` returns `[]`
- `f(vec![10, 5, -2])` returns `[-2, 5, 10]`
```
## Exercise 7: Mutable Reference as argument
```
Rating: 2/3 (Red)
Task:
1. You're given the following code snippet:
fn ref_arg(a: &mut u32) -> u32 {
    if *a == 0 {
        return 12;
    }
    *a = *a - 1;
    return ref_arg(a);
}
fn main() {
    let mut a = 10;
    let result = ref_arg(&mut a);
    println!("result={} a={}", result, a);
}
2. Does the code compile?
3. If not, fix the mistakes.
4. Rewrite the function so it does not use recursion anymore.
Hint:
a. For task 4, think about what the function does:
- What is the base case?
- Does it always do the same?
```
## Exercise 8: Odd/Even recursion
```
Rating: 1/3 (Yellow)
Task:
1. You're given the following code snippet:
fn is_odd(a: i32) -> bool {
    return !is_even(a);
}
fn is_even(a: i32) -> bool {
    return !is_odd(a);
}
fn main() {
    let a = 10;
    let result = is_even(a);
    println!("{} is even: {}", a, result);
}
2. Does the code compile?
3. If not, fix the mistakes.
4. Fix the recursive nature of the first two functions.
```
## Exercise 9: Check if given element is prime
```
Rating: 3/3 (Purple)
Task:
1. You're given the following code snippet:
fn is_prime(n: i32) -> bool {
    // Efficient implementation of checking if a number is prime
    // This function does not contain any errors, it's working properly
    // As a bonus, you can try to find out how and why this works
    if n == 2 || n == 3 { return true; }
    if n < 2 || n % 2 == 0 { return false; }
    if n < 9 { return true; }
    if n % 3 == 0 { return false; }
    let r = f64::sqrt(n as f64) as i32;
    let mut f = 5;
    while f <= r {
        if n % f == 0 { return false; }
        if n % (f + 2) == 0 { return false; } 
        f += 6;
    }
    return true;
}
fn element_at_is_prime(vector: Vec<i32>, index: u64) -> bool {
    let element = vector[index];
    return is_prime(element);
}
fn main() {
    let vector = vec![1, 2, 3, 11];
    let index: usize = 3;
    let result = element_at_is_prime(vector, index);
    println!("Index {} of vector {:?} is prime: {}", index, vector, result);
}
2. Does the code compile?
3. If not, fix the mistakes.
Hint:
a. The given function `is_prime()` is correct and does not contain any errors.
b. Remember Ownership rules - What happens if we pass a Vector as an argument?
```
## Exercise 10: Deref Coercion
```
Rating: 1/3 (Yellow)
Task:
1. You're given the following code snippet:
fn downgrade(a: &mut i32) -> &i32 {
    return a;
}
fn main() {
    let mut var: i32 = 10;
    let mut_ref_to_var = &mut var;
    let immut_ref_to_var_1 = downgrade(mut_ref_to_var);
    let immut_ref_to_var_2 = downgrade(mut_ref_to_var);
    println!(
        "immut_1={}, immut_2={}",
        immut_ref_to_var_1,
        immut_ref_to_var_2,
    );
}
2. Does the code compile? Why not?
```