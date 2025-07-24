use clap::Parser;

use crate::detect::Manager;

/// Raw CLI argument parsing based on the spec:
/// `manage <workspace> [manager] <cmd>...`
#[derive(Parser, Debug)]
#[command(name = "manage", version, about = "Workspace-scoped package manager wrapper")]
pub struct CliArgs {
    /// Workspace name (first positional argument)
    pub workspace: String,

    /// Perform a dry run without executing the command
    #[arg(long, short = 'd', default_value_t = false)]
    pub dry_run: bool,

    /// Remaining tokens (manager? + command) captured verbatim.
    #[arg(required = true, num_args = 1.., trailing_var_arg = true)]
    pub args: Vec<String>,
}

/// Parse CLI args and split into components.
/// Returns (workspace, optional manager, command tokens, dry_run)
pub fn parse_cli() -> (String, Option<Manager>, Vec<String>, bool) {
    let raw = CliArgs::parse();

    let mut args = raw.args; // full list of remaining tokens

    // Inspect first token (if any) to see if it's a manager keyword
    let manager_opt = args.first().and_then(|token| match token.to_ascii_lowercase().as_str() {
        "pnpm" => Some(Manager::Pnpm),
        "yarn" => Some(Manager::Yarn),
        _ => None,
    });

    // If the first token was a manager, remove it from the args list
    if manager_opt.is_some() {
        args.remove(0);
    }

    (raw.workspace, manager_opt, args, raw.dry_run)
}
