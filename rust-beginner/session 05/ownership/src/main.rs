fn main() {
    let a: i32 = 0;
    {
        let b: i32 = 1;
        if b == 1 {
            let v: Vec<i32> = vec![1];
            println!("{:?}", v);
        }
    }
    println!("{}", a);
}
