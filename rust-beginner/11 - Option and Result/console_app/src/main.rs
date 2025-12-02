use std::collections::HashMap;
use std::io::{self, prelude::*};

fn help(_args: &[&str]) -> Result<(), String> {
    println!("Available commands:");
    println!("  help");
    println!("  exit");
    println!("  echo");
    println!("  read");
    Ok(())
}

fn exit(_args: &[&str]) -> Result<(), String> {
    std::process::exit(0);
}

fn echo(args: &[&str]) -> Result<(), String> {
    for arg in args.iter() {
        print!("{} ", arg);
    }
    println!();
    Ok(())
}

fn read(args: &[&str]) -> Result<(), String> {
    if args.len() != 1 {
        return Err("read requires 1 argument".to_string());
    }
    let filename = args[0];
    let mut file = std::fs::File::open(filename).or_else(|e| Err(e.to_string()))?;
    let mut contents = String::new();
    let _ = file.read_to_string(&mut contents).or_else(|e| Err(e.to_string()))?;
    println!("{}", contents);
    Ok(())
}
fn main() {
    let mut commands: HashMap<&str, fn(&[&str]) -> Result<(), String>> = HashMap::new();
    commands.insert("help", help);
    commands.insert("exit", exit);
    commands.insert("echo", echo);
    commands.insert("read", read);
    const COMMANDS: [&str; 4] = [
        "help",
        "exit",
        "echo",
        "read",
    ];
    loop {
        print!("> ");
        // print! does not flush stdout, so we do it manually
        io::stdout().flush().ok().expect("Could not flush stdout");
        let mut line = String::new();
        let line_read = io::stdin().read_line(&mut line);
        match line_read {
            Ok(_) => {
                let line = line.trim_end(); // trim '\n'
                if line.len() == 0 { continue; }
                let words = line.split_whitespace().collect::<Vec<&str>>();
                let cmd = words[0];
                let args = &words[1..];
                if let Some(command) = commands.get(cmd) {
                    let result = command(args);
                    if let Err(error) = result {
                        println!("error: {}", error);
                    }
                } else {
                    println!("Unknown command: {}", cmd);
                    println!("Available commands:");
                    for cmd in COMMANDS.iter() {
                        println!("  {}", cmd);
                    }
                }
            }
            Err(error) => println!("error: {}", error),
        }
    }
}
