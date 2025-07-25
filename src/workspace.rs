use anyhow::{Result};
use log::debug;
use serde::Deserialize;
use std::fs::File;
use std::path::{Path, PathBuf};

/// Represents the `packages` field in a `pnpm-workspace.yaml` file.
#[derive(Debug, Deserialize)]
pub struct PnpmWorkspaceYaml {
    pub packages: Vec<String>,
}

/// Represents the two possible shapes of the `workspaces` field in `package.json`.
/// It can be either a direct array of globs, or an object containing a `packages` field.
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Workspaces {
    Globs(Vec<String>),
    Config { packages: Vec<String> },
}

/// Represents a subset of fields from a `package.json` file,
/// focusing on workspace configuration and the package name.
#[derive(Debug, Deserialize)]
pub struct PackageJson {
    /// The name of the package.
    pub name: Option<String>,
    /// Workspace configuration, which may be absent.
    #[serde(default)]
    pub workspaces: Option<Workspaces>,
}

/// Information about the discovered monorepo root.
#[derive(Debug)]
pub struct MonorepoRoot {
    /// The absolute path to the root directory.
    pub path: PathBuf,
    /// The list of workspace package globs defined.
    pub globs: Vec<String>,
}

/// Parses a `package.json` file to check if it defines a monorepo root.
fn parse_package_json_for_root(path: &Path) -> Result<Option<MonorepoRoot>> {
    let file = match File::open(path) {
        Ok(f) => f,
        Err(e) => {
            debug!("Could not open {}: {}", path.display(), e);
            return Ok(None);
        }
    };

    let config: PackageJson = match serde_json::from_reader(file) {
        Ok(c) => c,
        Err(_) => return Ok(None), // Ignore JSON parsing errors
    };

    let Some(workspaces_config) = config.workspaces else {
        return Ok(None); // Not a monorepo root if `workspaces` is missing
    };

    let globs = match workspaces_config {
        Workspaces::Globs(globs) => globs,
        Workspaces::Config { packages } => packages,
    };

    // Since `path` is the package.json, its parent is the root dir.
    let root_path = path.parent().unwrap().to_path_buf();
    Ok(Some(MonorepoRoot {
        path: root_path,
        globs,
    }))
}


/// Finds the monorepo root by searching upwards from a starting directory.
fn find_monorepo_root(start_dir: &Path) -> Result<Option<MonorepoRoot>> {
    for dir in start_dir.ancestors() {
        debug!("Searching for monorepo root in: {}", dir.display());

        // Check for pnpm
        let pnpm_config_path = dir.join("pnpm-workspace.yaml");
        if pnpm_config_path.exists() {
            debug!("Found pnpm-workspace.yaml at: {}", pnpm_config_path.display());
            let file = File::open(pnpm_config_path)?;
            let config: PnpmWorkspaceYaml = serde_yaml::from_reader(file)?;
            return Ok(Some(MonorepoRoot {
                path: dir.to_path_buf(),
                globs: config.packages,
            }));
        }

        // Check for yarn/npm by parsing package.json
        let package_json_path = dir.join("package.json");
        if package_json_path.exists() {
            if let Some(monorepo_root) = parse_package_json_for_root(&package_json_path)? {
                return Ok(Some(monorepo_root));
            }
        }

        // Stop at git boundary
        if dir.join(".git").exists() {
            debug!("Stopping at git root: {}", dir.display());
            break;
        }
    }

    Ok(None)
}

/// Discovers all workspace package names from the monorepo root.
pub fn discover_workspaces() -> Result<Vec<String>> {
    let current_dir = std::env::current_dir()?;
    
    let Some(root) = find_monorepo_root(&current_dir)? else {
        return Ok(vec![]);
    };

    debug!("Found monorepo root at '{}' with globs: {:?}", root.path.display(), root.globs);

    // TODO: Step 3 - Expand globs and parse package.json names.
    Ok(vec![])
} 