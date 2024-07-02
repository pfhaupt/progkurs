#![allow(unused)]

use rayon::iter::{IntoParallelIterator, ParallelIterator};
fn sum_of_all_numbers() {
    let now = std::time::Instant::now();
    let sum: i128 = (1..100_000_000)
        .into_iter().filter(|n| n % 2 == 1).map(|n|n * n).sum();
    println!("{}", sum);
    println!("Normal: {}", now.elapsed().as_secs_f64());
    let now = std::time::Instant::now();
    let sum: i128 = (1..100_000_000)
        .into_par_iter().filter(|n| n % 2 == 1).map(|n|n * n).sum();
    println!("{}", sum);
    println!("Parallel: {}", now.elapsed().as_secs_f64());
}

fn find_value() {
    let collection = [Container(1), Container(2), Container(3)];
    let element = collection.iter().find(|c| c.0 == 3);
    println!("Found container with a number of 3: {:?}", element);
}
fn is_prime(num: &u128) -> bool {
    if *num == 2 { return true; }
    else if num % 2 == 0 { return false; }
    (1..).map(|n|2*n+1).take_while(|n|n*n<=*num).all(|n|num % n != 0)
}
fn calculate_primes() {
    let all_numbers = 2..;
    all_numbers.filter(is_prime).for_each(|e|println!("{e}"));
}
#[derive(Debug)]
struct Container(i32);
impl TryFrom<i32> for Container {
    type Error = String;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value < 10 { Err(format!("Numbers below 10 are reserved, got {}", value)) }
        else { Ok(Self(value)) }
    }
}
fn main() -> Result<(), String> {
    let containers = (15..20)
        .inspect(|n|println!("Number: {}", n))
        .map(|i|Container::try_from(i))
        .inspect(|v|println!("Result: {:?}", v))
        .collect::<Result<Vec<_>, _>>()?;
    for c in &containers {
        println!("{:?}", c);
    }
    Ok(())
}
