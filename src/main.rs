#[allow(unused_imports)]
use std::io::{self, Write};
use std::env;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;
use std::process::Command;
// use std::thread;
// use std::time::Duration;

const BUILT_INS: [&str;5] = ["exit", "echo", "type", "pwd", "cd"];

fn main() {
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
            "pwd" => {
                match env::current_dir() {
                    Ok(path) => println!("{}", path.display()),
                    Err(e) => eprintln!("Error retrieving current directory: {}", e)
                }
            },
            "cd" => {
                if let Some(path) = args.get(0) {
                    if std::env::set_current_dir(path).is_err() {
                        println!("cd: {}: No such file or directory", path);
                    }
                }
            },
            _ => {
                // println!("Other:");
                let output = Command::new(command).
                                                        args(args)
                                                        .status(); 
                if output.is_err(){
                    println!("{}: command not found", input.trim()) 
                }
            }
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