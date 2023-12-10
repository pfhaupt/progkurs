fn main() {
    let size: usize = 5;
    let mut vec: Vec<usize> = vec![0; size];
    let mut value: usize = 2;
    vec[value] = 3;
    value = vec[vec[value]];
    println!("value: {}", value);
    println!("vec: {:?}", vec);
}
