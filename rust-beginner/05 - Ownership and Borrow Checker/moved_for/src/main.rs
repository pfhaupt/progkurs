fn main() {
    let vec: Vec<i32> = vec![1; 100_000_000];
    for elem in vec.clone() {
        println!("We're doing something with {}", elem);
    }
    println!("Now we can't use vec anymore! {:?}", vec);
}
