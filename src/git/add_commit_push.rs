use std::error::Error;

use super::git_input::git_input;
use crate::git_command::git_command;

pub fn add_commit_push() -> Result<(), Box<dyn Error>> {
    git_command(&["add", "-A"])?;
    let commit_message = git_input("commit")?;
    git_command(&["commit", "-m", &commit_message])?;
    let remote = git_input("remote")?;
    let branch = git_input("branch")?;
    git_command(&["push", &remote, &branch])?;

    Ok(())
}
