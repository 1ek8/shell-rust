#[allow(unused_imports)]
use std::io::{self, Write};
// use std::thread;
// use std::time::Duration;

fn main() {
    // TODO: Uncomment the code below to pass the first stage
    loop {
            print!("$ ");
        io::stdout().flush().unwrap();
        // thread::sleep(Duration::from_secs(3));

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let parts: Vec<&str> = input.trim().split_whitespace().collect();
        let command = parts[0];
        let args = &parts[1..];

        match command {
            "exit" => {
                break;
            },
            "echo" => {
                println!("{}", args.join(" "));
            },
            _ => {
                println!("{}: command not found", input.trim());
            }
        }
    }
}
