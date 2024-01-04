use std::env;
use std::error::Error;
use std::io::{self, Write};

pub fn git_input(command: &str) -> Result<String, Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    match command {
        "commit" => {
            if args.len() > 1 {
                Ok(args[1].clone())
            } else {
                println!("Enter commit message: ");
                get_user_input()
            }
        }
        "remote" => {
            if args.len() > 1 {
                Ok(args[1].clone())
            } else {
                println!("Enter remote name: ");
                get_user_input()
            }
        }
        "branch" => {
            if args.len() > 1 {
                Ok(args[1].clone())
            } else {
                println!("Enter branch name: ");
                get_user_input()
            }
        }
        _ => {
            println!("Invalid command");
            return Ok(String::from("Invalid command"));
        }
    }
}

fn get_user_input() -> Result<String, Box<dyn Error>> {
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}
