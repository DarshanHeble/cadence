use crate::config::load_config;
use chrono::Local;
use chrono_tz::Tz;
use regex::Regex;
use std::process::Command;

#[derive(Debug, Clone)]
pub struct CommitInfo {
    pub hash: String,
    pub short_hash: String,
    pub subject: String,
    pub release_date: Option<String>,
    pub pushed: bool,
}

#[derive(Debug, Clone)]
pub struct PushCheckResult {
    pub pushed: bool,
    pub count: usize,
    pub message: String,
    pub target_hash: Option<String>,
    pub pushed_commits: Vec<CommitInfo>,
    pub unlabeled_found: bool,
}

pub fn get_today_str(tz_name: &str) -> String {
    if let Ok(tz) = tz_name.parse::<Tz>() {
        let now = Local::now().with_timezone(&tz);
        now.format("%Y-%m-%d").to_string()
    } else {
        Local::now().format("%Y-%m-%d").to_string()
    }
}

pub fn run_git(args: &[&str], cwd: &str) -> (i32, String, String) {
    let output = Command::new("git").args(args).current_dir(cwd).output();

    match output {
        Ok(out) => {
            let code = out.status.code().unwrap_or(-1);
            let stdout = String::from_utf8_lossy(&out.stdout).trim().to_string();
            let stderr = String::from_utf8_lossy(&out.stderr).trim().to_string();
            (code, stdout, stderr)
        }
        Err(e) => (-1, "".to_string(), e.to_string()),
    }
}

pub fn parse_release_date(text: &str) -> Option<String> {
    let re = Regex::new(r"(?i)Release-Date:\s*(\d{4}-\d{2}-\d{2})").ok()?;
    re.captures(text).map(|cap| cap[1].to_string())
}

pub fn get_all_commits(cwd: &str, remote: &str, branch: &str) -> Vec<CommitInfo> {
    let _ = run_git(&["fetch", "-q", remote], cwd);
    let (code, remote_head, _) = run_git(&["rev-parse", &format!("{}/{}", remote, branch)], cwd);
    let remote_head = if code == 0 {
        remote_head
    } else {
        String::new()
    };

    let fmt = "%H%x1f%s%x1f%b%x1e";
    let (code, stdout, _) = run_git(&["log", "--reverse", &format!("--format={}", fmt)], cwd);
    if code != 0 || stdout.is_empty() {
        return Vec::new();
    }

    let mut commits = Vec::new();
    let mut is_pushed = !remote_head.is_empty();

    for raw in stdout.split('\x1e') {
        let raw = raw.trim();
        if raw.is_empty() {
            continue;
        }
        let parts: Vec<&str> = raw.split('\x1f').collect();
        let commit_hash = parts.first().unwrap_or(&"").to_string();

        let subject = parts.get(1).unwrap_or(&"").to_string();
        let body = parts.get(2).unwrap_or(&"").to_string();

        let rel_date = parse_release_date(&body).or_else(|| parse_release_date(&subject));
        let short_hash = if commit_hash.len() >= 7 {
            commit_hash[..7].to_string()
        } else {
            commit_hash.clone()
        };

        commits.push(CommitInfo {
            hash: commit_hash.clone(),
            short_hash,
            subject,
            release_date: rel_date,
            pushed: is_pushed,
        });

        if commit_hash == remote_head {
            is_pushed = false;
        }
    }

    commits
}

pub fn run_push_check() -> PushCheckResult {
    let cfg = load_config();
    let cwd = &cfg.repo_path;
    let remote = &cfg.remote;
    let branch = &cfg.branch;
    let today = get_today_str(&cfg.timezone);

    let _ = run_git(&["fetch", "-q", remote], cwd);
    let (code, remote_head, _) = run_git(&["rev-parse", &format!("{}/{}", remote, branch)], cwd);
    let remote_head = if code == 0 {
        remote_head
    } else {
        String::new()
    };

    let rev_range = if !remote_head.is_empty() {
        format!("{}..HEAD", remote_head)
    } else {
        "HEAD".to_string()
    };

    let fmt = "%H%x1f%s%x1f%b%x1e";
    let (code, stdout, _) = run_git(
        &["log", "--reverse", &format!("--format={}", fmt), &rev_range],
        cwd,
    );

    if code != 0 || stdout.is_empty() {
        return PushCheckResult {
            pushed: false,
            count: 0,
            message: "No unpushed commits found.".to_string(),
            target_hash: None,
            pushed_commits: Vec::new(),
            unlabeled_found: false,
        };
    }

    let mut pending = Vec::new();
    for raw in stdout.split('\x1e') {
        let raw = raw.trim();
        if raw.is_empty() {
            continue;
        }
        let parts: Vec<&str> = raw.split('\x1f').collect();
        let commit_hash = parts.first().unwrap_or(&"").to_string();

        let subject = parts.get(1).unwrap_or(&"").to_string();
        let body = parts.get(2).unwrap_or(&"").to_string();
        let rel_date = parse_release_date(&body).or_else(|| parse_release_date(&subject));
        let short_hash = if commit_hash.len() >= 7 {
            commit_hash[..7].to_string()
        } else {
            commit_hash.clone()
        };

        pending.push(CommitInfo {
            hash: commit_hash,
            short_hash,
            subject,
            release_date: rel_date,
            pushed: false,
        });
    }

    let mut target: Option<CommitInfo> = None;
    let mut pushed_commits = Vec::new();
    let mut unlabeled_found = false;

    for commit in pending {
        if commit.release_date.is_none() {
            unlabeled_found = true;
            break; // Stop at unlabeled commit
        }

        let date = commit.release_date.as_ref().unwrap();
        if date <= &today {
            target = Some(commit.clone());
            pushed_commits.push(commit);
        } else {
            break; // Future-dated commit, stop
        }
    }

    if let Some(t) = target {
        let (push_code, _, push_err) = run_git(
            &["push", remote, &format!("{}:refs/heads/{}", t.hash, branch)],
            cwd,
        );
        if push_code == 0 {
            let msg = format!(
                "Successfully pushed {} commit(s) up to {} ({}).",
                pushed_commits.len(),
                t.short_hash,
                t.release_date.unwrap_or_default()
            );
            PushCheckResult {
                pushed: true,
                count: pushed_commits.len(),
                message: msg,
                target_hash: Some(t.hash),
                pushed_commits,
                unlabeled_found,
            }
        } else {
            PushCheckResult {
                pushed: false,
                count: 0,
                message: format!("Git push failed: {}", push_err),
                target_hash: None,
                pushed_commits: Vec::new(),
                unlabeled_found,
            }
        }
    } else {
        PushCheckResult {
            pushed: false,
            count: 0,
            message: "No commits due for release today.".to_string(),
            target_hash: None,
            pushed_commits: Vec::new(),
            unlabeled_found,
        }
    }
}
