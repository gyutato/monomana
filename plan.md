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

1. **Dependencies**
   - `clap_complete` for template generation (compile-time helper).
   - `globset`, `serde`, `serde_json`, `serde_yaml`, `dirs`, `sha2`.
2. **Module scaffold**
   - `workspace.rs` → discover & cache workspaces.
   - `completion.rs` → install logic & template embed.
3. **Add subcommand**
   - Extend `CliArgs` with hidden `__list_workspaces`.
4. **On startup**
   - Call `completion::ensure_zsh_completion_installed()`.
5. **Unit tests**
   - Mock repo layout → verify workspace discovery.
   - Temp HOME → ensure file written and hash changes update it.
6. **Doc update**
   - README “Tab-completion works automatically in zsh”.

## Nice-to-Have (stretch)

- Bash / Fish completion parity.
- On-disk cache with 1-second filewatch to auto-invalidate.
- Homebrew formula that links completion file to `/usr/share/zsh/site-functions`.
