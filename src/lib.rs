pub mod config;
pub mod git;
pub mod tui;

use clap::{Parser, Subcommand};
use std::io::{self, Write};
use std::process::Command;

use config::{
    append_push_log, load_config, load_push_log, save_config, Config, LogCommit, LogEntry,
};
use git::{get_all_commits, get_today_str, run_git, run_push_check};
use tui::run_tui;

#[derive(Parser)]
#[command(
    name = "cadence",
    about = "Schedule git commits to appear as steady daily progress on GitHub"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Initialize Cadence configuration and guide in current repository
    Init,
    /// Stage files and commit with Release-Date trailer
    Commit {
        /// Commit message
        message: String,
        /// Optional release date (YYYY-MM-DD)
        #[arg(short, long)]
        date: Option<String>,
    },
    /// Relabel Release-Date trailer on an existing commit
    Relabel {
        /// Commit ref (e.g. HEAD, HEAD~1)
        #[arg(default_value = "HEAD")]
        ref_spec: String,
        /// New release date (YYYY-MM-DD)
        date: String,
    },
    /// View repository commit pacing status summary
    Status,
    /// View push history log
    Log,
    /// Trigger manual push check decision logic
    Push,
    /// Passthrough standard git command
    #[command(external_subcommand)]
    GitPassthrough(Vec<String>),
}

fn prompt(label: &str, default: &str) -> String {
    print!("{} [{}]: ", label, default);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let trimmed = input.trim();
    if trimmed.is_empty() {
        default.to_string()
    } else {
        trimmed.to_string()
    }
}

fn init_repo() {
    println!("Initializing Cadence in current repository...");
    let repo_path = prompt("Repository path", ".");
    let remote = prompt("Remote name", "origin");
    let branch = prompt("Branch name", "main");
    let timezone = prompt("Timezone", "Asia/Kolkata");

    let cfg = Config {
        repo_path,
        remote,
        branch,
        timezone,
    };
    if let Err(e) = save_config(&cfg) {
        eprintln!("Error saving config: {}", e);
        return;
    }

    let guide_content = r#"# Cadence Project Guide & Instructions

This repository is managed by **Cadence** - an automated Git progress pacing system.

## 🤖 Instructions for AI Coding Assistants & Developers

When making commits in this project, **always use `cad` or `cadence commit` instead of standard `git commit`**. 
Cadence automatically attaches the required `Release-Date: YYYY-MM-DD` trailer to ensure steady progress pacing on GitHub.

### 📌 Cadence CLI Commands Reference

| Command | Usage & Purpose |
|---|---|
| `cad commit "message"` | Stage files & create a commit labeled with **today's date**. |
| `cad commit "message" --date YYYY-MM-DD` | Stage files & pre-label a commit for a **future date**. |
| `cad status` | Show current commit counts (Pushed vs. Pending vs. Scheduled vs. Unlabeled). |
| `cad push` | Manually execute push check to advance remote `origin/main` for eligible commits. |
| `cad relabel [HEAD|hash] YYYY-MM-DD` | Modify/fix the `Release-Date` trailer on an existing commit. |
| `cad log` | Display audit log of previous pushes. |
| `cad` | Launch the interactive Terminal Dashboard (TUI). |
| `cad <git-command>` | Passthrough standard git commands (e.g. `cad diff`, `cad status`, `cad checkout`). |

---

## ⚙️ Configuration Files
- `.cadence.json`: Local settings (remote name, branch, timezone).
- `.cadence_log.json`: Push history audit log.
- `.cadence_guide.md`: This reference file.
"#;
    let _ = std::fs::write(".cadence_guide.md", guide_content);

    println!("Cadence initialized successfully!");
    println!("Created .cadence.json");
    println!("Created .cadence_guide.md for developer and AI guidance.");
}

fn commit_with_date(message: &str, date_str: Option<String>) {
    let cfg = load_config();
    let today = date_str.unwrap_or_else(|| get_today_str(&cfg.timezone));
    let full_msg = format!("{}\n\nRelease-Date: {}", message, today);

    let _ = run_git(&["add", "-A"], &cfg.repo_path);
    let (code, stdout, stderr) = run_git(&["commit", "-m", &full_msg], &cfg.repo_path);
    if code == 0 {
        println!("{}", stdout);
    } else {
        eprintln!("{}", stderr);
    }
}

