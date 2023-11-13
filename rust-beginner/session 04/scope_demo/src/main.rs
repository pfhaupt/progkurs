fn main() {
    let mut x: i32 = 0;
    {
        let mut y: i32 = 1;
        while x < 100 {
            println!("{}", x);
            let tmp: i32 = x;
            x = x + y;
            y = tmp;
            {
                let x: i32 = 0;
                println!("{}", x);
            }
        }
    }
    // let z: i32 = y;
    println!("{}", x);
}
