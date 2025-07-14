use std::path::Path;

use anyhow::{bail, Result};

/// Supported package managers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Manager {
    Pnpm,
    Yarn,
}

impl std::fmt::Display for Manager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Manager::Pnpm => "pnpm",
            Manager::Yarn => "yarn",
        };
        write!(f, "{}", s)
    }
}

/// Attempt to detect the package manager by searching for lock files starting
/// from `start_dir` and walking up the directory tree.
///
/// - `pnpm-lock.yaml` → Manager::Pnpm
/// - `yarn.lock`     → Manager::Yarn
///
/// If no lock file is found, an error is returned.
///
/// NOTE: This is the initial minimal implementation – it stops at the first
/// match and does not yet handle tie-breaking when both files exist in the
/// same directory. That logic will be refined later.
pub fn detect_manager(start_dir: impl AsRef<Path>) -> Result<Manager> {
    use log::debug;

    for dir in start_dir.as_ref().ancestors() {
        debug!("Searching for lock files in {}", dir.display());

        let pnpm_lock = dir.join("pnpm-lock.yaml");
        let yarn_lock = dir.join("yarn.lock");

        let pnpm_exists = pnpm_lock.exists();
        let yarn_exists = yarn_lock.exists();

        if pnpm_exists || yarn_exists {
            // Tie-break priority: pnpm > yarn when both exist in same directory.
            return Ok(if pnpm_exists { Manager::Pnpm } else { Manager::Yarn });
        }

        // Stop traversing once we hit a git root (directory containing .git).
        if dir.join(".git").exists() {
            break;
        }
    }

    bail!("No pnpm-lock.yaml or yarn.lock found up to git root");
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn detects_pnpm() {
        let dir = tempdir().unwrap();
        fs::write(dir.path().join("pnpm-lock.yaml"), "mock").unwrap();
        let manager = detect_manager(dir.path()).unwrap();
        assert_eq!(manager, Manager::Pnpm);
    }

    #[test]
    fn detects_yarn() {
        let dir = tempdir().unwrap();
        fs::write(dir.path().join("yarn.lock"), "mock").unwrap();
        let manager = detect_manager(dir.path()).unwrap();
        assert_eq!(manager, Manager::Yarn);
    }

    #[test]
    fn tie_break_prefers_pnpm_over_yarn() {
        let dir = tempdir().unwrap();
        fs::write(dir.path().join("pnpm-lock.yaml"), "mock").unwrap();
        fs::write(dir.path().join("yarn.lock"), "mock").unwrap();
        let manager = detect_manager(dir.path()).unwrap();
        assert_eq!(manager, Manager::Pnpm);
    }

    #[test]
    fn stops_at_git_root() {
        let dir = tempdir().unwrap();
        // create subdir structure dir/level1/level2
        let level1 = dir.path().join("level1");
        let level2 = level1.join("level2");
        std::fs::create_dir_all(&level2).unwrap();

        // Place a git root at dir/.git
        std::fs::create_dir(dir.path().join(".git")).unwrap();

        // Place lock file above git root to ensure it's not discovered
        std::fs::write(dir.path().join("pnpm-lock.yaml"), "mock").unwrap();

        // Running detect_manager starting from deepest path should NOT find the lock file above git root
        let result = detect_manager(&level2);
        assert!(result.is_err());
    }
} 