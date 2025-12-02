# Enums
## Usage
To get started, you'll need a working Rust environment. Open up your go-to editor and follow the instructions in each exercise. The difficulty ratings are there to give you an idea of what to expect, based on what you've learned.

## Goal
The goal of these exercises is to familiarize you with Rust's Enums and Pattern Matching. You'll learn to spot syntax errors, and learn how to use enums and match.

## Exercise 1: Dog Breeds
```
Rating: 1/3 (Yellow)
Task:
1. You're given the following code snippet:
enum Dog {
    Labrador,
    Retriever,
    Husky,
    Labrador
}
fn main() {
    let dog = Dog::Labrador;
}
2. Does this snippet compile? Why not?
```
## Exercise 2: Printing a Building
```
Rating: 1/3 (Yellow)
Task:
1. You're given the following code snippet:
enum Building {
    School,
    Market,
    Shop,
}
2. Using `match`, implement the `Debug` trait for the Building enum without using `derive`.
Hint:
a. The output should look like this:
- `This is a {enum value}!`
```
## Exercise 3: Buying a Building
```
Rating: 2/3 (Red)
Task:
1. You're given the following code snippet:
enum Building {
    School(u32),
    Market(u32),
    Shop(u32)
}
2. Modify the Debug implementation to also print the cost.
Hint:
a. The attached u32 represents the cost to build one school, market or shop.
b. The output should look like this:
- `This is a {enum value}! It costs {enum data} euro.`
c. Pattern Matching in Rust is very powerful, you can also bind variables using patterns:
let a = 5;
match a {
    b => {
        // You can use `b` in here, it has the same value as `a`.
    }
}
```
## Exercise 4: Village
```
Rating: 2/3 (Red)
1. You're given the following code snippet:
enum Building {
    School(u32),
    Market(u32),
    Shop(u32)
}
struct Village {
    buildings: Vec<Building>,
    total_cost: u32
}
2. Write a function that does the following:
- It takes in a mutable reference to a Village instance
- It sets its total cost to the sum of all Building costs in the Vector.
- It returns the total cost.
```
## Exercise 5: A more detailed School
```
Rating: 2/3 (Red)
Task:
1. You're given the following code snippet:
enum SchoolType {
    MiddleSchool,
    HighSchool,
    PrivateSchool,
}
struct School {
    cost: u32,
    rooms: u8,
    teachers: u8,
    typ: SchoolType,
}
enum Building {
    School(School),
    Market(u32),
    Shop(u32)
}
struct Village {
    buildings: Vec<Building>,
    total_cost: u32
}
2. Write a function that does the following:
- It takes in a reference to a Village instance
- It goes over every building, counts each middle school and adds their costs together
- It returns a String of the form `There are {count} middle school(s) in this Village, with a total cost of {price} euro.`
```
## Exercise 6: The Exclusive Library
```
Rating: 2/3 (Red)
Task:
1. You're given the following code snippet:
enum BookGenre {
    Fiction,
    NonFiction,
    Mystery,
    ScienceFiction,
}
struct Book {
    title: String,
    author: String,
    genre: BookGenre,
}
enum LibrarySection {
    General(Book),
    SpecialCollections(Book),
}
2. Write a function that does the following:
- It takes in a Vector of LibrarySection
- It counts the number of books in the SpecialCollections section
- It returns a String saying `There are {count} book(s) in the Special Collections.`
```
## Exercise 7: Traffic Light Controller
```
Rating: 1/3 (Yellow)
Task:
1. You're given the following code snippet:
enum TrafficLight {
    Red,
    Yellow,
    Green
}
2. Implement a function that simulates a simple traffic light controller:
- The function takes a TrafficLight enum and returns a String, telling you what to do:
    - `The light is Red. Stop.`
    - `The light is Yellow. Slow down.`
    - `The light is Green. Go ahead.`
```
## Exercise 8: Weather Patterns
```
Rating: 2/3 (Red)
Task:
1. You're given the following code snippet:
enum Weather {
    Sunny(u32), // Temperature
    Cloudy(u32), // Cloud Coverage Percentage
    Rainy(u32) // Rainfall in mm
}
2. Write a function that takes a Weather enum and prints a weather report:
- `It's Sunny today with a temperature of 25°C.`
- `It's Cloudy today with a cloud coverage of 70%.`
- `It's raining today, expected rainfall is 120mm.`
```
## Exercise 9: Pet Shop Animals
```
Rating: 2/3 (Red)
Task:
1. You're given the following code snippet:
enum Pet {
    Dog(String), // Breed
    Cat(String), // Breed
    Fish(String), // Species
}
struct PetShop {
    pets: Vec<Pet>,
}
2. Write a function that takes a PetShop instance and prints the available pets and their breeds/species.
```
## Exercise 10: Advanced Pattern Matching
```
Rating: 2/3 (Red)
Task:
1. You're given the following code snippet:
enum Respone {
    Success(i32),
    Warning(String, i32),
    Error(String),
}
2. Write a function that takes a vector of Response enums and categorizes them into successes, warnings and errors, returning a report.
- Example output: `5 successes, 3 warnings, and 2 errors.`
```