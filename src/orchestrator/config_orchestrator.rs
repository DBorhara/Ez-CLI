use crate::config::nvim::open_nvim_config;
use crate::config::tmux::open_tmux_config;
use crate::config::zsh::open_zsh_config;
use std::error::Error;
pub fn config_orchestrator(input: usize) -> Result<(), Box<dyn Error>> {
    match input {
        1 => open_nvim_config(),
        2 => open_tmux_config(),
        3 => open_zsh_config(),
        _ => Err("invalid input provided. expected a specific command identifier.".into()),
    }
}
