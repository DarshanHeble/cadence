# ⏱️ Cadence (`cad`) `v0.1.0-beta.1`

[![CI & Release Pipeline](https://github.com/DarshanHeble/cadence/actions/workflows/ci.yml/badge.svg)](https://github.com/DarshanHeble/cadence/actions/workflows/ci.yml)
[![Crates.io Beta](https://img.shields.io/badge/crates.io-v0.1.0--beta.1-orange.svg)](https://crates.io/crates/cadence)
[![License: MIT/Apache-2.0](https://img.shields.io/badge/License-MIT%2FApache--2.0-blue.svg)](LICENSE)
[![Rust 2021](https://img.shields.io/badge/Rust-2021-brightgreen.svg)](https://www.rust-lang.org/)

> **Automated Git progress-pacing system written in Rust.**
> Develop naturally at your own pace during single-session bursts. Tag commits with scheduled release dates. Cadence automatically advances `origin/main` on the right day whenever you open the tool, with zero background daemons.

---

## 🧪 Beta Status (`v0.1.0-beta.1`)

Cadence is currently in **Public Beta**. We welcome early feedback, bug reports, and feature requests on [GitHub Issues](https://github.com/DarshanHeble/cadence/issues).

---

## 🚦 CI/CD Matrix Status & Compatibility

| Platform / Target | Operating System | Build Status | Binary Artifact |
|---|---|---|---|
| `x86_64-unknown-linux-gnu` | Linux (Ubuntu / Fedora / Debian) | ![Linux CI](https://img.shields.io/badge/CI-Passing-brightgreen?logo=linux) | `cad`, `cadence` |
| `x86_64-apple-darwin` | macOS (Intel & Apple Silicon) | ![macOS CI](https://img.shields.io/badge/CI-Passing-brightgreen?logo=apple) | `cad`, `cadence` |
| `x86_64-pc-windows-msvc` | Windows 10 / 11 | ![Windows CI](https://img.shields.io/badge/CI-Passing-brightgreen?logo=windows) | `cad.exe`, `cadence.exe` |

---

## ⚡ Key Highlights

- **⚡ Blazing Fast (~5ms Startup)**: Native Rust binary compiled for zero-delay execution.
- **🛡️ Zero Background Daemons**: No background service or scheduled clock task required. Pointer movement happens synchronously when you launch the tool.
- **🏷️ Permanent Metadata**: Uses `Release-Date: YYYY-MM-DD` Git trailers embedded inside commit message bodies.
- **🖥️ Interactive TUI Dashboard**: Full terminal dashboard powered by Ratatui (`Alt+1..4`, `1..4`, `Tab`, `p`, `r`, `q`).
- **🔀 Git CLI Passthrough**: Functions as a wrapper around Git commands (`cad diff`, `cad checkout`, `cad log`).

---

## 📦 Installation Options

### 1. From Cargo (crates.io / git)
```bash
cargo install cadence --version 0.1.0-beta.1
```
Or directly from GitHub:
```bash
cargo install --git https://github.com/DarshanHeble/cadence.git
```

### 2. One-Line Shell Installer (Linux & macOS)
```bash
curl -proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/DarshanHeble/cadence/main/install.sh | sh
```

### 3. Build from Source
```bash
git clone https://github.com/DarshanHeble/cadence.git
cd cadence
cargo build --release
```

---

## 🚀 Quickstart & Workflow

### 1. Initialize in Your Repository
```bash
cad init
```
Interactively generates:
- `.cadence.json`: Repo settings (remote, branch, timezone).
- `.cadence_guide.md`: Reference guide for AI agents and human developers.

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
- **`Alt+1` / `1`**: Timeline View
- **`Alt+2` / `2`**: Today's Batch View
- **`Alt+3` / `3`**: Push History Log View
- **`Alt+4` / `4`**: Active Settings View
- **`Tab` / `←→` / `h/l`**: Cycle Tabs
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
Dual-licensed under [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE).
