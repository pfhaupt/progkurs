macro_rules! gen {
    ($($m:tt, $ex:ident),*) => {
        $(
            mod $m;
            #[allow(unused)]
            use $m::main as $ex;
        )*
        fn main() {
            $(
                println!("{}", "-".repeat(20));
                println!("{}", stringify!($ex));
                $ex();
                println!("{}", "-".repeat(20));
            )*
        }
    }
}

gen!(
    ex1, ex1main,
    ex2, ex2main,
    ex3, ex3main,
    ex4, ex4main,
    c, cmain
);