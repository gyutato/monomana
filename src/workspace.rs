use anyhow::{anyhow, Result};
use globset::{Glob, GlobSetBuilder};
use log::debug;
use serde::Deserialize;
use std::collections::HashSet;
use std::fs::File;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

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
        // Not in a monorepo, no workspaces to find.
        return Ok(vec![]);
    };
    debug!("Monorepo root found at: {}", root.path.display());

    let mut builder = GlobSetBuilder::new();
    for glob_str in &root.globs {
        let glob = Glob::new(glob_str)?;
        builder.add(glob);
    }
    let glob_set = builder.build()?;

    let mut workspace_names = HashSet::new();

    for entry in WalkDir::new(&root.path)
        .min_depth(1) // Skip the root directory itself
        .max_depth(4) // Optimization: limit search depth
        .into_iter()
        .filter_entry(|e| !is_hidden_or_node_modules(e))
        .filter_map(Result::ok) // Ignore errors during walk
    {
        let path = entry.path();
        // Check if the path relative to the root matches any of the globs
        if let Ok(relative_path) = path.strip_prefix(&root.path) {
            if glob_set.is_match(relative_path) {
                let package_json_path = path.join("package.json");
                if package_json_path.exists() {
                    if let Ok(name) = extract_package_name(&package_json_path) {
                        workspace_names.insert(name);
                    }
                }
            }
        }
    }

    let mut sorted_names: Vec<_> = workspace_names.into_iter().collect();
    sorted_names.sort();
    Ok(sorted_names)
}

/// Helper to check if a directory entry is hidden or `node_modules`.
fn is_hidden_or_node_modules(entry: &walkdir::DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with('.') || s == "node_modules")
        .unwrap_or(false)
}

/// Helper to extract the `name` from a `package.json` file.
fn extract_package_name(path: &Path) -> Result<String> {
    let file = File::open(path)?;
    let package_json: PackageJson = serde_json::from_reader(file)?;
    package_json.name.ok_or_else(|| anyhow!("`name` field missing in {}", path.display()))
}