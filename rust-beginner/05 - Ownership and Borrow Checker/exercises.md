# Ownership, References and Borrow Checking
## Usage
To get started, you'll need a working Rust environment. Open up your go-to editor and follow the instructions in each exercise. The difficulty ratings are there to give you an idea of what to expect, based on what you've learned.

## Goal
The goal of these exercises is to familiarize you with Rust's Ownership-Model. You'll learn to work with references, and learn the differences between Move and Copy.

## Exercise 1: Basic References
```
Rating: 1/3 (Yellow)
Task:
1. Create an integer variable.
2. Create a reference to this variable and print its value using the reference.
```

## Exercise 2: Multiple Immutable References
```
Rating: 1/3 (Yellow)
Task:
1. Create an integer variable.
2. Make two immutable references to this variable.
3. Print the values of these references.
```

## Exercise 3: Mutable Reference Modification
```
Rating: 2/3 (Red)
Task:
1. Create a mutable integer variable.
2. Make a mutable reference to the variable.
3. Using this reference, modify the variable's value.
4. Print the new value of the variable.
```

## Exercise 4: Ownership Transfer
```
Rating: 2/3 (Red)
Task:
1. Create a Vector of integers.
2. In a code of block, assign this Vector to another variable.
3. Try to use the original variable outside the block and observe what happens.
```

## Exercise 5: Immutable and Mutable References
```
Rating: 2/3 (Red)
Task:
1. Create a mutable integer variable.
2. Create an immutable reference to this variable, and print it.
3. Create a mutable reference and modify the variable.
4. Try to use the first immutable reference again.
```

## Exercise 6: Vector Modification
```
Rating: 3/3 (Purple)
Task:
1. Create a mutable Vector of integers.
2. While its length is less than 100, push the last element, incremented by one, to it.
3. Using a for-loop, iterate over the Vector and square each element.
4. Print the updated vector.
```

## Exercise 7: References of References
```
Rating: 2/3 (Red)
Task:
1. Create an immutable integer variable.
2. Create an immutable reference to this variable, and print it.
3. Create an immutable reference to the reference, and print it.
```

## Exercise 8: Borrowing Elements
```
Rating: 2/3 (Red)
Task:
1. Create a Vector of integers.
2. Create a mutable reference to the second element.
3. Using that reference, double the element.
4. Print the updated Vector.
```

## Exercise 9: Reference Loop
```
Rating: 3/3 (Purple)
Task:
1. Does the following code snippet compile?
fn main() {
    let a: i32 = 5;
    let mut r: &i32 = &a;
    for _ in 0..10 {
        r = &r;
    }
    let r1 = &r;
    println!("{}", r1);
}
2. What does the code do?
3. What type does `r1` have?
Hint:
a. `_` is a valid variable name in Rust, and is a wildcard pattern. It's a hint for the compiler that you're not using that variable.
```
## Exercise 10: Array References
```
Rating: 1/3 (Yellow)
Task:
1. Create a mutable array of boolean values.
2. Using a for-loop, set all elements to false.
3. Using a for-loop, set all elements to true;
4. Print the array after each step.
```

## Bonus Challenge: Advanced Vector Manipulation with References
```
Rating: 4/3 (Extra difficult)
Task:
1. Create a mutable Vector of integers and initialize it with numbers 1 to 100.
2. Create a mutable reference to the Vector.
3. Write a loop that iterates over the Vector using the mutable reference. For each element, check if it is even or divisible by 3. If so, remove this element.
4. Iterate over the original vector, and print every number not divisible by 5 and not divisible by 7.
4. Print the vector.

Hint:
a. You will need to be careful with indices when removing elements from the Vector during iteration.
b. If you have two conditions and want to check for `cond1 AND cond2`, you can use
> if cond1 && cond2 { }
c. If you have two conditions and want to check for `cond1 OR cond2`, you can use
> if cond1 || cond2 { }
d. To check if number `a` is divisible by `b`, you can use the modulo operator.
> if a % b == 0 { /* a is divisible by b */ }
```
