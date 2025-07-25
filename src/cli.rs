use clap::{Parser, Subcommand};

use crate::detect::Manager;

#[derive(Parser, Debug)]
#[command(name = "monomana", version, about = "A lightning-fast monorepo manager")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Run a command in a workspace.
    #[command(name = "run")]
    Run(RunArgs),

    /// (internal) List available workspaces for completion.
    #[command(hide = true)]
    ListWorkspaces(ListWorkspacesArgs),
}

#[derive(Parser, Debug)]
pub struct RunArgs {
    /// Workspace name to execute the command in.
    pub workspace: String,

    /// Perform a dry run without executing the command.
    #[arg(long, short = 'd', default_value_t = false)]
    pub dry_run: bool,

    /// The command to run, optionally prefixed with the package manager (pnpm, yarn).
    #[arg(required = true, num_args = 1.., trailing_var_arg = true)]
    pub command_and_args: Vec<String>,
}

#[derive(Parser, Debug)]
pub struct ListWorkspacesArgs {}

/// Inspects the first token of the command to see if it's a manager keyword.
/// If so, returns the manager and removes the token from the list.
pub fn extract_manager_from_args(
    args: &mut Vec<String>,
) -> Option<Manager> {
    let manager_opt = args
        .first()
        .and_then(|token| match token.to_ascii_lowercase().as_str() {
            "pnpm" => Some(Manager::Pnpm),
            "yarn" => Some(Manager::Yarn),
            _ => None,
        });

    if manager_opt.is_some() {
        args.remove(0);
    }

    manager_opt
}
