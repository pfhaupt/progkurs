fn main() {
    let mut vector: Vec<i32> = vec![1, 2];
    for i in 3..=10 {
        vector.push(i);
    }
    for elem in &mut vector {
        *elem *= 2;
    }
    for elem in &mut vector {
        *elem += 1;
    }
    println!("{}", vector.contains(&3));
    println!("{}", vector.contains(&11));
    println!("{}", vector.contains(&14));
    println!("{}", vector.contains(&20));
    println!("{:?}", vector);
}
