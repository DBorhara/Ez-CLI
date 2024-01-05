use std::{error::Error, process::Command};

const NVIM: &str = "nvim";
pub fn open_tmux_config() -> Result<(), Box<dyn Error>> {
    let tmux_command = Command::new(NVIM).arg("~/.dotfiles/tmux").status()?;

    if !tmux_command.success() {
        Err(format!("Error opening your tmux config.").into())
    } else {
        Ok(())
    }
}
