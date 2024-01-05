use std::{env, error::Error, process::Command};

const NVIM: &str = "nvim";
pub fn open_tmux_config() -> Result<(), Box<dyn Error>> {
    let home_dir = env::var("HOME")?;

    let tmux_path = format!("{}/.config/tmux/tmux.conf", home_dir);
    let tmux_command = Command::new(NVIM).arg(&tmux_path).status()?;

    if !tmux_command.success() {
        Err(format!("Error opening your zsh file.").into())
    } else {
        Ok(())
    }
}
