use std::{
    io::{self, Write},
    path::Path,
    process::Command,
    env
};

fn main() {
    loop {
        print!(">> ");
        io::stdout().flush().unwrap();

        // get user input
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let input = input.trim();
                if input.is_empty() {
                    continue;
                }

                // parse command
                match parse_line(input) {
                    false => return,
                    _ => continue
                }
            }
            Err(e) => {
                eprintln!("Error reading input: {}", e);
                break;
            }
        }
    }
}

fn parse_line(input: &str) -> bool {
    let mut parts = input.split_whitespace();
    let command = parts.next().unwrap();
    let args: Vec<&str> = parts.collect();

    // handle built-in commands
    match command {
        "cd" => {
            // go to root if no args
            let new_dir = args.first().unwrap_or(&"/");
            let root = Path::new(new_dir);
            if let Err(e) = env::set_current_dir(root) {
                eprintln!("cd: {}", e);
            }
        }
        "exit" => {
            println!("Goodbye!");
            return false;
        }
        // now we handle external commands
        command => {
            let mut cmd = match Command::new(command).args(args).spawn() {
                Ok(cmd) => cmd,
                Err(e) => {
                    eprintln!("Command not found '{}': {}", command, e);
                    return true;
                }
            };
            
            if let Err(e) = cmd.wait() {
                eprintln!("Failed to spawn command: {}", e);
            }
        }
    }
    return true;
}
