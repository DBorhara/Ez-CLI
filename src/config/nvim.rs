use std::{env, error::Error, process::Command};

const NVIM: &str = "nvim";
pub fn open_nvim_config() -> Result<(), Box<dyn Error>> {
    let home_dir = env::var("HOME")?;

    let nvim_path = format!("{}/.config/nvim", home_dir);
    let nvim_command = Command::new(NVIM).arg(&nvim_path).status()?;

    if !nvim_command.success() {
        Err(format!("Error opening your nvim config.").into())
    } else {
        Ok(())
    }
}
