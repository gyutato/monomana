use anyhow::Result;

/// Discovers all workspace package names from the monorepo root.
pub fn discover_workspaces() -> Result<Vec<String>> {
    // TODO: Implement pnpm/yarn workspace discovery.
    Ok(vec![])
} 