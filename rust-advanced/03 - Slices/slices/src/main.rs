#![allow(unused)]

mod strings;
use strings::main as str_main;
mod arrays;
use arrays::main as arr_main;

fn main() {
    arr_main();
    str_main();
}
