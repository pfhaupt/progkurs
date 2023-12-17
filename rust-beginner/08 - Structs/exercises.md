# Structs
## Usage
To get started, you'll need a working Rust environment. Open up your go-to editor and follow the instructions in each exercise. The difficulty ratings are there to give you an idea of what to expect, based on what you've learned.

## Goal
The goal of these exercises is to familiarize you with Rust's struct syntax. You'll learn how to use structs.

## Exercise 1: Points in 3D
```
Rating: 1/3 (Yellow)
Task:
1. Write a struct `Point3D` that represents a point in 3D space with floating point accuracy.
2. Create 2 instances of Point3D.
```
## Exercise 2: Lines in 3D
```
Rating: 1/3 (Yellow)
Task:
1. Using the struct defined in Exercise 1, write a struct `Line3D` that represents a line in 3D space.
2. Create 3 instances of Line3D.
Hint:
a. Lines can be defined by providing their start and end points.
```
## Exercise 3: Length of Lines in 3D
```
Rating: 2/3 (Red)
Task:
1. Using the struct defined in Exercise 2, write a method that returns the length of a given Line3D-instance.
2. Create a Line3D-instance from (1, 2, 3) to (4, 5, 6).
3. Create a Line3D-instance from (4, 5, 6) to (1, 2, 3).
4. Check if both lines have the same length.
```
## Exercise 4: Rectangles in 3D
```
Rating: 2/3 (Red)
Task:
1. Using the struct defined in Exercise 2, write a struct `Rect3D` that represents a rectangle in 3D space.
2. Create 2 instances of Rect3D.
3. Write a method that returns a Vector of all Point3Ds of the rectangle.
```
## Exercise 5: Boxes in 3D
```
Rating: 2/3 (Red)
Task:
1. Using the struct defined in Exercise 4, write a struct `Box3D` that represents a box in 3D space.
2. Create 1 instance of Box3D.
3. Write a method that returns a Vector of all Line3Ds of the box.
```
## Exercise 6: Efficient Boxes in 3D
```
Rating: 3/3 (Purple)
Task:
1. Storing the rectangles of Box3D has a lot of repetition in the stored Point3Ds.
2. Write a struct `EffBox3D` that represents a box in 3D space using only Point3Ds.
3. Write an associated function that takes in a Box3D and converts it to EffBox3D.
```
## Exercise 7: Points along a Line
```
Rating: 2/3 (Red)
Task:
1. Using the struct defined in Exercise 2, write a method that does the following:
- It takes in a parameter of type `usize` called `count`.
- It returns a Vector of Point3D.
- The Vector contains evenly spaced Point3Ds between the start and end of the line.
Hint:
a. Using the formula `Direction = Start - End` and `P = Start + x * Direction` you can calculate any point along the line.
```
## Exercise 8: Pets
```
Rating: 1/3 (Yellow)
Task:
1. Write a struct `Cat` that has a name and an age.
2. Write a struct `Fish` that has a name.
3. Write a method that makes the cat meow.
4. Write a method that lets the cat eat a fish.
Hint:
a. `Eating` the fish should also extend to the Ownership-part of Rust, the method should not take a reference.
```
## Exercise 9: Humans with Pets, Part I
```
Rating: 1/3 (Yellow)
Task:
1. Write a struct `Human` that has a name, an age and a height.
2. Using the struct defined in Exercise 8, write a method that does the following:
- It takes in a parameter of type &Vec<Cat> called `cats`.
- It returns an `usize` called `counter`.
- For every Cat in the Vector, print `The cat {cat_name} has been petted. :)` and increase the counter.
```
## Exercise 10: Humans with Pets, Part II
```
Rating: 2/3 (Red)
Task:
1. Using the struct defined in Exercise 9, extend it so it now keeps track of a Vector of cats.
2. Write a method that feeds all cats in that Vector.
```