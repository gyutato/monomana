use std::{env, fs, path::PathBuf};

fn main() {
    // Embedded script content (same as src/_monomana)
    const SCRIPT: &str = include_str!("src/_monomana");

    // Determine completion directory (Oh-My-Zsh vs vanilla zsh)
    let target_dir: Option<PathBuf> = env::var("ZSH")
        .map(PathBuf::from)
        .map(|p| p.join("completions"))
        .ok()
        .or_else(|| {
            env::var("HOME").ok().map(|home| PathBuf::from(home).join(".zsh/completions"))
        });

    let Some(dir) = target_dir else {
        eprintln!("[monomana build.rs] Could not determine target directory for zsh completion script (ZSH or HOME missing)");
        return;
    };

    if let Err(e) = fs::create_dir_all(&dir) {
        eprintln!("[monomana build.rs] Failed to create dir {dir:?}: {e}");
        return;
    }

    let path = dir.join("_monomana");
    if let Err(e) = fs::write(&path, SCRIPT) {
        eprintln!("[monomana build.rs] Failed to write completion script to {path:?}: {e}");
    }
} 