import sys
import datetime
import argparse
from textual.app import App, ComposeResult
from textual.widgets import Header, Footer, TabbedContent, TabPane, Button, Static
from textual.binding import Binding

from push_check import run_push_check
from config_manager import load_config, save_config, append_push_log, DEFAULT_CONFIG
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
    print("Cadence initialized successfully!")
    print(f"Created config.json: {cfg}")

def main():
    if len(sys.argv) > 1:
        cmd = sys.argv[1]
        if cmd == "init":
            init_repo()
            return
        elif cmd == "commit":
            # Delegate to commit helper
            from commit_helper import main as commit_main
            sys.argv = sys.argv[1:] # Shift args
            commit_main()
            return

    # Step 4.2 Push Check (runs on launch)
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
