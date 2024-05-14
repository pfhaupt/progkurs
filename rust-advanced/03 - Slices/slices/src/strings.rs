
use rand::Rng;
use rand::distributions::Uniform;

// https://stackoverflow.com/a/58437629
fn pretty_print_int(i: usize) -> String {
    let mut s = String::new();
    let i_str = i.to_string();
    let a = i_str.chars().rev().enumerate();
    for (idx, val) in a {
        if idx != 0 && idx % 3 == 0 {
            s.insert(0, '.');
        }
        s.insert(0, val);
    }
    s
}

fn not_efficient(input: &String) -> Option<(String, String)> {
    if input.is_empty() { None }
    else {
        let mut chars = input.chars().peekable();
        let mut buffer = String::new();
        while let Some(ch) = chars.peek() {
            if !ch.is_alphanumeric() { break; }
            buffer.push(chars.next().unwrap());
        }
        if buffer.is_empty() {
            return not_efficient(&chars.skip(1).collect())
        }
        Some((buffer, chars.collect()))
    }
}

fn semi_efficient(input: &str) -> Option<(&str, &str)> {
    if input.is_empty() { None }
    else {
        let mut rest = input;
        while rest.starts_with(char::is_alphanumeric) {
            rest = &rest[1..];
        }
        if rest.len() == input.len() {
            return semi_efficient(&input[1..])
        }
        let word = input.strip_suffix(rest).unwrap();
        Some((word, rest))
    }
}

fn very_efficient(input: &str) -> Option<(&str, &str)> {
    if input.is_empty() { None }
    else {
        let mut rest = input;
        while rest.starts_with(char::is_alphanumeric) {
            rest = &rest[1..];
        }
        if rest.len() == input.len() {
            return very_efficient(&input[1..])
        }
        let l = rest.len();
        let start = input.len() - l;
        let word = &input[..start];
        Some((word, rest))
    }
}

fn modify() {
    let mut original = "Hello World!".to_string();
    let slice = &original[..5];
    // original.push('!'); // Compiler error
    println!("slice = {}", slice);
}

fn showcase() {
    let original = "Hello, how are you?\nI am fine, thanks for asking!";
    // not_efficient() is the only method that does not panic here :^)
    // let original = "Hello! 😊"; Panic because UTF8
    let mut slice = original;
    while let Some((word, rest)) = very_efficient(slice) {
        print!("{:?} ", word);
        slice = rest;
    }
    println!("\nNo more words.");
}

fn bench() {
    const CUTOFF: usize = 5 * 1000 * 1000;
    let mut run_slow = true;
    let mut run_semi = true;
    let mut run_fast = true;
    let mut pow10 = 100;
    let mut length = 0;
    while run_slow || run_semi || run_fast {
        length += pow10;
        if length / pow10 == 10 {
            pow10 *= 10;
        }
        let now = std::time::Instant::now();
        let test_string: String = rand::thread_rng()
            .sample_iter(Uniform::new(char::from(32), char::from(126)))
            .take(length)
            .map(char::from)
            .collect();
        let string_time = now.elapsed().as_micros() as usize;
        println!("Creating string took {}µs", pretty_print_int(string_time));
        let mut slow_time = 0;
        let mut semi_time = 0;
        let mut fast_time = 0;
        let now = std::time::Instant::now();
        let mut fast_foo = test_string.as_str();
        debug_assert!(fast_foo.len() == test_string.len());
        if run_fast {
            while let Some((start, rest)) = very_efficient(fast_foo) {
                fast_foo = rest;
            }
            fast_time = now.elapsed().as_micros() as usize;
            run_fast = fast_time < CUTOFF;
        }
        if run_semi {
            let now = std::time::Instant::now();
            let mut semi_foo = test_string.as_str();
            debug_assert!(semi_foo.len() == test_string.len());
            while let Some((start, rest)) = semi_efficient(semi_foo) {
                semi_foo = rest;
            }
            semi_time = now.elapsed().as_micros() as usize;
            assert!(semi_foo == fast_foo);
            run_semi = semi_time < CUTOFF;
        }
        if run_slow {
            let now = std::time::Instant::now();
            let mut slow_foo = test_string.clone();
            debug_assert!(slow_foo.len() == test_string.len());
            while let Some((start, rest)) = not_efficient(&slow_foo) {
                slow_foo = rest;
            }
            slow_time = now.elapsed().as_micros() as usize;
            assert!(slow_foo == fast_foo);
            run_slow = slow_time < CUTOFF;
        }
        let slow_ratio = if fast_time == 0 { 0 } else { slow_time / fast_time };
        let semi_ratio = if fast_time == 0 { 0 } else { semi_time / fast_time };
        let length = test_string.len();
        let fast_text = format!("FAST: {}µs ({}ns per character)", pretty_print_int(fast_time), pretty_print_int(fast_time * 1000 / length));
        let semi_text = if run_semi {
            format!("SEMI: {}µs ({}x slower than FAST)", pretty_print_int(semi_time), pretty_print_int(semi_ratio))
        } else {
            format!("SEMI: >{}µs", pretty_print_int(CUTOFF))
        };
        let slow_text = if run_slow {
            format!("SLOW: {}µs ({}x slower than FAST)", pretty_print_int(slow_time), pretty_print_int(slow_ratio))
        } else {
            format!("SLOW: >{}µs", pretty_print_int(CUTOFF))
        };
        println!(
            "Length {}:\nFirst 40: {}\n{slow_text}\n{semi_text}\n{fast_text}\n",
            pretty_print_int(length),
            &test_string[0..40],
        );
    }
}

pub fn main() {
    showcase();
    bench();
}
