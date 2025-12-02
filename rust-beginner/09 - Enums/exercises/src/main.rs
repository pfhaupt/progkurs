mod exercise_1;
use exercise_1::main as exercise_1_main;
mod exercise_2;
use exercise_2::main as exercise_2_main;
// Exercise 3 is commented out because it doesn't compile.
// mod exercise_3;
// use exercise_3::main as exercise_3_main;

fn main() {
    exercise_1_main();
    exercise_2_main();
    // exercise_3_main();
}
