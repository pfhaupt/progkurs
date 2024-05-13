fn main() {
    let size: usize = 5;
    let mut vec: Vec<usize> = vec![42, 31, 1, 19, 3];
    let mut value: usize = vec[size - 1];
    vec[value] = 2;
    value = vec[vec[value]];
    vec[0] = vec[value];
    println!("value: {}", value);
    println!("vec: {:?}", vec);
}
