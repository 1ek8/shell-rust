#[allow(unused_imports)]
use std::io::{self, Write};
// use std::thread;
// use std::time::Duration;

fn main() {
    // TODO: Uncomment the code below to pass the first stage
    print!("$ ");
    io::stdout().flush().unwrap();
    // thread::sleep(Duration::from_secs(3));

    let mut command = String::new();
    io::stdin().read_line(&mut command).unwrap();
    println!("{}: command not found", command.trim());
}