fn relabel_commit(commit_ref: &str, date_str: &str) {
    let cfg = load_config();
    let (code, stdout, _) = run_git(&["log", "-1", "--format=%B", commit_ref], &cfg.repo_path);
    if code != 0 {
        eprintln!("Error: Commit ref '{}' not found.", commit_ref);
        return;
    }

    let msg = stdout.trim();
    let re = regex::Regex::new(r"(?i)Release-Date:\s*\d{4}-\d{2}-\d{2}").unwrap();
    let new_msg = if re.is_match(msg) {
        re.replace(msg, format!("Release-Date: {}", date_str))
            .to_string()
    } else {
        format!("{}\n\nRelease-Date: {}", msg, date_str)
    };

    let (code, stdout, stderr) = run_git(&["commit", "--amend", "-m", &new_msg], &cfg.repo_path);
    if code == 0 {
        println!("Relabeled commit successfully.");
        println!("{}", stdout);
    } else {
        eprintln!("{}", stderr);
    }
}

fn print_status() {
    let cfg = load_config();
    let today = get_today_str(&cfg.timezone);
    println!("Cadence Status for repository: {}", cfg.repo_path);
    println!(
        "Remote: {} | Branch: {} | Timezone: {} | Today: {}\n",
        cfg.remote, cfg.branch, cfg.timezone, today
    );

    let commits = get_all_commits(&cfg.repo_path, &cfg.remote, &cfg.branch);
    if commits.is_empty() {
        println!("No commits found.");
        return;
    }

    let pushed_count = commits.iter().filter(|c| c.pushed).count();
    let pending_count = commits
        .iter()
        .filter(|c| !c.pushed && c.release_date.as_ref().is_some_and(|d| d <= &today))
        .count();
    let scheduled_count = commits
        .iter()
        .filter(|c| !c.pushed && c.release_date.as_ref().is_some_and(|d| d > &today))
        .count();

    let unlabeled_count = commits
        .iter()
        .filter(|c| !c.pushed && c.release_date.is_none())
        .count();

    println!("Pushed commits:          {}", pushed_count);
    println!("Pending due commits:     {}", pending_count);
    println!("Scheduled future commits: {}", scheduled_count);
    if unlabeled_count > 0 {
        println!(
            "\x1b[91m⚠️ Unlabeled commits:    {} (queue blocked until labeled!)\x1b[0m",
            unlabeled_count
        );
    }
    println!();
}

fn print_log() {
    let logs = load_push_log();
    if logs.is_empty() {
        println!("No push history recorded yet.");
        return;
    }
    println!("=== Cadence Push History Log ===");
    for entry in logs.iter().rev() {
        println!("🚀 Push at {} - {}", entry.timestamp, entry.message);
        for c in &entry.commits {
            println!("   • {} - {} ({})", c.short_hash, c.subject, c.release_date);
        }
    }
}

pub fn entry() {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Init) => init_repo(),
        Some(Commands::Commit { message, date }) => commit_with_date(&message, date),
        Some(Commands::Relabel { ref_spec, date }) => relabel_commit(&ref_spec, &date),
        Some(Commands::Status) => print_status(),
        Some(Commands::Log) => print_log(),
        Some(Commands::Push) => {
            let res = run_push_check(false);
            println!("{}", res.message);
            if res.pushed {
                let entry = LogEntry {
                    timestamp: chrono::Local::now().to_rfc3339(),
                    message: res.message.clone(),
                    count: res.count,
                    commits: res
                        .pushed_commits
                        .iter()
                        .map(|c| LogCommit {
                            short_hash: c.short_hash.clone(),
                            subject: c.subject.clone(),
                            release_date: c.release_date.clone().unwrap_or_default(),
                        })
                        .collect(),
                };
                append_push_log(entry);
            }
        }
        Some(Commands::GitPassthrough(args)) => {
            let _ = Command::new("git").args(&args).status();
        }
        None => {
            let res = run_push_check(true);
            if res.pushed {
                let entry = LogEntry {
                    timestamp: chrono::Local::now().to_rfc3339(),
                    message: res.message.clone(),
                    count: res.count,
                    commits: res
                        .pushed_commits
                        .iter()
                        .map(|c| LogCommit {
                            short_hash: c.short_hash.clone(),
                            subject: c.subject.clone(),
                            release_date: c.release_date.clone().unwrap_or_default(),
                        })
                        .collect(),
                };
                append_push_log(entry);
            }
            if let Err(e) = run_tui(res) {
                eprintln!("Error running TUI: {}", e);
            }
        }
    }
}
