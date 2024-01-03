use std::env::{self, args};
use std::error::Error;
use std::io::{self, Write};
use std::process::Command;

const GIT: &str = "git";

fn main() -> Result<(), Box<dyn Error>> {
    run_git_commands()?;
    Ok(())
}

fn run_git_commands() -> Result<(), Box<dyn Error>> {
    git_command(&["add", "-A"])?;
    git_command(&["commit", "-m", &message_input()?])?;
    git_command(&["push", &remote_input()?, &branch_input()?])?;

    println!("Successfully added, committed, and pushed all changes.");
    Ok(())
}

fn git_command(args: &[&str]) -> Result<(), Box<dyn Error>> {
    let status = Command::new(GIT).args(args).status()?;

    if !status.success() {
        Err(format!("Git command {:?} failed.", args).into())
    } else {
        Ok(())
    }
}

fn message_input() -> Result<String, Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        Ok(args[1].clone())
    } else {
        print!("Enter commit message: ");
        io::stdout().flush()?;

        let mut message = String::new();
        io::stdin().read_line(&mut message)?;
        Ok(message.trim().to_string())
    }
}

fn remote_input() -> Result<String, Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        Ok(args[1].clone())
    } else {
        print!("Enter remote name: ");
        io::stdout().flush()?;

        let mut remote = String::new();
        io::stdin().read_line(&mut remote)?;
        Ok(remote.trim().to_string())
    }
}

fn branch_input() -> Result<String, Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        Ok(args[1].clone())
    } else {
        print!("Enter branch name: ");
        io::stdout().flush()?;

        let mut branch = String::new();
        io::stdin().read_line(&mut branch)?;
        Ok(branch.trim().to_string())
    }
}

fn current_branch() -> Result<String, Box<dyn Error>> {
    let output = Command::new(GIT)
        .args(&["rev-parse", "--abbrev-ref", "HEAD"])
        .output()?;

    if !output.status.success() {
        Err("Failed to get current branch name.".into())
    } else {
        Ok(String::from_utf8(output.stdout)?.trim().to_string())
    }
}
