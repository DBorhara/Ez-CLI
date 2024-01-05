use std::{error::Error, process::Command};

const NVIM: &str = "nvim";
pub fn open_nvim_config() -> Result<(), Box<dyn Error>> {
    let nvim_command = Command::new(NVIM).arg("~/.dotfiles/nvim").status()?;

    if !nvim_command.success() {
        Err(format!("Error opening your nvim config.").into())
    } else {
        Ok(())
    }
}
