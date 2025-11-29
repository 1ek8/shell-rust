#[allow(unused_imports)]
use std::io::{self, Write};
use std::env;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;
// use std::thread;
// use std::time::Duration;

const BUILT_INS: [&str;3] = ["exit", "echo", "type"];

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
            //slices are view types, they implement copy trait, ownership isnt moved by passing them, dont need to reference in fn calls
            "type" => cmd_type_handler(&args), 
            "exit" => break,
            "echo" => println!("{}", &args.join(" ")),
            _ => println!("{}: command not found", input.trim())
        }
    }
}

fn cmd_type_handler(args: &[&str]){
    for type_command in args {
        if BUILT_INS.contains(&type_command) {
            println!("{} is a shell builtin", type_command)
        } else if let Some(path) = find_exec(&type_command) {
            println!("{} is {}", type_command, path.display());
        } else {
            println!("{}: not found", type_command);
        }
    }
}

fn find_exec(command: &str) -> Option<PathBuf> {
    let path_var = env::var("PATH").ok()?;

    for path_dir in env::split_paths(&path_var) {
        let full_path = path_dir.join(command);

        if full_path.is_file() {
            if let Ok(metadata) = full_path.metadata() {
                if metadata.permissions().mode() & 0o111 != 0 { // at least someone - user, group or others must have exec permission for file
                    return Some(full_path);
                }
            }
        }
    }
    None
}