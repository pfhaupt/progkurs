# Traits
## Usage
To get started, you'll need a working Rust environment. Open up your go-to editor and follow the instructions in each exercise. The difficulty ratings are there to give you an idea of what to expect, based on what you've learned.

## Goal
The goal of these exercises is to familiarize you with Rust's trait syntax. You'll learn to spot syntax errors, and learn how to use traits.

## Exercise 1: Age of Person
```
Rating: 1/3 (Yellow)
Task:
1. You're given the following code snippet:
trait Age {
    fn print(&self);
}
struct Person {
    age: u8
}
2. Implement the trait for Person. It should print the age of the Person.
```
## Exercise 2: Height of Person
```
Rating: 1/3 (Yellow)
Task:
1. You're given the following code snippet:
trait Height {
    fn print(&self);
}
struct Tree {
    height: u8
}
2. Implement the trait for Tree. It should print the height of the Tree.
```
## Exercise 3: Geometry
```
Rating: 2/3 (Red)
Task:
1. You're given the following code snippet:
trait Geometry {
    fn area(&self) -> f32;
    fn perimeter(&self) -> f32;
}
struct Rectangle {
    width: f32,
    height: f32
}
struct Circle {
    x: f32,
    y: f32,
    diameter: f32
}
2. Implement the trait for both structs.
```
## Exercise 4: Recursive Clone
```
Rating: 2/3 (Red)
1. You're given the following code snippet:
struct Cat {
    name: String,
    age: u8
}
struct Human {
    name: String,
    age: u8,
    cats: Vec<Cat>
}
2. Implement the Clone-trait for both structs, without deriving them.
```
## Exercise 5: Student
```
Rating: 2/3 (Red)
Task:
1. You're given the following code snippet:
trait Human {
    fn name(&self) -> String;
    fn age(&self) -> u8;
}
trait Programmer: Human {
    fn fav_language(&self) -> String;
}
struct Student {
    name: String,
    age: u8,
}
2. Implement both traits for the Student.
```
## Exercise 6: Equality
```
Rating: 2/3 (Red)
Task:
1. You're given the following code snippet:
trait Equality {
    fn equals(&self, other: &Self) -> bool;
    fn not_equals(&self, other: &Self) -> bool {
        !self.equals(other)
    }
}
struct Point2D {
    x: i32,
    y: i32
}
struct Line2D {
    start: Point2D,
    end: Point2D
}
2. Implement the trait for both structures. For Line2D, make use of Point2D::equals().
```
## Exercise 7: Toggle State
```
Rating: 1/3 (Yellow)
Task:
1. You're given the following code snippet:
trait Toggle {
    fn toggle(&mut self);
}
struct LightSwitch {
    is_on: bool
}
2. Implement the Toggle trait for LightSwitch, which should change `is_on` from true to false and vice versa.
```
## Exercise 8: Item Count
```
Rating: 1/3 (Yellow)
Task:
1. You're given the following code snippet:
trait ItemCount {
    fn count(&self) -> usize;
}
struct Inventory {
    items: Vec<String>
}
2. Implement the ItemCount trait for Inventory, returning the count of items in the inventory.
```
## Exercise 9: Basic Arithmetics
```
Rating: 2/3 (Red)
Task:
1. You're given the following code snippet:
trait BasicArithmetic {
    fn add(&self, other: &Self) -> Self;
    fn subtract(&self, other: &Self) -> Self;
}
struct Number {
    value: i32
}
2. Implement the BasicArithmetic trait for the Number struct. Ensure the operations are performed correctly.
```
## Exercise 10: String Concatenation
```
Rating: 1/3 (Yellow)
Task:
1. You're given the following code snippet:
trait Concat {
    fn concat(&self, other: &Self) -> Self;
}
struct MyString {
    content: String
}
2. Implement the Concat trait for MyString, which should return a new MyString with the content of both strings concatenated.
```