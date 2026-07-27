# AGENT.md - AI Agent Operating Manual for Cadence Repositories

This document serves as the authoritative protocol for AI coding assistants (such as Antigravity, GitHub Copilot, Cursor, etc.) working inside repositories managed by **Cadence (`cad`)**.

---

## 🎯 Core Directive for AI Agents

Whenever you stage and commit code changes in a Cadence-managed repository, **DO NOT run standard `git commit`**.

Instead, always use the **Cadence CLI executable (`cad commit` or `cadence commit`)**:

```bash
cad commit "Your clear, descriptive commit message"
```

---

## 🛠️ Cadence Mechanics Overview

Cadence manages Git commit progress pacing without force-pushing or rewriting linear history.

1. **Linear Pointer Advancement**: Cadence checks where `origin/main` currently points, inspects unpushed commits for the `Release-Date: YYYY-MM-DD` trailer, and moves `origin/main` forward to the latest commit due today.
2. **Trailer Injection**: Running `cad commit` automatically stages modified/new files and appends `Release-Date: YYYY-MM-DD` to the commit message body.

---

## 📌 Standard Command Reference for AI Agents

| Task | Recommended Command |
|---|---|
| **Make a normal daily commit** | `cad commit "Implement user authentication"` |
| **Schedule a future-dated commit** | `cad commit "Add database migration" --date YYYY-MM-DD` |
| **Fix/Update release trailer on existing commit** | `cad relabel [HEAD\|commit_hash] YYYY-MM-DD` |
| **Check commit pacing status** | `cad status` |
| **Trigger manual push check** | `cad push` |
| **View push audit history log** | `cad log` |
| **Run standard git actions** | `cad diff`, `cad checkout`, `cad branch` (passthrough commands) |

---

## ⚙️ Configuration File Rules

- **`.cadence.json`**: Repository settings (`repo_path`, `remote`, `branch`, `timezone`). Do not overwrite unless explicitly asked by the user.
- **`.cadence_log.json`**: Push history log managed by Cadence.
- **`.cadence_guide.md` & `AGENT.md`**: AI & developer guidance docs.
