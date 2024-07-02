mod good;
mod simple;
use std::env::args;

use good::main as good_main;
use simple::main as simple_main;

fn main() {
    let run_simple = args().any(|a|a == "simple");
    if run_simple {
        simple_main();
    } else {
        good_main();
    }
}