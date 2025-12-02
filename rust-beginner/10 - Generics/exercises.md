# Generics
## Usage
To get started, you'll need a working Rust environment. Open up your go-to editor and follow the instructions in each exercise. The difficulty ratings are there to give you an idea of what to expect, based on what you've learned.

## Goal
The goal of these exercises is to familiarize you with Rust's Generics. You'll learn to spot syntax errors, and learn how to use Generics.

## Exercise 1: Generic Pair
```
Rating: 1/3 (Yellow)
Task:
- Define a generic struct `Pair<T, U>` that holds two values of any types T and U.
- Implement a method `new` to create a new Pair.
```

## Exercise 2: Generic Function Swap
```
Rating: 1/3 (Yellow)
Task:
- Write a generic function `swap<T, U>` that takes a tuple of two elements (T, U) and returns a tuple with its elements swapped (U, T).
```

## Exercise 3: Generic Accumulator
```
Rating: 2/3 (Red)
Task:
- Create a generic struct `Accumulator<T>` that holds a value of any numeric type T.
- Implement methods `add` and `subtract` to modify the value.
```

## Exercise 4: Generic Stack
```
Rating: 2/3 (Red)
Task:
- Implement a generic stack `Stack<T>` with operations `push`, `pop`, and `peek`.
```

## Exercise 5: Generic KeyValue Store
```
Rating: 3/3 (Purple)
Task:
- Create a generic `KeyValue<K, V>` struct to store a key of type K and a value of type V.
- Implement a method `get_value` that returns a reference to the value.
```

## Exercise 6: Generic Traits for Calculation
```
Rating: 3/3 (Purple)
Task:
- Define a generic trait `Calculator<T>` with a method `calculate` that accepts and returns the type T.
- Implement this trait for a struct `BasicCalculator` which performs a basic arithmetic operation.
```

## Exercise 7: Generic Largest Function
```
Rating: 2/3 (Red)
Task:
- Write a function `largest<T>` which takes a generic slice &[T] and returns the largest item.
- Add trait bounds to ensure it works only for types implementing `PartialOrd` and `Copy`.
```

## Exercise 8: Multiple Generic Types
```
Rating: 2/3 (Red)
Task:
- Create a generic struct `TwoValues<T, U>` that holds two values of types T and U.
- Implement a method `mix` that swaps the types and returns `TwoValues<U, T>`.
```

## Exercise 9: Generic Enum Option
```
Rating: 1/3 (Yellow)
Task:
- Define a generic enum `Option<T>` with two variants: `Some(T)` and `None`.
- Implement a method `is_some` that returns true if it is Some variant, false otherwise.
```

## Exercise 10: Generic Iterator Countdown
```
Rating: 3/3 (Purple)
Task:
- Implement a generic iterator `CountDown<T>` that iterates from a specified number down to 1.
- The iterator should work for any type T that implements the `Sub` and `From<u8>` traits.
```
