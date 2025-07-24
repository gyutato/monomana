use std::process::{Command, ExitStatus};
#[cfg(unix)]
use std::os::unix::process::ExitStatusExt;

use anyhow::Result;

use crate::detect::Manager;

/// Build and execute the underlying package-manager command.
///
/// - `workspace`: workspace name to scope the command to.
/// - `manager`: selected package manager (pnpm | yarn).
/// - `cmd_tokens`: the rest of the CLI tokens to forward to the manager.
/// - `dry_run`: if true, print the command instead of executing.
///
/// Returns the child process `ExitStatus`.
pub fn run(
    workspace: &str,
    manager: Manager,
    cmd_tokens: &[String],
    dry_run: bool,
) -> Result<ExitStatus> {
    let mut command = match manager {
        Manager::Pnpm => {
            let mut cmd = Command::new("pnpm");
            cmd.arg("--filter").arg(workspace).args(cmd_tokens);
            cmd
        }
        Manager::Yarn => {
            let mut cmd = Command::new("yarn");
            cmd.arg("workspace").arg(workspace).args(cmd_tokens);
            cmd
        }
    };

    if dry_run {
        println!("DRY RUN: {:?}", command);
        // Return a successful exit status for dry runs
        #[cfg(unix)]
        return Ok(std::process::ExitStatus::from_raw(0));
        #[cfg(not(unix))]
        return Ok(Command::new("cmd").arg("/C").arg("exit 0").status()?);
    }

    // Inherit stdio so that output streams directly to the console.
    let status = command.status()?;
    Ok(status)
} 