from textual.containers import Container, VerticalScroll
from textual.widgets import Static, Button

class BatchView(Container):
    def compose(self):
        yield Static("[bold cyan]Today's Batch & Push Check[/bold cyan]\n", id="batch-title")
        yield Static("Press [bold yellow]'p'[/bold yellow] or click below to recheck and push eligible commits.\n", id="batch-sub")
        yield Button("Run Push Check Now (p)", id="btn-recheck", variant="primary")
        yield VerticalScroll(id="batch-results")

    def update_results(self, push_result: dict):
        results_box = self.query_one("#batch-results", VerticalScroll)
        results_box.remove_children()

        msg = push_result.get("message", "No recent action.")
        pushed = push_result.get("pushed", False)
        count = push_result.get("count", 0)
        unlabeled = push_result.get("unlabeled_found", False)

        if pushed:
            results_box.mount(Static(f"[bold green]✅ PUSH SUCCESSFUL:[/bold green] {msg}\n"))
            pushed_commits = push_result.get("pushed_commits", [])
            for c in pushed_commits:
                results_box.mount(Static(f"  [green]✔[/green] [bold]{c['short_hash']}[/bold] - {c['subject']} ({c['release_date']})"))
        else:
            results_box.mount(Static(f"[bold yellow]ℹ️ STATUS:[/bold yellow] {msg}\n"))

        if unlabeled:
            results_box.mount(Static("\n[bold red]⚠️ Notice:[/bold red] Found unlabeled commits in the queue. Progression stopped until labeled."))
