use std::process::{Command, ExitStatus};

use anyhow::Result;

use crate::detect::Manager;

/// Build and execute the underlying package-manager command.
///
/// - `workspace`: workspace name to scope the command to.
/// - `manager`: selected package manager (pnpm | yarn).
/// - `cmd_tokens`: the rest of the CLI tokens to forward to the manager.
///
/// Returns the child process `ExitStatus`.
pub fn run(workspace: &str, manager: Manager, cmd_tokens: &[String]) -> Result<ExitStatus> {
    let mut command = match manager {
        Manager::Pnpm => {
            let mut cmd = Command::new("pnpm");
            cmd.args(cmd_tokens).arg("--filter").arg(workspace);
            cmd
        }
        Manager::Yarn => {
            let mut cmd = Command::new("yarn");
            cmd.arg("workspace").arg(workspace).args(cmd_tokens);
            cmd
        }
    };

    // Inherit stdio so that output streams directly to the console.
    let status = command.status()?;
    Ok(status)
} 