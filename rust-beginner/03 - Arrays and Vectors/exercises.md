# Arrays and Vectors
## Usage
To get started, you'll need a working Rust environment. Open up your go-to editor and follow the instructions in each exercise. The difficulty ratings are there to give you an idea of what to expect, based on what you've learned.

## Goal
The goal of these exercises is to familiarize you with Rust's arrays and vectors. You'll learn to work with indices, manipulate your first data structures, and gain a basic understanding of how arrays and vectors are used in Rust.

## Exercise 1: Reverse Order Array
```
Rating: 1/3 (Yellow)
Task:
1. Create an array named `squares` with the squares of the first five natural numbers (excluding 0).
2. Print the array in reverse order.
Hint:
a. Remember, you can access array elements using indices. The last element of an array `arr` can be accessed with `arr[arr.len() - 1]`.
```

## Exercise 2: Calculating Array Sum
```
Rating: 2/3 (Red)
Task:
1. Create an array of integers [10, 20, 30, 40, 50].
2. Calculate and print the sum of the array's elements.
Hint:
a. Add the elements of the array directly. For example, `sum = arr[0] + arr[1] + ...`
```

## Exercise 3: Vector Initialization with a Range
```
Rating: 1/3 (Yellow)
Task:
1. Initialize a vector using `vec!` to contain the numbers 1 through 5.
2. Print the entire vector.
```

## Exercise 4: Doubling Vector Elements
```
Rating: 2/3 (Red)
Task:
1. Create a mutable vector with integers from 1 to 5.
2. Double each element of the vector.
3. Print the updated vector.
```

## Exercise 5: Vector Element Removal
```
Rating: 2/3 (Red)
Task:
1. Create a vector containing [5, 10, 15, 20, 25].
2. Remove the last element of the vector.
3. Print the updated vector.
```

## Exercise 6: Vector Capacity Exploration
```
Rating: 3/3 (Purple)
Task:
1. Create a vector and gradually add elements to it.
2. Observe and print how the capacity of the vector changes. What do you notice?
```

## Exercise 7: Array of Fixed Size
```
Rating: 1/3 (Red)
Task:
1. Create an array of fixed size 4, containing your favorite numbers.
2. Print the first and last elements of this array.
```

## Exercise 8: Growing a Vector
```
Rating: 1/3 (Yellow)
Task:
1. Start with an empty mutable vector.
2. Add four elements to it, one at a time.
3. Print the vector after each addition.
```

## Exercise 9: Array of Boolean Values
```
Rating: 1/3 (Yellow)
Task:
1. Create an array that holds five boolean values (mix of `true` and `false`).
2. Print out the array.
```
## Exercise 10: Concatenating Two Vectors
```
Rating: 2/3 (Red)
Task:
1. Create two vectors, one with the first three even numbers and the other with the first three odd numbers.
2. Combine these vectors into a single vector by pushing all elements from the odd number vector into the even number vector.
3. Print the combined vector.
```

## Bonus Challenge: Multidimensional Array
```
Rating: 4/3 (Extra difficult)
Task:
1. Represent the seating plan of a small airplane as a multidimensional array with 7 rows, and each row containing 5 seats.
2. Initialize the array with `false` to indicate empty seats.
3. Update the array such that 3 seats are now taken. (set the respective elements to `true`)
4. Print the updated array.
Hint:
a. Arrays in Rust are types too, you can define arrays of arrays like
> let multi_arr: [[i32; 5]; 3] = [[0; 5]; 3];
b. Every index level gets you one step closer to the actual values:
> multi_arr[0] returns a [i32; 5] array
> multi_arr[0][3] then gets the element at index 3 in that array.
c. Printing multidimensional arrays in a human-friendly way is easier once we introduced for-loops, a simple printout just using println! is enough for now.
```