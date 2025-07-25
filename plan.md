# monomana CLI – Autocompletion Roadmap (Next Milestone)

## Objective

Provide out-of-the-box zsh tab-completion that suggests workspace names for the first argument, **without requiring any manual setup** from the user.

## Functional Requirements

1. **Zero-config install** – After `cargo install monomana` (or the first run), tab-completion must already work in a typical zsh session.
2. **Dynamic workspace suggestions** – The completion list must reflect the current monorepo’s workspaces every time it is invoked.
3. **Performance** – Workspace discovery should be fast (<50 ms cold, <10 ms warm) and cached per repo.
4. **Safety** – Completion script is written only to user-writable paths; no root privileges assumed.

## High-Level Design

```
          ┌───────────────┐
          │   zsh user    │  hits <Tab>
          └──────┬────────┘
                 │ calls _monomana (completion fn)
          ┌──────▼────────┐
          │_monomana file │  if missing → created by binary
          └──────┬────────┘
                 │ executes `monomana __list_workspaces`
          ┌──────▼────────┐
          │ monomana CLI  │  fast workspace discovery
          └───────────────┘
```

### Components

1. **Workspace discovery**

   - If `pnpm-lock.yaml` present ⇒ parse `pnpm-workspace.yaml` → `packages` globs (yaml).
   - Else if `yarn.lock` present ⇒ parse root `package.json` → `workspaces` field (json).
   - Expand globs → dirs → read each `package.json` → collect `name` values.
   - Cache result in memory; optional on-disk cache keyed by repo hash + mtime.

2. **Hidden subcommand** `__list_workspaces`

   - Outputs one workspace name per line.
   - Fast path uses cache; slow path rescans.

3. **Completion template** (embedded string)

```zsh
#compdef monomana
_monomana() {
  local -a workspaces
  workspaces=(${(f)"$(monomana __list_workspaces)"})
  _arguments '1:workspace:(${workspaces})' '*::cmd:_normal'
}
_monomana "$@"
```

4. **Auto-install logic** (run-time)
   1. Determine writable completion dir:
      - `$XDG_DATA_HOME/zsh/site-functions` (fallback `~/.local/share/zsh/site-functions`).
      - If not in `$fpath`, pick `~/.zsh/completions` and show hint once.
   2. Compare SHA-256 of embedded template vs existing file `_monomana`.
   3. If file missing or outdated → write/replace.
   4. Skip when `MONOMANA_NO_COMPLETIONS=1`.

## Implementation Steps

1.  **Dependencies & Scaffolding**

    - [x] Add `clap_complete`, `globset`, `serde`, `sha2`, etc. to `Cargo.toml`.
    - [x] Create `workspace.rs` and `completion.rs` modules.

2.  **Workspace Discovery (Core Logic)**

    - [x] **Find Monorepo Root**: Implement `find_monorepo_root` to locate the top-level directory containing either `pnpm-workspace.yaml` or a `package.json` with a `workspaces` field. This includes parsing these files to extract the raw glob patterns.
    - [ ] **Expand Globs & Collect Names**:
      - In `discover_workspaces`, use the `globset` crate to match the collected patterns against directories within the monorepo root.
      - For each matched directory, read its `package.json`.
      - Parse the `package.json` and extract the `name` field.
      - Collect all unique names into a `Vec<String>`.
    - [ ] **(Optional) Caching**: Implement in-memory or on-disk caching for the discovered list to improve performance on subsequent runs.

3.  **CLI Integration**

    - [x] **Add Hidden Subcommand**: Extend `CliArgs` with a hidden `__list_workspaces` subcommand that calls `discover_workspaces` and prints each name on a new line.

4.  **Zsh Autocompletion**

    - [ ] **Generate & Embed Template**: Use `clap_complete` (likely in a `build.rs` script) to generate a static zsh completion script template and embed it in the binary. The template will be designed to call `monomana __list_workspaces`.
    - [ ] **Auto-install Logic**: In `completion.rs`, implement `ensure_zsh_completion_installed`. This function will check for the completion script in standard zsh paths (`$XDG_DATA_HOME/zsh/site-functions`, etc.) and write/update it if it's missing or outdated by comparing its hash with the embedded one.
    - [ ] **Startup Hook**: Call `ensure_zsh_completion_installed` from `main.rs` on startup, wrapped in a way that doesn't block the main functionality (e.g., in a separate thread or with a timeout).

5.  **Testing & Documentation**
    - [ ] Add unit tests for glob expansion and `package.json` name extraction.
    - [ ] Add integration tests to verify that `__list_workspaces` works correctly in a mock monorepo.
    - [ ] Add tests for the completion script installation logic using a temporary home directory.
    - [ ] Update the README to explain the automatic zsh completion feature.

## Nice-to-Have (stretch)

- Bash / Fish completion parity.
- On-disk cache with 1-second filewatch to auto-invalidate.
- Homebrew formula that links completion file to `/usr/share/zsh/site-functions`.
