use std:: {
    io::{stdin, stdout, Write},
    path::Path,
    process::Command,
    env
};

fn main() {
    loop {
        // get user input
        print!(">> ");
        stdout().flush().unwrap();
        let mut input = String::new();
        stdin().read_line(&mut input)
            .expect("failed to read line");

        // parse
        if input.is_empty() {
            continue;
        }
        match parse_line(&input) {
            false => return,
            true => continue
        }
    }
}

fn parse_line(input: &str) -> bool {
    let mut args = input.trim().split_whitespace();
    let command = args.next().unwrap();
    let args: Vec<&str> = args.collect();

    // execute commands
    match command {
        "cd" => {
            let new_dir = args.first().unwrap_or(&"/");
            let root = Path::new(new_dir);
            if let Err(e) = env::set_current_dir(root) {
                eprintln!("error changing directory: {}", e);
            }
        },
        "exit" => {
            println!("🦀🦀🦀 goodbye 🦀🦀🦀");
            return false;
        },

        // external commands
        command => {
            let mut child = match Command::new(command).args(args).spawn() {
                Ok(c) => c,
                Err(e) => {
                    eprintln!("command '{}' not found: {}", command, e);
                    return true;
                }
            };

            if let Err(e) = child.wait() {
                eprintln!("command terminated with error code: {}", e);
            }
        }
    }
    return true;
}
