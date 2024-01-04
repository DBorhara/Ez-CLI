use std::error::Error;
mod git;
use git::add_commit_push::add_commit_push;

use crate::git::git_command;

fn main() -> Result<(), Box<dyn Error>> {
    add_commit_push()?;
    Ok(())
}
