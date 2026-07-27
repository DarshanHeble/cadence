from textual.containers import Container, Vertical, Horizontal
from textual.widgets import Static, Input, Button
from config_manager import load_config, save_config

class SettingsView(Container):
    def compose(self):
        yield Static("[bold cyan]Cadence Settings[/bold cyan]\n", id="settings-title")
        
        with Vertical(id="settings-form"):
            yield Static("Repository Path:")
            yield Input(id="input-repo-path", placeholder="/path/to/repository")
            
            yield Static("Remote Name:")
            yield Input(id="input-remote", placeholder="origin")
            
            yield Static("Branch Name:")
            yield Input(id="input-branch", placeholder="main")
            
            yield Static("Timezone:")
            yield Input(id="input-timezone", placeholder="Asia/Kolkata")
            
            yield Static("")
            yield Button("Save Settings", id="btn-save-settings", variant="success")
            yield Static("", id="settings-status")

    def on_mount(self):
        self.load_current_settings()

    def load_current_settings(self):
        cfg = load_config()
        self.query_one("#input-repo-path", Input).value = cfg.get("repo_path", ".")
        self.query_one("#input-remote", Input).value = cfg.get("remote", "origin")
        self.query_one("#input-branch", Input).value = cfg.get("branch", "main")
        self.query_one("#input-timezone", Input).value = cfg.get("timezone", "Asia/Kolkata")

    def save_settings(self):
        cfg = {
            "repo_path": self.query_one("#input-repo-path", Input).value,
            "remote": self.query_one("#input-remote", Input).value,
            "branch": self.query_one("#input-branch", Input).value,
            "timezone": self.query_one("#input-timezone", Input).value,
        }
        save_config(cfg)
        self.query_one("#settings-status", Static).update("[bold green]Settings saved successfully![/bold green]")
