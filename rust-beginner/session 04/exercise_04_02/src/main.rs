// Spoiler: We never write anything to vector, it does not need to be mutable
#![allow(unused_mut)]

// Spoiler: We're assigning to `elem`, but never do anything with it
//          Disabling this warning makes it look like assigning to `elem` also updates `vector`
//          which it does not.
#![allow(unused_variables)]
#![allow(unused_assignments)]

// If you read this you're cool.

fn main() {
    let mut vector: Vec<i32> = vec![1, 2, 3, 4];
    for index in 0..vector.len() {
        let mut elem = vector[index];
        elem *= 2;
    }
    println!("{:?}", vector);
    for prime in [2, 3, 5, 7] {
        if vector.contains(&prime) {
            println!("Vector contains prime {}", prime);
        }
    }
}
