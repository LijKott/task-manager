# task-manager

A terminal task manager that lives in your shell, syncs across machines without an account, and never touches the cloud unless you want it to.

<!-- Replace with an asciinema GIF showing: tm add → tm list → tm tui session -->
![demo](./assets/demo.gif)

**[Install from crates.io](https://crates.io/crates/task-manager)**

---

## Quick start

```sh
cargo install task-manager
tm add "my first task"
tm list
```

That's it. Tasks persist to a local SQLite database in your home directory.

---

## Features

- Add, complete, and delete tasks from a single `tm` command
- Full-screen TUI (`tm tui`) with vim-style navigation — no mouse required
- Timestamps on every task: created, updated, completed
- File-based sync: dump to JSON, drop it anywhere (Dropbox, rsync, git), import on another machine
- Conflict resolution on import: the task with the newer `updated_at` wins, no manual merging

---

## How to run it locally

Requires **Rust 1.77+** (uses the 2024 edition). No system dependencies — SQLite is bundled.

```sh
git clone https://github.com/lijkott/task-manager
cd task-manager
cargo run -- add "test task"
```

To build an optimized release binary and put it on your PATH:

```sh
cargo install --path .
```

---

## Usage

```sh
tm add "task description"          # add a task
tm add "urgent thing" --priority high
tm list                            # all tasks
tm list --pending                  # incomplete only
tm done <id>                       # mark complete
tm delete <id>                     # remove

tm sync export --out ~/sync/tasks.json   # export snapshot
tm sync import --from ~/sync/tasks.json  # merge into local db

tm tui                             # open interactive UI
```

### TUI keybindings

| Key | Action |
|-----|--------|
| `j` / `k` | Move down / up |
| `Enter` | Toggle task complete |
| `a` | Add new task |
| `d` | Delete selected task |
| `s` | Open sync menu |
| `q` | Quit |

---

## How it works

Most task apps either lock your data in a proprietary format or require a sync service with an account. This one stores everything in a local SQLite file — one file, portable, readable by any SQLite client if you ever want out.

Sync is a deliberate non-feature in the networked sense. Instead of building a server, tasks serialize to a flat JSON snapshot via `serde`. You own the transport: point `--out` at a Dropbox folder, a git-tracked dotfiles repo, or an rsync target. On import, records merge by task ID and the `updated_at` timestamp breaks ties — so editing the same task on two machines picks the most recent version without a conflict prompt. This keeps the binary stateless and the sync logic under 50 lines.

The TUI is built with `ratatui` on top of `crossterm` for cross-platform terminal handling. State is kept in a single `App` struct that the event loop mutates; the render pass is a pure read of that struct, which makes it straightforward to test render output separately from input handling.

---

## Credits

- [clap](https://crates.io/crates/clap) — argument parsing
- [rusqlite](https://crates.io/crates/rusqlite) — embedded SQLite (bundled feature so no system install needed)
- [serde](https://crates.io/crates/serde) + [serde_json](https://crates.io/crates/serde_json) — sync serialization
- [ratatui](https://crates.io/crates/ratatui) — TUI framework
- [crossterm](https://crates.io/crates/crossterm) — cross-platform terminal I/O
- [chrono](https://crates.io/crates/chrono) — timestamps with serde support

---

## License

MIT
