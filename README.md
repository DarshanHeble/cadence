# ⏱️ Cadence (`cad`)

> **Automated Git progress-pacing system written in Rust.**
> Develop naturally at your own pace during single-session bursts. Tag commits with scheduled release dates. Cadence automatically advances `origin/main` on the right day whenever you open the tool, with zero background daemons.

---

![Cadence Dashboard Banner](docs/assets/cadence_dashboard_preview.png)

---

## 📸 Recommended Screenshots & Visual Assets to Include

Add the following screenshot files into `docs/assets/` to make your GitHub repository README look top-tier:

1. **`cadence_dashboard_preview.png`**: A screenshot of the Ratatui TUI dashboard open on the **Timeline tab** showing pushed (filled green block) vs pending vs scheduled commits.
2. **`cadence_status_preview.png`**: Screenshot of running `cad status` in your terminal demonstrating the summary counts.
3. **`cadence_commit_preview.png`**: Screenshot of running `cad commit "My commit"` showing automatic trailer addition.

---

## ⚡ Key Highlights

- **⚡ Blazing Fast (~5ms Startup)**: Written in pure Rust with Ratatui & Crossterm.
- **🛡️ Zero Background Daemons**: No background service or scheduled clock task required. Pushing happens synchronously when you launch the tool.
- **🏷️ Permanent Metadata**: Uses `Release-Date: YYYY-MM-DD` Git trailers embedded inside commit message bodies.
- **🖥️ Interactive TUI Dashboard**: Full terminal dashboard with keyboard shortcuts (`1-4`, `p`, `r`, `q`).
- **🔀 Git CLI Passthrough**: Functions as a wrapper around Git commands (`cad diff`, `cad checkout`, `cad log`).

---

## 📦 Installation

### Option 1: Cargo Install
```bash
cargo install --path .
```

### Option 2: Build from Source
```bash
git clone https://github.com/DarshanHeble/cadence.git
cd cadence
cargo build --release
```
The compiled binaries will be located at `target/release/cad` and `target/release/cadence`.

---

## 🚀 Quickstart & Workflow

### 1. Initialize in Your Repository
```bash
cad init
```
This interactively generates:
- `.cadence.json`: Repo settings (remote, branch, timezone).
- `.cadence_guide.md`: Quick reference guide for AI agents and human developers.

### 2. Make Paced Commits
```bash
# Stage changes & commit with today's Release-Date trailer
cad commit "Add user authentication module"

# Pre-label a commit for a future date
cad commit "Refactor database models" --date 2026-08-05
```

### 3. Check Status & Logs
```bash
cad status                            # View summary breakdown
cad push                              # Execute manual push check
cad relabel HEAD 2026-08-01           # Fix/update trailer date on a commit
cad log                               # View push history log
```

### 4. Interactive TUI Dashboard
Simply run `cad` to execute startup push checks and open the dashboard:
```bash
cad
```

#### TUI Keyboard Shortcuts:
- **`1`**: Timeline View
- **`2`**: Today's Batch View
- **`3`**: Push History Log View
- **`4`**: Active Settings View
- **`p`**: Trigger Push Recheck
- **`r`**: Refresh Screen
- **`q`**: Quit Dashboard

---

## 🤖 AI Assistant Guidelines

When AI coding assistants (Antigravity, Copilot, Cursor, etc.) operate inside a Cadence repository, they should inspect `.cadence_guide.md` and use `cad commit "message"` instead of raw `git commit`.

---

## 🛠️ CI/CD & Development

- **Formatting Check**: `cargo fmt --all -- --check`
- **Linting**: `cargo clippy --all-targets -- -D warnings`
- **Tests**: `cargo test --all`

---

## 📜 License
Dual-licensed under MIT or Apache-2.0.
