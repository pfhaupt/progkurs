#![allow(unused)]
// We'll use this file to run our exercises
// Ignore the mod keyword for now, we'll cover it later
// For now, just know that we're importing the contents of the exercise_1.rs file
// into this scope
mod exercise_1;
mod exercise_2;
mod exercise_3;
use exercise_1::main as exercise_1_main;
use exercise_2::main as exercise_2_main;
use exercise_3::main as exercise_3_main;

fn main() {
    exercise_1_main();
    exercise_2_main();
    exercise_3_main();
    println!("Hello, world!");
}
