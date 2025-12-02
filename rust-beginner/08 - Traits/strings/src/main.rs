#![allow(unused)]
fn main() {
    let str = "Hello, world!";
    let string = String::from("Hello, world!");
    let hello = &string[0..5];
    let emote_char = '👋';
    let emote = "👋";
    // Below will panic at runtime, because index 1 is not a valid byte boundary
    // let emote_slice = &emote[0..1];
    let emote_string = String::from("👋");
    let hello_in_greek = String::from("Γεια σου κόσμε!");
    let hello_in_japanese = String::from("こんにちは世界!");
    println!("str: {}", str);
    println!("string: {}", string);
    println!("hello: {}", hello);
    println!("emote_char: {}", emote_char);
    println!("emote: {}", emote);
    println!("emote_string: {}", emote_string);
    println!("hello_in_greek: {}", hello_in_greek);
    println!("hello_in_japanese: {}", hello_in_japanese);
}
