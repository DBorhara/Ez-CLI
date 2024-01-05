use std::{env, error::Error, process::Command};

const NVIM: &str = "nvim";

pub fn open_zsh_config() -> Result<(), Box<dyn Error>> {
    let home_dir = env::var("HOME")?;

    let zshrc_path = format!("{}/.zshrc", home_dir);
    let zsh_command = Command::new(NVIM).arg(&zshrc_path).status()?;

    if !zsh_command.success() {
        Err(format!("Error opening your zsh config.").into())
    } else {
        Ok(())
    }
}
