use anyhow::Result;
use std::env;
use std::fs;
use std::path::PathBuf;

/// Embedded zsh completion script (custom hand-rolled).
const ZSH_COMPLETION_SCRIPT: &str = include_str!("_monomana");

/// Ensures the zsh completion script is installed a-nd up-to-date.
/// NOTE: Under oh-my-zsh environment, the script must be created at $ZSH/completions
/// Otherwise, the script is created at $HOME/.zsh/completions by default.
pub fn ensure_zsh_completion_installed() -> Result<()> {
    // A. Figure out target directory
    let target_dir: PathBuf = if let Ok(zsh_root) = env::var("ZSH") {
        // oh-my-zsh environment detected
        PathBuf::from(zsh_root).join("completions")
    } else {
        // vanilla zsh – default to $HOME/.zsh/completions
        let home = env::var("HOME")?;
        PathBuf::from(home).join(".zsh").join("completions")
    };

    // B. Ensure directory exists
    fs::create_dir_all(&target_dir)?;

    // C. Write (or overwrite) the completion file 
    let target_path = target_dir.join("_monomana");
    fs::write(&target_path, ZSH_COMPLETION_SCRIPT)?;

    Ok(())
} 