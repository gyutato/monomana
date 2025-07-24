use anyhow::Result;

mod detect;
mod cli;
mod exec;
mod workspace;
mod completion;

fn main() -> Result<()> {
    env_logger::init();

    // 1. Parse CLI arguments.
    let (workspace, manager_opt, cmd_tokens, dry_run) = cli::parse_cli();

    // 2. Determine package manager (explicit or auto-detect).
    let manager = match manager_opt {
        Some(m) => m,
        None => {
            let cwd = std::env::current_dir()?;
            detect::detect_manager(cwd)?
        }
    };

    // 3. Execute command.
    let status = exec::run(&workspace, manager, &cmd_tokens, dry_run)?;

    // 4. Propagate exit code.
    std::process::exit(status.code().unwrap_or(1));
}
