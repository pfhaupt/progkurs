// Config to stop rust-analyzer from warning me about the code after the first loop
#![allow(unreachable_code)]

// Config to stop rust-analyzer from warning me about the functions I never called
#![allow(unused)]

fn main() {
    loop_loop(); // will freeze program
    while_loop(); // will also freeze
    for_loop();
}

fn loop_loop() {
    loop {
        println!("Hehe, this will print forever!!!");
    }
    println!("Loops are fun");

    let mut number: i8 = 0;
    loop {
        println!("Weeeee!!!");
        number = number + 1;
    }
}

fn while_loop() {
    let mut number: i8 = 0;
    while number < 10 {
        println!("Number: {}", number);
        number += 1;
    }
    println!("The final number is: {}", number);

    let condition: bool = true;
    while condition {
        println!("Do as long as condition is true!");
        // extra code...
    }
}

fn for_loop() {
    let array: [i32; 5] = [1, 2, 3, 4, 5];
    for element in array {
        println!("Current: {}", element);
    }

    let vector: Vec<i32> = vec![120, 768, 9021, -4012];
    for blub in vector { // Note: This moves the vector!
        println!("Current: {}", blub);
    }
    
    for n in 0..10 {
        println!("Number: {}", n);
    }
    for m in 0..=10 {
        println!("Mumber: {}", m);
    }
}

fn convert_to_loop() {
    let vector: Vec<i32> = vec![120, 768, 9021, -4012];
    for element in vector { // Note: This moves the vector!
        println!("Current: {}", element);
    }

    let vector: Vec<i32> = vec![120, 768, 9021, -4012];
    for index in 0..vector.len() {
        let element = vector[index];
        println!("Current: {}", element);
    }

    let mut index: usize = 0;
    while index < vector.len() {
        let element = vector[index];
        println!("Current: {}", element);
        index += 1;
    }

    let mut index: usize = 0;
    loop {
        if index >= vector.len() {
            break;
        }
        let element = vector[index];
        println!("Current: {}", element);
        index += 1;
    }
}