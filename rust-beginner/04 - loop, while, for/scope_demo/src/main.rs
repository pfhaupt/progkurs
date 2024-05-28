fn main() { // Scope A
    let mut x: i32 = 0;
    { // Scope B
        let mut y: i32 = 1;
        while x < 100 { // Scope C
            println!("{}", x);
            let tmp: i32 = x;
            x = x + y;
            y = tmp;
            { // Scope D
                let x: i32 = 0;
                println!("{}", x);
            }
        }
    }
    // let z: i32 = y;
    println!("{}", x);
}
