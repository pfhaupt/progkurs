fn main() {
    for n in 0..10 {
        if n < 5 {
            continue;
        }
        if n == 7 {
            break;
        }
        println!("n: {}", n);
    }
}
