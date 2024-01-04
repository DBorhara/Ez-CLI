use std::error::Error;

use crate::git::add_commit_push::add_commit_push;

pub fn git_orchestrator(input: usize) -> Result<(), Box<dyn Error>> {
    println!("GIT ORCHESTRATOR INPUT: {}", input);
    match input {
        1 => add_commit_push(),
        _ => Err("Invalid input provided. Expected a specific command identifier.".into()),
    }
}
