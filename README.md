# Cadence

> **Automated Git progress pacing tool.**
> Keep committing naturally during development. Tag commits with scheduled release dates. Pushes commits on the right day automatically whenever you open the tool or run Cadence commands.

---

## 💡 How Cadence Works (For Humans & AI Agents)

Git commits form a linear pointer chain. A remote branch (like `origin/main`) is simply a pointer pointing to a commit object in that chain. 

When Cadence runs a **Push Check**:
1. It queries `origin/main` to find the last pushed commit object hash.
2. It walks forward through all unpushed local commits.
3. It parses the `Release-Date: YYYY-MM-DD` trailer embedded inside each commit message.
4. It finds the highest commit whose `Release-Date` is **today or earlier**.
5. It advances `origin/main` to that target commit hash.

This allows developers to build projects fast in single sessions while maintaining a steady, realistic daily activity cadence on GitHub without rewriting history or using complex background daemons.

---

## ⚙️ Project Configuration Files

Cadence uses dedicated project-specific hidden dotfiles stored in your repository root to avoid naming collisions with other tools or frameworks:

- **`.cadence.json`**: Stores user & repository settings (repository path, remote name, branch name, target timezone).
- **`.cadence_log.json`**: Audit history log of all automated and manual pushes executed by Cadence.

```json
// Example .cadence.json
{
  "repo_path": ".",
  "remote": "origin",
  "branch": "main",
  "timezone": "Asia/Kolkata"
}
```

---

## 🚀 Quickstart Guide & CLI Commands

### 1. One-Time Setup in Any Project
To initialize Cadence inside any git repository:
```bash
cadence init
```

### 2. Making Paced Commits
Use `cadence commit` as a drop-in wrapper around `git commit`:
```bash
# Automatically appends Release-Date: <today>
cadence commit "Implement user registration endpoint"

# Schedule a commit for a future date
cadence commit "Refactor database migrations" --date 2026-08-05
```

### 3. Checking Status & Managing Commits
```bash
# View summary of pushed, pending, scheduled, and unlabeled commits
cadence status

# Relabel/fix a commit's Release-Date trailer
cadence relabel HEAD 2026-08-01

# View push history log
cadence log

# Trigger explicit push check CLI command
cadence push
```

### 4. Terminal Dashboard (TUI)
Run `cadence` without subcommands to run startup push checks and open the TUI dashboard:
```bash
cadence
```

#### Keyboard Shortcuts in TUI:
- **`1`**: Switch to Timeline view
- **`2`**: Switch to Today's Batch view
- **`3`**: Switch to Push Log view
- **`4`**: Switch to Settings view
- **`p`**: Trigger Push Recheck
- **`r`**: Refresh view
- **`q`**: Quit Dashboard

---

## 🤖 Instructions for AI Coding Assistants

When operating inside a project managed by Cadence:
1. Prefer using `cadence commit "message"` over standard `git commit` to maintain proper release trailer tracking.
2. If custom release dates are needed, append `--date YYYY-MM-DD`.
3. Check `.cadence.json` to inspect or verify active remote branch settings.
