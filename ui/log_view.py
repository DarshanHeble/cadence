from textual.containers import Container, VerticalScroll
from textual.widgets import Static
from config_manager import load_push_log

class LogView(Container):
    def compose(self):
        yield Static("[bold cyan]Push History Log[/bold cyan]\n", id="log-title")
        yield VerticalScroll(id="log-list")

    def refresh_log(self):
        logs = load_push_log()
        log_box = self.query_one("#log-list", VerticalScroll)
        log_box.remove_children()

        if not logs:
            log_box.mount(Static("[dim]No push activity logged yet.[/dim]"))
            return

        for entry in reversed(logs):
            ts = entry.get("timestamp", "N/A")
            msg = entry.get("message", "")
            count = entry.get("count", 0)
            
            header = f"[bold green]🚀 Push on {ts}[/bold green] ({count} commit(s))"
            log_box.mount(Static(header))
            log_box.mount(Static(f"  {msg}"))
            
            commits = entry.get("commits", [])
            for c in commits:
                log_box.mount(Static(f"    • [cyan]{c.get('short_hash', '')}[/cyan] - {c.get('subject', '')}"))
            log_box.mount(Static(""))  # spacing
