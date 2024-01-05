use std::{error::Error, process::Command};

const NVIM: &str = "nvim";

pub fn open_zsh_config() -> Result<(), Box<dyn Error>> {
    // Not opening zsh file correctly
    // TODO: Figure out why nvim isn't opening it
    let zsh_command = Command::new(NVIM).arg("~/.zshrc").status()?;

    if !zsh_command.success() {
        Err(format!("Error opening your zsh config.").into())
    } else {
        Ok(())
    }
}
