# manage CLI Tool – Project Plan

## 1. Purpose

Provide a lightweight Rust-based CLI (`manage`) that simplifies running workspace-scoped package-manager commands for monorepos. It wraps common commands for `pnpm` and `yarn workspaces`, allowing developers to avoid repetitive `--filter` or `workspace` flags.

## 2. Key Features (v1)

- **Unified command**: `manage <workspace> [manager] <command…>`
- **Package-manager auto-detection** via lock files (`pnpm-lock.yaml`, `yarn.lock`).
- **Explicit manager override** (optional positional arg).
- **Command forwarding**: Executes underlying package-manager command while appending the correct workspace scope:
  - `pnpm`: `--filter <workspace>`
  - `yarn`: `workspace <workspace>`
- **Transparent I/O**: stdout/stderr streamed directly.
- **Exit codes**: propagate the child process exit status.

> Future (v2+): alias registry, npm/bun support, interactive selection, config file, etc.

## 3. Usage Examples

```bash
# auto-detect pnpm
manage web build                 #→ pnpm build --filter web

# auto-detect yarn
manage api add lodash            #→ yarn workspace api add lodash

# explicit manager override
manage ui pnpm outdated          #→ pnpm outdated --filter ui
manage api yarn add axios        #→ yarn workspace api add axios
```

## 4. Auto-Detection Algorithm

1. Starting at the current working directory, walk up to the git-root (or filesystem root).
2. Check for lock files in each directory:
   - `pnpm-lock.yaml` → select `pnpm`.
   - `yarn.lock` → select `yarn`.
3. If both are present in the same folder, default precedence:
   1. `pnpm-lock.yaml`
   2. `yarn.lock`
4. If no lock file found and no manager arg supplied → return an error & exit with non-zero code.

## 5. CLI Specification (clap)

Positional arguments:

1. `workspace` _(String, required)_ – workspace name.
2. `manager` _(Enum \[pnpm|yarn], optional)_ – overrides auto-detection.
3. `cmd`… _(Vec<String>, required)_ – command & args forwarded to the manager.

```
manage <workspace> [manager] <cmd>...
```

## 5.1. Parsing Rules – Distinguishing `manager` vs `cmd`

1. **workspace (required)**: the first positional token is _always_ interpreted as the workspace name.
2. **manager (optional)**: the _second_ token is treated as the manager **only if** it matches one of the supported manager keywords (case-insensitive): `pnpm` or `yarn`.
3. **cmd**: all remaining tokens are forwarded verbatim as the command to execute.

Examples

```
manage my-workspace add my-package       # manager omitted (auto-detect)
# → workspace = "my-workspace"
# → manager   = *auto-detect*
# → cmd       = ["add", "my-package"]

manage my-workspace yarn add my-package  # explicit manager
# → workspace = "my-workspace"
# → manager   = yarn
# → cmd       = ["add", "my-package"]
```

Edge case: if a command intentionally starts with the word `yarn` or `pnpm`, the parser will interpret it as the manager. To avoid ambiguity, users can pass the explicit `--manager` flag (future enhancement) or prepend a harmless argument (e.g., `--`).

## 6. High-Level Execution Flow

```mermaid
flowchart TD
    A[Parse CLI args] --> B{manager arg provided?}
    B -- yes --> C[Use provided manager]
    B -- no --> D[detect_manager()]
    D --> C
    C --> E{Manager}
    E -- pnpm --> F[Build: pnpm <cmd> --filter <workspace>]
    E -- yarn --> G[Build: yarn workspace <workspace> <cmd>]
    F --> H[spawn child process]
    G --> H
    H --> I[Stream stdout/stderr & propagate exit code]
```

## 7. Implementation Tasks

1. **Project setup**
   - Initialize Cargo project (already exists).
   - Add dependencies: `clap`, `serde`, `dirs`, `walkdir` (for traversal), `anyhow`.
2. **Define `Manager` enum** (`pnpm`, `yarn`).
3. **Implement `detect_manager()`**
   - Traverse directories upward; return `Result<Manager>`.
4. **CLI parsing with `clap`**
   - Positional args: `workspace`, `manager?`, `cmd...`.
5. **Command construction**
   - Build `Vec<String>` according to selected manager.
6. **Process execution**
   - Use `std::process::Command` with inherited I/O.
7. **Error handling & exit codes**
   - Map errors to meaningful messages; propagate child exit status.
8. **Testing & Validation**
   - Manual tests in sample pnpm/yarn repos.
   - Unit tests for detection function (mock filesystem).
9. **Documentation & Packaging**
   - Update `README.md` with instructions.

### 7.a. Detailed Task Breakdown (expanded)

- **Project setup**

  - Provide an example `Cargo.toml` snippet listing all required dependencies with versions.
  - Configure `edition = "2021"` and `warn-unused = "deny"` in `[workspace]` settings.

- **Module scaffolding**

  - Create module files: `cli.rs`, `detect.rs`, `exec.rs`, and `error.rs` for cleaner separation.
  - `main.rs` imports and orchestrates these modules.

- **detect_manager() specifics**

  - Walk directories upward until the git root (identified via `.git` folder) or filesystem root.
  - Log the search path when the `RUST_LOG=debug` env var is set (`env_logger`).
  - Unit-test using `tempdir` to simulate different lock-file layouts.

- **CLI parsing nuances**

  - Use `trailing_var_arg = true` to capture arbitrary length commands.
  - Reference §5.1 Parsing Rules in the help description.

- **Command construction**

  - Quote/escape the workspace argument if it contains whitespace or special shell characters.

- **Process execution details**

  - Forward SIGINT/SIGTERM to the child process for graceful shutdown.
  - On non-Unix platforms fallback to immediate kill if signal forwarding isn’t supported.
  - Propagate exit code; if unavailable, return `1`.

- **Error messaging**

  - Standardise errors (`thiserror` or `anyhow`) with user-friendly display, e.g.:
    - `"No lock file found. Specify manager explicitly or create a lock file."`

- **Testing strategy**

  - Unit: manager detection, command building.
  - Integration: `--dry-run` flag executes printing instead of spawning, enabling assertion of full command string.

- **Logging & Flags**
  - Add `--version` (clap auto), `--help` (auto), and `--verbose / -v` to toggle debug logging.

## 8. Future Enhancements

- Manager detection preference flags (e.g., `--prefer-yarn`).
- Support additional managers (`npm`, `bun`).
- Configurable alias system: `manage add <alias> ...`.
- Interactive workspace selection.
- Telemetry ‑- usage analytics (opt-in).

---

End of plan.
