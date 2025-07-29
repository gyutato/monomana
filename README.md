# monomana

`monomana` (**mono**repo **mana**ger) is a command-line interface (CLI) tool designed to simplify command execution within monorepo workspaces. It automatically detects the package manager (like pnpm or yarn) and runs your commands in the specified workspace.

## Installation

Since the tool is not published to crates.io yet, you can install it by cloning the repository and building it.
\
**[Beforehand, you need to install the Rust toolchain and the `cargo` package manager.](https://doc.rust-lang.org/cargo/getting-started/installation.html)**

```bash
# clone the repository
git clone https://github.com/monomana/monomana.git
cd monomana

# build and install
cargo install --path .
```

## Usage

The primary command is `run`, which allows you to execute commands in a specific workspace.

### Syntax

```bash
monomana run <workspace_name> [package_manager] <command_to_run>
```

- `<workspace_name>`: The name of the workspace where the command will be executed.
- `[package_manager]` (Optional): You can explicitly specify `pnpm` or `yarn`. If omitted, `monomana` will attempt to auto-detect the manager based on the lock files in the workspace.
- `<command_to_run>`: The actual command and its arguments you want to run (e.g., `build`, `test`, `dev`).

### Examples

**Running a 'dev' script in a workspace named 'webapp'**

The tool will auto-detect the package manager.

```bash
monomana run my-workspace dev
```

**Building a project in a workspace named 'api' using pnpm**

This example explicitly tells `monomana` to use `pnpm`.

```bash
monomana run my-pnpm-workspace pnpm build
```

**Performing a dry run**

You can see what command would be executed without actually running it by using the `--dry-run` or `-d` flag.

```bash
monomana run webapp --dry-run build
```

**check the workspace list**

The tool reads the `workspaces` field in the `package.json` file or the `pnpm-workspace.yaml` file automatically, and lists the workspaces.

```bash
monomana list-workspaces
```

## Auto-completion

`monomana` provides workspace auto-completion for zsh(oh-my-zsh) environment.

Press `tab` key after `monomana run` to auto-complete the workspace name.

### Oh-My-Zsh

If you are using oh-my-zsh, the auto-completion should be enabled by default.

**Error handling in this case is TBD.**

### zsh (without oh-my-zsh)

In vanilla zsh environment, you need to add the following to your `.zshrc` file.

For shorthand, add the following to your `.zshrc` file.

```bash
# your .zshrc
# ...other rc configs
ZSH_COMPLETION_DIR=$HOME/.zsh/completions
fpath=($ZSH_COMPLETION_DIR $fpath)
typeset -U fpath
autoload -Uz compinit
compinit
```

**Here is the detailed explanation of the above code:**

1. run `compinit`

```bash
# your .zshrc
# ...other rc configs
autoload -U compinit; compinit
```

2. add `fpath` path **before** running `compinit`

```bash
ZSH_COMPLETION_DIR=$HOME/.zsh/completions
fpath=($ZSH_COMPLETION_DIR $fpath)
typeset -U fpath
autoload -U compinit; compinit
```

3. (Optional) activate `GLOB_COMPLETE` option

```bash
setopt glob_complete
```
