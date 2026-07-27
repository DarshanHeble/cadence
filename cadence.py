import sys
import datetime
import argparse
from textual.app import App, ComposeResult
from textual.widgets import Header, Footer, TabbedContent, TabPane, Button, Static
from textual.binding import Binding

from push_check import run_push_check, get_today_str, get_all_commits

from config_manager import load_config, save_config, append_push_log, load_push_log, DEFAULT_CONFIG

from commit_helper import commit_with_date

from ui.timeline_view import TimelineView
from ui.batch_view import BatchView
from ui.log_view import LogView
from ui.settings_view import SettingsView

class CadenceApp(App):
    CSS = """
    Screen {
        background: $surface;
    }
    #timeline-title, #batch-title, #log-title, #settings-title {
        margin: 1 1;
    }
    .warning-box {
        background: $error-darken-3;
        color: $text;
        padding: 1;
        margin-bottom: 1;
        border: solid red;
    }
    #settings-form {
        width: 60;
        margin: 1 2;
    }
    Input {
        margin-bottom: 1;
    }
    """

    BINDINGS = [
        Binding("1", "show_tab('tab-timeline')", "1: Timeline", show=True),
        Binding("2", "show_tab('tab-batch')", "2: Today's Batch", show=True),
        Binding("3", "show_tab('tab-log')", "3: Push Log", show=True),
        Binding("4", "show_tab('tab-settings')", "4: Settings", show=True),
        Binding("p", "recheck_push", "p: Push Recheck", show=True),
        Binding("r", "refresh_all", "r: Refresh", show=True),
        Binding("q", "quit", "q: Quit", show=True),
    ]

    def action_show_tab(self, tab_id: str) -> None:
        tabbed = self.query_one(TabbedContent)
        tabbed.active = tab_id

    def action_refresh_all(self) -> None:
        timeline = self.query_one("#view-timeline", TimelineView)
        timeline.refresh_timeline()
        log_view = self.query_one("#view-log", LogView)
        log_view.refresh_log()


    def __init__(self, initial_push_result=None):
        super().__init__()
        self.initial_push_result = initial_push_result or {}

    def compose(self) -> ComposeResult:
        yield Header(show_clock=True)
        with TabbedContent(initial="tab-timeline"):
            with TabPane("Timeline", id="tab-timeline"):
                yield TimelineView(id="view-timeline")
            with TabPane("Today's Batch", id="tab-batch"):
                yield BatchView(id="view-batch")
            with TabPane("Push Log", id="tab-log"):
                yield LogView(id="view-log")
            with TabPane("Settings", id="tab-settings"):
                yield SettingsView(id="view-settings")
        yield Footer()

    def on_mount(self):
        # Refresh timeline view
        timeline = self.query_one("#view-timeline", TimelineView)
        timeline.refresh_timeline()

        # Update batch view with startup push check result
        batch = self.query_one("#view-batch", BatchView)
        if self.initial_push_result:
            batch.update_results(self.initial_push_result)

        # Refresh push log
        log_view = self.query_one("#view-log", LogView)
        log_view.refresh_log()

    def action_recheck_push(self):
        res = run_push_check()
        if res.get("pushed"):
            entry = {
                "timestamp": datetime.datetime.now().isoformat(),
                "message": res.get("message"),
                "count": res.get("count"),
                "commits": res.get("pushed_commits", [])
            }
            append_push_log(entry)

        # Refresh views
        batch = self.query_one("#view-batch", BatchView)
        batch.update_results(res)

        timeline = self.query_one("#view-timeline", TimelineView)
        timeline.refresh_timeline()

        log_view = self.query_one("#view-log", LogView)
        log_view.refresh_log()

    def on_button_pressed(self, event: Button.Pressed) -> None:
        if event.button.id == "btn-recheck":
            self.action_recheck_push()
        elif event.button.id == "btn-save-settings":
            settings = self.query_one("#view-settings", SettingsView)
            settings.save_settings()

