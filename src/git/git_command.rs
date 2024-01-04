use std::error::Error;
use std::process::Command;

const GIT: &str = "git";

pub fn git_command(args: &[&str]) -> Result<(), Box<dyn Error>> {
    let git_command = Command::new(GIT).args(args).status()?;

    if !git_command.success() {
        Err(format!("Git command {:?} failed to execute.", args).into())
    } else {
        Ok(())
    }
}
