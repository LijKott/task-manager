# task-manager

A fast, keyboard-driven CLI task manager written in Rust. Manage tasks from the terminal with a clean command interface or an interactive TUI, backed by a local SQLite database and optional file-based sync so your tasks follow you across machines.

## Features

- **CLI interface** — add, list, complete, and delete tasks with simple subcommands via `clap`
- **Interactive TUI** — browse and manage tasks in a terminal UI built with `ratatui` and `crossterm`
- **Persistent storage** — tasks are stored in a local SQLite database via `rusqlite`
- **Timestamps** — creation and completion times tracked with `chrono`
- **File-based sync** — export/import a portable JSON snapshot to sync across machines (no account required)
- **Serializable data** — all models implement `serde` for seamless JSON round-trips

## Installation

### From crates.io

```sh
cargo install task-manager
```

### From source

```sh
git clone https://github.com/lijkott/task-manager
cd task-manager
cargo install --path .
```

## Usage

### CLI commands

```sh
# Add a new task
tm add "Write the sync module"

# Add a task with a priority tag
tm add "Fix critical bug" --priority high

# List all tasks
tm list

# List only pending tasks
tm list --pending

# Mark a task as done
tm done <id>

# Delete a task
tm delete <id>

# Sync: export tasks to a JSON file
tm sync export --out ~/Dropbox/tasks.json

# Sync: import tasks from a JSON file
tm sync import --from ~/Dropbox/tasks.json
```

### Interactive TUI

Launch the full-screen terminal UI:

```sh
tm tui
```

| Key | Action |
|-----|--------|
| `j` / `k` | Move down / up |
| `Enter` | Toggle task complete |
| `a` | Add new task |
| `d` | Delete selected task |
| `s` | Open sync menu |
| `q` | Quit |

## Sync strategy

Sync is intentionally simple and dependency-free: tasks are serialized to a single JSON file that you drop anywhere — a shared folder, a USB drive, a dotfiles repo. On import, tasks are merged by ID; newer `updated_at` timestamps win conflicts.

This means you can sync across machines using any file-sharing method you already use (Dropbox, Syncthing, rsync, git, etc.) without needing an account or network service.

## Tech stack

| Crate | Role |
|-------|------|
| [`clap`](https://crates.io/crates/clap) | Argument parsing with derive macros |
| [`rusqlite`](https://crates.io/crates/rusqlite) | Embedded SQLite storage (bundled feature) |
| [`serde`](https://crates.io/crates/serde) + [`serde_json`](https://crates.io/crates/serde_json) | Model serialization for sync |
| [`ratatui`](https://crates.io/crates/ratatui) | Terminal UI rendering |
| [`crossterm`](https://crates.io/crates/crossterm) | Cross-platform terminal input/output |
| [`chrono`](https://crates.io/crates/chrono) | Timestamps with serde support |

## Project structure

```
src/
  main.rs       # Entry point, CLI dispatch
  commands.rs   # Clap command definitions and handlers
  db.rs         # SQLite schema, queries, migrations
  models.rs     # Task struct and serde impls
  tui.rs        # Ratatui app loop and keybindings
```

## License

MIT
