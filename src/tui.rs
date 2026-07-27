use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Tabs},
    Terminal,
};
use std::io;
use crate::config::{load_config, load_push_log, append_push_log, LogCommit, LogEntry};

use crate::git::{get_all_commits, get_today_str, run_push_check, PushCheckResult};

#[derive(Copy, Clone, PartialEq, Eq)]
enum TabIndex {
    Timeline = 0,
    Batch = 1,
    Log = 2,
    Settings = 3,
}


pub fn run_tui(initial_push_res: PushCheckResult) -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut current_tab = TabIndex::Timeline;
    let mut push_result = initial_push_res;

    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints(
                    [
                        Constraint::Length(3),
                        Constraint::Min(0),
                        Constraint::Length(3),
                    ]
                    .as_ref(),
                )
                .split(f.size());

            // Navigation Tabs
            let titles = vec!["[1] Timeline", "[2] Today's Batch", "[3] Push Log", "[4] Settings"];
            let tabs = Tabs::new(titles)
                .block(Block::default().borders(Borders::ALL).title(" Cadence Progress Pacer "))
                .select(current_tab as usize)
                .style(Style::default().fg(Color::Cyan))
                .highlight_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD));
            f.render_widget(tabs, chunks[0]);

            // Content Body based on selected Tab
            match current_tab {
                TabIndex::Timeline => {
                    let cfg = load_config();
                    let today = get_today_str(&cfg.timezone);
                    let commits = get_all_commits(&cfg.repo_path, &cfg.remote, &cfg.branch);

                    let mut items = Vec::new();
                    if commits.is_empty() {
                        items.push(ListItem::new("No git commits found in this repository."));
                    } else {
                        let has_unlabeled = commits.iter().any(|c| !c.pushed && c.release_date.is_none());
                        if has_unlabeled {
                            items.push(ListItem::new(Span::styled(
                                "⚠️ UNLABELED COMMITS FOUND: Queue is blocked until trailers are added!",
                                Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
                            )));
                            items.push(ListItem::new(""));
                        }

                        for c in commits {
                            let status_span = if c.pushed {
                                Span::styled("■ Pushed", Style::default().fg(Color::Green))
                            } else if let Some(ref d) = c.release_date {
                                if d <= &today {
                                    Span::styled("□ Pending (Due)", Style::default().fg(Color::Yellow))
                                } else {
                                    Span::styled("□ Scheduled (Future)", Style::default().fg(Color::DarkGray))
                                }
                            } else {
                                Span::styled("⚠️ Unlabeled", Style::default().fg(Color::Red))
                            };

                            let date_str = c.release_date.unwrap_or_else(|| "MISSING".to_string());
                            let line = Line::from(vec![
                                status_span,
                                Span::raw(format!(" [{}] {} ", c.short_hash, c.subject)),
                                Span::styled(format!("({})", date_str), Style::default().fg(Color::Cyan)),
                            ]);
                            items.push(ListItem::new(line));
                        }
                    }

                    let list = List::new(items).block(Block::default().borders(Borders::ALL).title(" Commit Pacing Timeline "));
                    f.render_widget(list, chunks[1]);
                }
                TabIndex::Batch => {
                    let mut text = Vec::new();
                    text.push(Line::from(Span::styled("Today's Batch & Launch Push Check", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))));
                    text.push(Line::from(""));

                    if push_result.pushed {
                        text.push(Line::from(Span::styled(format!("✅ SUCCESS: {}", push_result.message), Style::default().fg(Color::Green))));
                        text.push(Line::from("Pushed commits:"));
                        for c in &push_result.pushed_commits {
                            text.push(Line::from(format!("  ✔ {} - {} ({})", c.short_hash, c.subject, c.release_date.as_deref().unwrap_or(""))));
                        }
                    } else {
                        text.push(Line::from(Span::styled(format!("ℹ️ STATUS: {}", push_result.message), Style::default().fg(Color::Yellow))));
                    }

                    if push_result.unlabeled_found {
                        text.push(Line::from(""));
                        text.push(Line::from(Span::styled("⚠️ Notice: Found unlabeled commit(s). Queue progression stopped.", Style::default().fg(Color::Red))));
                    }

                    text.push(Line::from(""));
                    text.push(Line::from(Span::styled("Press 'p' to recheck and push eligible commits now.", Style::default().fg(Color::Gray))));


                    let p = Paragraph::new(text).block(Block::default().borders(Borders::ALL).title(" Batch Push Result "));
                    f.render_widget(p, chunks[1]);
                }
                TabIndex::Log => {
                    let logs = load_push_log();
                    let mut items = Vec::new();

                    if logs.is_empty() {
                        items.push(ListItem::new("No push activity logged yet."));
                    } else {
                        for entry in logs.iter().rev() {
                            items.push(ListItem::new(Span::styled(
                                format!("🚀 Push on {} ({} commits)", entry.timestamp, entry.count),
                                Style::default().fg(Color::Green).add_modifier(Modifier::BOLD),
                            )));
                            items.push(ListItem::new(format!("   {}", entry.message)));
                            for c in &entry.commits {
                                items.push(ListItem::new(format!("     • {} - {}", c.short_hash, c.subject)));
                            }
                            items.push(ListItem::new(""));
                        }
                    }

                    let list = List::new(items).block(Block::default().borders(Borders::ALL).title(" Push History Log "));
                    f.render_widget(list, chunks[1]);
                }
                TabIndex::Settings => {
                    let cfg = load_config();
                    let text = vec![
                        Line::from(Span::styled("Cadence Configuration (.cadence.json)", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))),
                        Line::from(""),
                        Line::from(format!("Repository Path: {}", cfg.repo_path)),
                        Line::from(format!("Remote Name:     {}", cfg.remote)),
                        Line::from(format!("Branch Name:     {}", cfg.branch)),
                        Line::from(format!("Timezone:        {}", cfg.timezone)),
                        Line::from(""),
                        Line::from(Span::styled("Run 'cad init' to reconfigure settings interactively.", Style::default().fg(Color::DarkGray))),
                    ];
                    let p = Paragraph::new(text).block(Block::default().borders(Borders::ALL).title(" Active Settings "));
                    f.render_widget(p, chunks[1]);
                }
            }

            // Footer Keybinding Legend
            let footer_text = " [1-4] Tabs | [p] Push Recheck | [r] Refresh | [q] Quit";
            let footer = Paragraph::new(footer_text).block(Block::default().borders(Borders::ALL));
            f.render_widget(footer, chunks[2]);
        })?;

        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => break,
                    KeyCode::Char('1') => current_tab = TabIndex::Timeline,
                    KeyCode::Char('2') => current_tab = TabIndex::Batch,
                    KeyCode::Char('3') => current_tab = TabIndex::Log,
                    KeyCode::Char('4') => current_tab = TabIndex::Settings,
                    KeyCode::Char('p') => {
                        let res = run_push_check();
                        if res.pushed {
                            let entry = LogEntry {
                                timestamp: chrono::Local::now().to_rfc3339(),
                                message: res.message.clone(),
                                count: res.count,
                                commits: res.pushed_commits.iter().map(|c| LogCommit {
                                    short_hash: c.short_hash.clone(),
                                    subject: c.subject.clone(),
                                    release_date: c.release_date.clone().unwrap_or_default(),
                                }).collect(),
                            };
                            append_push_log(entry);
                        }
                        push_result = res;
                        current_tab = TabIndex::Batch;
                    }
                    KeyCode::Char('r') => {
                        // Triggers redraw
                    }
                    _ => {}
                }
            }
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
    terminal.show_cursor()?;
    Ok(())
}
