# Basic Types and Variables
## Usage
To get started, you'll need a working Rust environment. Open up your go-to editor and follow the instructions in each exercise. The difficulty ratings are there to give you an idea of what to expect, based on what you've learned.

## Goal
The aim of these exercises is to get you comfortable with Rust's basic types and variables. You'll be working with integers, write your first programs, and get a feel for Rust's type system.

## Exercise 1: Initialize and print an Integer Variable
```
Rating: 1/3 (Yellow)
Task:
1. Initialize an integer variable named `age` with value 20.
2. Print the value of `age` using the `println!` macro.
3. What happens if you try to change the value of `age`?
```

## Exercise 2: Working with mutable Variables
```
Rating: 1/3 (Yellow)
Task:
1. Declare a mutable integer variable `counter` and initialize it with 0.
2. Increment `counter` by 1.
3. Print the new value of `counter`.
```

## Exercise 3: Type Inference and explicit types
```
Rating: 1.5/3 (Yellowish Red)
Task:
1. Declare an integer variable `x` with an explicit type and assign 10 to it.
2. Declare another variable `y` without a type and assign 20 to it.
3. What inferred type does `y` have?
```

## Exercise 4: Overflow Behavior
```
Rating: 2/3 (Red)
Task:
You're given the code snippet
> let mut a: u8 = 255;

What will happen if we add 1 to `a`?
a. it becomes 256
b. it becomes 0
c. it becomes undefined
d. The program crashes
```

## Exercise 5: Basic arithmetics
```
Rating: 2/3 (Red)
Task:
1. Declare a variable `x` with a value of 10.
2. Declare a variable `y` with a value of 3.
3. Divide `x` by `y` and store the result in `z`.
4. Print `z` to the console. What do you notice?
```

## Exercise 6: The size of Integer Types
```
Rating: 0/3 (Green)
Task:
1. Declare a variable `small` of type i8 and set it to 120.
2. Declare a variable `large` of type i128 and set it to 50000.
3. What happens if you set `small` to 130?
```

## Exercise 7: Negative Numbers
```
Rating: 0/3 (Green)
Task:
1. Declare a variable `positive` of type u8 and set it to 50.
2. Try setting a variable `negative` of type u8 to -10. What happens?
3. What integer type should you use to store negative numbers?
```

## Exercise 8: Max and Min values of an Integer Type
```
Rating: 2/3 (Red)
Task:
1. What is the maximum value that can be stored in a variable of type u8?
2. What is the minimum value that can be stored in a variable of type i8?
3. BONUS: How would you find out these limits programmatically in Rust?
```

## Exercise 9: Integer Conversion
```
Rating: 2/3 (Red)
Task:
1. Declare a variable `x` of type i32 and assign 1000 to it.
2. Declare a variable `y` of type i8.
3. Try converting the value of `x` into `y`. What happens?
4. BONUS: How can you perform this conversion without losing data or causing a panic?
```

## Exercise 10: Default Values
```
Rating: 2/3 (Red)
Task:
1. Declare an integer variable `x` but do not initialize it.
2. Try printing the value of `x`. What happens?
3. BONUS: What must you do to successfully declare an uninitialized variable in Rust?
```