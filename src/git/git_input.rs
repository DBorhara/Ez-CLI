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
                get_user_input(Some("Auto Commit: Regular backup/checkpoint."))
            }
        }
        "remote" => {
            if args.len() > 1 {
                Ok(args[1].clone())
            } else {
                println!("Enter remote name: ");
                get_user_input(Some("origin"))
            }
        }
        "branch" => {
            if args.len() > 1 {
                Ok(args[1].clone())
            } else {
                println!("Enter branch name: ");
                get_user_input(Some("main"))
            }
        }
        _ => {
            println!("Invalid command");
            return Ok(String::from("Invalid command"));
        }
    }
}

fn get_user_input(default: Option<&str>) -> Result<String, Box<dyn Error>> {
    if let Some(default_value) = default {
        println!(
            "Press Enter for default value ('{}'), or enter new value:",
            default_value
        );
    } else {
        println!("Enter value:");
    }

    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    if input.trim().is_empty() {
        Ok(default.unwrap_or("").to_string())
    } else {
        Ok(input.trim().to_string())
    }
}
