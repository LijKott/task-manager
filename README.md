# task-manager

A terminal task manager that stores tasks locally in SQLite and never touches the cloud.

![demo](./assets/demo.gif)

---

## Quick start

**Via cargo:**

```sh
cargo install task-manager-kotter
```

**Via pre-built binary** (no Rust required):

1. Download the binary for your platform from the [latest release](https://github.com/lijkott/task-manager/releases/latest):
   - `tsk-x86_64-unknown-linux-gnu` — Linux
   - `tsk-x86_64-apple-darwin` — macOS (Intel)
   - `tsk-aarch64-apple-darwin` — macOS (Apple Silicon)
   - `tsk-x86_64-pc-windows-msvc.exe` — Windows
2. Make it executable and move it onto your PATH (Linux/macOS):
   ```sh
   chmod +x tsk-*
   mv tsk-* ~/.local/bin/tsk
   ```
3. On Windows, rename it to `tsk.exe` and place it somewhere on your `PATH`.

Tasks persist to a local SQLite database in your project directory.

---

## Features

- Add, list, complete, and delete tasks from a single command
- Full-screen TUI (`task-manager-kotter tui`) with arrow-key navigation
- Toggle tasks done/undone — flip the status of any task at any time
- Reset the entire task list with one command
- No account, no network, no system dependencies — SQLite is bundled

---

## How to run it locally

Requires **Rust 1.85+** (uses the 2024 edition). No system dependencies — SQLite is bundled via the `bundled` feature.

```sh
git clone https://github.com/lijkott/task-manager
cd task-manager
cargo run
```

Running without a subcommand opens the TUI directly.

---

## Usage

```sh
task-manager-kotter add "task description"   # add a task
task-manager-kotter list                     # list all tasks
task-manager-kotter done <id>                # toggle task complete/incomplete
task-manager-kotter delete <id>              # remove a task
task-manager-kotter reset                    # delete all tasks
task-manager-kotter tui                      # open interactive UI
```

Running `task-manager-kotter` with no arguments also opens the TUI.

### TUI keybindings

| Key | Action |
|-----|--------|
| `↑` / `↓` | Move up / down |
| `a` | Add new task |
| `d` | Toggle selected task done/undone |
| `x` | Delete selected task |
| `r` | Reset all tasks |
| `q` | Quit |

**While adding a task:**

| Key | Action |
|-----|--------|
| `Enter` | Confirm and save |
| `Esc` | Cancel |
| `Backspace` | Delete last character |

---

## How it works

Tasks are stored in a local SQLite file managed by `rusqlite` with the `bundled` feature, so there is no system SQLite install required — the library is compiled in. The CLI is wired up with `clap`'s derive API; adding a new subcommand is a one-line enum variant.

The TUI is built with `ratatui` on top of `crossterm` for cross-platform terminal handling. State lives in a single `App` struct with two modes — `Normal` and `Adding`. The event loop mutates the struct; the render pass reads it. This separation keeps input handling and rendering easy to follow independently.

---

## Credits

- [clap](https://crates.io/crates/clap) — argument parsing
- [rusqlite](https://crates.io/crates/rusqlite) — embedded SQLite (bundled, no system install needed)
- [ratatui](https://crates.io/crates/ratatui) — TUI framework
- [crossterm](https://crates.io/crates/crossterm) — cross-platform terminal I/O
- [chrono](https://crates.io/crates/chrono) — timestamps with serde support
- [serde](https://crates.io/crates/serde) + [serde_json](https://crates.io/crates/serde_json) — serialization

---

## License

MIT