def init_repo():
    print("Initializing Cadence in current repository...")
    repo_path = input("Repository path [.]: ").strip() or "."
    remote = input("Remote name [origin]: ").strip() or "origin"
    branch = input("Branch name [main]: ").strip() or "main"
    tz = input("Timezone [Asia/Kolkata]: ").strip() or "Asia/Kolkata"

    cfg = {
        "repo_path": repo_path,
        "remote": remote,
        "branch": branch,
        "timezone": tz
    }
    save_config(cfg)

    # Generate project-specific .cadence_guide.md for developers & AI agents
    guide_content = """# Cadence Project Guide & Instructions

This repository is managed by **Cadence** - an automated Git progress pacing system.

## 🤖 Instructions for AI Coding Assistants & Developers

When making commits in this project, **always use `cadence commit` instead of standard `git commit`**. 
Cadence automatically attaches the required `Release-Date: YYYY-MM-DD` trailer to ensure steady progress pacing on GitHub.

### 📌 Cadence CLI Commands Reference

| Command | Usage & Purpose |
|---|---|
| `cadence commit "message"` | Stage files & create a commit labeled with **today's date**. |
| `cadence commit "message" --date YYYY-MM-DD` | Stage files & pre-label a commit for a **future date**. |
| `cadence status` | Show current commit counts (Pushed vs. Pending vs. Scheduled vs. Unlabeled). |
| `cadence push` | Manually execute push check to advance remote `origin/main` for eligible commits. |
| `cadence relabel [HEAD|hash] YYYY-MM-DD` | Modify/fix the `Release-Date` trailer on an existing commit. |
| `cadence log` | Display audit log of previous pushes. |
| `cadence` | Launch the interactive Terminal Dashboard (TUI). |
| `cadence <git-command>` | Passthrough standard git commands (e.g. `cadence diff`, `cadence status`, `cadence checkout`). |

---

## ⚙️ Configuration Files
- `.cadence.json`: Local settings (remote name, branch, timezone).
- `.cadence_log.json`: Push history audit log.
- `.cadence_guide.md`: This reference file.
"""
    with open(".cadence_guide.md", "w", encoding="utf-8") as f:
        f.write(guide_content)

    print("Cadence initialized successfully!")
    print(f"Created .cadence.json: {cfg}")
    print("Created .cadence_guide.md for developer and AI guidance.")



def print_status():
    cfg = load_config()
    cwd = cfg.get("repo_path", ".")
    remote = cfg.get("remote", "origin")
    branch = cfg.get("branch", "main")
    tz_name = cfg.get("timezone", "Asia/Kolkata")
    today = get_today_str(tz_name)

    print(f"Cadence Status for repository: {cwd}")
    print(f"Remote: {remote} | Branch: {branch} | Timezone: {tz_name} | Today: {today}\n")

    commits = get_all_commits(cwd, remote, branch)
    if not commits:
        print("No commits found.")
        return

    pushed_count = sum(1 for c in commits if c["pushed"])
    pending_count = sum(1 for c in commits if not c["pushed"] and c["release_date"] and c["release_date"] <= today)
    scheduled_count = sum(1 for c in commits if not c["pushed"] and c["release_date"] and c["release_date"] > today)
    unlabeled_count = sum(1 for c in commits if not c["pushed"] and not c["release_date"])

    print(f"Pushed commits: {pushed_count}")
    print(f"Pending due commits: {pending_count}")
    print(f"Scheduled future commits: {scheduled_count}")
    if unlabeled_count > 0:
        print(f"\033[91m⚠️ Unlabeled commits: {unlabeled_count} (queue blocked until labeled!)\033[0m")
    print()

def print_log():
    logs = load_push_log()
    if not logs:
        print("No push history recorded yet.")
        return
    print("=== Cadence Push History Log ===")
    for entry in reversed(logs):
        print(f"🚀 Push at {entry.get('timestamp', '')} - {entry.get('message', '')}")
        for c in entry.get("commits", []):
            print(f"   • {c.get('short_hash', '')} - {c.get('subject', '')} ({c.get('release_date', '')})")

def main():
    if len(sys.argv) > 1:
        cmd = sys.argv[1]
        if cmd == "init":
            init_repo()
            return
        elif cmd == "commit":
            from commit_helper import main as commit_main
            sys.argv = sys.argv[1:]
            commit_main()
            return
        elif cmd == "relabel":
            from commit_helper import relabel_commit
            if len(sys.argv) < 3:
                print("Usage: cadence relabel [commit_ref] YYYY-MM-DD")
                print("Example: cadence relabel HEAD 2026-08-01")
                return
            ref = sys.argv[2] if len(sys.argv) > 3 else "HEAD"
            d = sys.argv[3] if len(sys.argv) > 3 else sys.argv[2]
            relabel_commit(ref, d)
            return
        elif cmd == "status":
            print_status()
            return
        elif cmd == "log":
            print_log()
            return
        elif cmd == "push":
            res = run_push_check()
            print(res.get("message"))
            if res.get("pushed"):
                entry = {
                    "timestamp": datetime.datetime.now().isoformat(),
                    "message": res.get("message"),
                    "count": res.get("count"),
                    "commits": res.get("pushed_commits", [])
                }
                append_push_log(entry)
            return
        elif cmd in ["add", "checkout", "branch", "diff", "fetch", "pull", "merge", "rebase", "reset"]:
            # Passthrough git CLI commands
            import subprocess
            subprocess.run(["git"] + sys.argv[1:])
            return

    # Default launch behavior: run push check then open TUI
    push_res = run_push_check()
    if push_res.get("pushed"):
        entry = {
            "timestamp": datetime.datetime.now().isoformat(),
            "message": push_res.get("message"),
            "count": push_res.get("count"),
            "commits": push_res.get("pushed_commits", [])
        }
        append_push_log(entry)

    app = CadenceApp(initial_push_result=push_res)
    app.run()

if __name__ == "__main__":
    main()

