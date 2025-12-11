use std::io::{self, Write};

fn main() {
    loop {
        print!(">> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let parts: Vec<String> = input.trim().split_whitespace().map(|s| s.to_string()).collect();
        exec(parts);
    }
}

fn exec(parts: Vec<String>) {
    if parts.is_empty() {
        println!("oh no! no command :(");
        return;
    }

    let cmd = &parts[0];
    let args = &parts[1..];

    println!("Command: {}", cmd);
    for arg in args {
        println!("Arg: {}", arg);
    }
}