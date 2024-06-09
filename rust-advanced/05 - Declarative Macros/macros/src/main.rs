#![allow(unused)]

use std::collections::HashMap;

macro_rules! showcase {
    [] => {
        println!("Passed nothing!");
    };
    ($e:expr) => {
        println!("Passed expr `{}`", stringify!($e));
    };
}

macro_rules! rules {
    ($e:expr) => { "Expression."; };
    ($i:ident) => { "Identifier."; };
    ($t:ty) => { "Type."; };
    ($($val:expr),*) => {
        "Comma-separated sequence of expressions.";
        "The sequence can be empty.";
    };
    ($($id:ident)+) => {
        "Sequence of identifiers, with no separators.";
        "There must be at least one identifier. (+)";
    };
    ($($($t:ty),*);+) => {
        "Semicolon-separated sequence of comma-separated sequence of types.";
        "Sequence of types can be empty. (*)";
        "Sequence of sequences must have at least one element. (+)";
    };
    () => {}; // None
}
macro_rules! decl {
    ($i:ident $e:expr) => {
        let $i = b;
    };
}
fn other() {
    // decl!(a 0);
}
type TCResult = Result<(), ()>;
struct Block;
struct Variable;
struct Function {
    params: Vec<Variable>,
    body: Block,
}
struct Method {
    params: Vec<Variable>,
    body: Block,
}
macro_rules! shared_func {
    ($f:ident) => {
        {
            let mut res = Ok(());
            for p in &$f.params {
                res = param_type_check(&p);
                if res.is_err() { break; }
            }
            if res.is_err() { res }
            else { block_type_check(&$f.body) }
        }
    };
}
fn method_type_check(method: &Method) -> TCResult {
    shared_func!(method)?;
    // Other method related checks
    Ok(())
}
fn func_type_check(func: &Function) -> TCResult {
    shared_func!(func);
    // Other function related checks
    Ok(())
}
fn block_type_check(block: &Block) -> TCResult { Ok(()) }
fn param_type_check(param: &Variable) -> TCResult { Ok(()) }
fn main() {
}
macro_rules! calc {
    ($e:expr) => {
        println!("{} = {}", stringify!($e), $e);
    };
}
// fn main() {
//     calc!(5 + 4);
//     calc!(7 * 5 - 3);
// }

fn rules() {
    rules!(5 + 10);
    rules!(a);
    rules!(5, 4, 3);
    rules!(a b);
    rules!(i32);
    let i32: u8 = 5;
    let u8: i16 = 12;
    let str: u32 = 1;
    println!("{} {} {}", i32, u8, &str);
    rules!(i32, u8, &str);
    rules!(i32, u8, &str; String, u8);
}
