from textual.app import RenderResult
from textual.widget import Widget
from textual.widgets import Static
from textual.containers import Container, VerticalScroll
from push_check import get_all_commits, get_today_str
from config_manager import load_config

class TimelineView(Container):
    def compose(self):
        yield Static("[bold cyan]Timeline View[/bold cyan]\n", id="timeline-title")
        yield VerticalScroll(id="timeline-content")

    def refresh_timeline(self):
        cfg = load_config()
        cwd = cfg.get("repo_path", ".")
        remote = cfg.get("remote", "origin")
        branch = cfg.get("branch", "main")
        tz_name = cfg.get("timezone", "Asia/Kolkata")
        today = get_today_str(tz_name)

        commits = get_all_commits(cwd, remote, branch)
        content_box = self.query_one("#timeline-content", VerticalScroll)
        content_box.remove_children()

        if not commits:
            content_box.mount(Static("[dim]No git commits found in this repository.[/dim]"))
            return

        # Group by release date
        grouped = {}
        unlabeled = []
        for c in commits:
            rd = c["release_date"]
            if not rd:
                unlabeled.append(c)
            else:
                grouped.setdefault(rd, []).append(c)

        if unlabeled:
            unlabeled_text = "[bold red]⚠️ UNLABELED COMMITS WARNING:[/bold red]\n"
            for c in unlabeled:
                unlabeled_text += f"  [yellow]• {c['short_hash']}[/yellow] - {c['subject']} [bold red](Missing Release-Date trailer)[/bold red]\n"
            content_box.mount(Static(unlabeled_text, classes="warning-box"))

        sorted_dates = sorted(grouped.keys(), reverse=True)
        for date_key in sorted_dates:
            is_today = (date_key == today)
            date_header = f"[bold green]📅 {date_key} (TODAY)[/bold green]" if is_today else f"[bold blue]📅 {date_key}[/bold blue]"
            
            content_box.mount(Static(date_header))
            for c in grouped[date_key]:
                if c["pushed"]:
                    status = "[green]■ Pushed[/green]"
                elif c["release_date"] <= today:
                    status = "[yellow]□ Pending (Due)[/yellow]"
                else:
                    status = "[dim]□ Scheduled (Future)[/dim]"
                
                line = f"  {status} [bold]{c['short_hash']}[/bold] - {c['subject']}"
                content_box.mount(Static(line))
            content_box.mount(Static(""))  # spacing
