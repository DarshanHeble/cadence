# Cadence (Rust)

> **Automated Git progress pacing tool written in Rust.**
> Keep committing naturally during development. Tag commits with scheduled release dates. Pushes commits on the right day automatically whenever you open the tool or run Cadence commands.

---

## ⚡ Key Features (Rust Edition)

- **Single Standalone Executable**: Native high-performance `cad` and `cadence` binaries compiled in Rust (~5ms startup).
- **Zero Background Daemons**: No clock or background thread running between sessions.
- **Ratatui Terminal UI**: Interactive terminal dashboard supporting tabs, keyboard shortcuts (`1-4`, `p`, `r`, `q`), and progress timeline rendering.
- **Git Passthrough**: Subcommands like `cad status`, `cad push`, `cad relabel`, `cad log`, `cad init`, plus native passthrough to `git` (`cad diff`, `cad checkout`).

---

## 💡 How Cadence Works

Git commits form a linear pointer chain. A remote branch (like `origin/main`) is simply a pointer pointing to a commit object in that chain. 

When Cadence runs a **Push Check**:
1. It queries `origin/main` to find the last pushed commit object hash.
2. It walks forward through all unpushed local commits.
3. It parses the `Release-Date: YYYY-MM-DD` trailer embedded inside each commit message.
4. It finds the highest commit whose `Release-Date` is **today or earlier**.
5. It advances `origin/main` to that target commit hash.

---

## 🚀 Quickstart & Commands

```bash
# Build release binary
cargo build --release

# Install executables locally (~/.cargo/bin/cad and cadence)
cargo install --path .

# CLI Commands
cad init                              # Initialize repository config
cad commit "Implement feature"       # Stage & commit with Release-Date: <today>
cad commit "Future work" --date 2026-08-05
cad status                            # View status summary
cad push                              # Execute manual push check
cad relabel HEAD 2026-08-01           # Modify release date trailer
cad log                               # View push history log
cad                                   # Launch interactive TUI Dashboard
```

---

## ⚙️ Configuration Dotfiles

- **`.cadence.json`**: Repository settings (`repo_path`, `remote`, `branch`, `timezone`).
- **`.cadence_log.json`**: Audit history log of pushes.
- **`.cadence_guide.md`**: Automatically generated developer & AI instructions guide.
