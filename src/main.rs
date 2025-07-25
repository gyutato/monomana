use anyhow::Result;
use clap::Parser;
use cli::{Cli, Command};

mod cli;
mod completion;
mod detect;
mod exec;
mod workspace;

fn main() -> Result<()> {
    env_logger::init();
    let cli = Cli::parse();

    match cli.command {
        Command::Run(mut args) => {
            // 1. Determine package manager (explicit or auto-detect).
            let manager = match cli::extract_manager_from_args(&mut args.command_and_args) {
                Some(m) => m,
                None => {
                    let cwd = std::env::current_dir()?;
                    detect::detect_manager(cwd)?
                }
            };

            // 2. Execute command.
            let status = exec::run(
                &args.workspace,
                manager,
                &args.command_and_args,
                args.dry_run,
            )?;

            // 3. Propagate exit code.
            std::process::exit(status.code().unwrap_or(1));
        }
        Command::ListWorkspaces(_) => {
            let workspaces = workspace::discover_workspaces()?;
            for workspace in workspaces {
                println!("{}", workspace);
            }
            Ok(())
        }
    }
}
