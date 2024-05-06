pub fn main() {
    #[cfg(not(feature="spam"))]
    normal::main();
    #[cfg(feature="spam")]
    spam_stuff::main();
}

#[cfg(not(feature="spam"))]
#[allow(unused)]
mod normal {
    use std::{fs, path::Path};

    impl Flags {
        fn parse_arg(args: std::env::Args) -> Self {
            // Usage: cargo run -- [args]
            // Example: cargo run -- --do_stuff
            let arg_vec: Vec<_> = args.collect();
            let do_stuff = arg_vec.iter().any(|e|e == "--do_stuff");
            let something = arg_vec.iter().any(|e|e == "--something");
            Self {
                do_stuff,
                something
            }
        }
    }
    #[derive(Debug)]
    struct Flags {
        do_stuff: bool,
        something: bool,
    }
    struct Parser<'f, 's> {
        flags: &'f Flags,
        original: &'s str,
    }
    impl<'f, 's> Parser<'f, 's> {
        fn new(flags: &'f Flags, original: &'s str) -> Self {
            Self { flags, original }
        }
        fn next(&mut self) -> Option<&str> {
            let tkn = self.original.split_whitespace().next()?;
            self.original = &self.original[(tkn.len()+1)..];
            Some(tkn)
        }
    }
    pub fn main() {
        let args = std::env::args();
        let flags = Flags::parse_arg(args);
        let content = match fs::read_to_string("./foo.txt") {
            Ok(content) => content,
            Err(e) => panic!("{}", e)
        };
        let mut parser = Parser::new(&flags, &content);
        while let Some(tkn) = parser.next() {
            println!("{}", tkn);
        }
    }
}

#[cfg(feature="spam")]
mod spam_stuff {
    /*
    Usage: cargo run --features=spam
    The idea is to spam as many flags and structs in a short time as possible
    and to then look how long it takes to clone the flag structs vs borrowing them
    Spoiler: Borrowing is faster :^)

    Yes, I heavily abuse procedural macros for that.
    I would NOT want to handcraft 10000 structs :^)

    Modify carefully :)
    -> false, 10m loops, 5 structs, 100 flags -> 20sec
    -> true, 10m loops, 5 structs, 100 flags -> 0.27sec (75 times faster!)

    100 flags is actually not that much for big projects.
    They're rarely used all together, but still need to be stored somewhere.
     */
    spam::use_ref!(true);
    spam::loop_count!(10000000);
    spam::struct_count!(5);
    spam::gen_flags!(100);
    spam::gen_structs!();

    pub fn main() {
        let now = std::time::Instant::now();
        // Note: We're not actually parsing anything here
        let args = std::env::args();
        let flags = Flags::parse(&args);
        spam::setup_structs!();
        println!("{}ms", (std::time::Instant::now() - now).as_millis());
    }
}
