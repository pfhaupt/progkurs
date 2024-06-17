macro_rules! calculator {
    ($e:expr) => {
        println!("{} = {}", stringify!($e), $e);
    };
    () => {
        println!("Nothing was provided.");
    };
}
macro_rules! count_ident {
    () => { 0 };
    ($id:ident) => { 1 };
    ($id:ident, $($rest:ident),*) => {
        1 + count_ident!($($rest),*)
    };
}
fn main() {
    let count = count_ident!(a, b, c);
    println!("We have {count} identifiers!");
}
