import re
import subprocess
import datetime
from typing import List, Dict, Optional, Tuple
import pytz
from config_manager import load_config

TRAILER_PREFIX = "Release-Date:"

def run_git_command(args: List[str], cwd: str = ".") -> Tuple[int, str, str]:
    result = subprocess.run(
        ["git"] + args,
        cwd=cwd,
        capture_output=True,
        text=True
    )
    return result.returncode, result.stdout.strip(), result.stderr.strip()

def get_today_str(tz_name: str) -> str:
    try:
        tz = pytz.timezone(tz_name)
        now = datetime.datetime.now(tz)
    except Exception:
        now = datetime.datetime.now()
    return now.strftime("%Y-%m-%d")

def parse_release_date(commit_body: str) -> Optional[str]:
    match = re.search(r"Release-Date:\s*(\d{4}-\d{2}-\d{2})", commit_body, re.IGNORECASE)
    if match:
        return match.group(1).strip()
    return None

def get_all_commits(cwd: str = ".", remote: str = "origin", branch: str = "main") -> List[Dict]:
    # Fetch quiet first
    run_git_command(["fetch", "-q", remote], cwd=cwd)
    
    # Check remote head
    code, remote_head, _ = run_git_command(["rev-parse", f"{remote}/{branch}"], cwd=cwd)
    if code != 0:
        remote_head = ""

    # Get log format: hash|subject|body
    fmt = "%H%x1f%s%x1f%b%x1e"
    code, stdout, _ = run_git_command(["log", "--reverse", f"--format={fmt}"], cwd=cwd)
    if code != 0 or not stdout:
        return []

    raw_commits = stdout.split("\x1e")
    commits = []
    
    is_pushed = True if remote_head else False
    
    for raw in raw_commits:
        raw = raw.strip()
        if not raw:
            continue
        parts = raw.split("\x1f")
        commit_hash = parts[0]
        subject = parts[1] if len(parts) > 1 else ""
        body = parts[2] if len(parts) > 2 else ""

        rel_date = parse_release_date(body) or parse_release_date(subject)
        
        commit_info = {
            "hash": commit_hash,
            "short_hash": commit_hash[:7],
            "subject": subject,
            "release_date": rel_date,
            "pushed": is_pushed
        }
        
        commits.append(commit_info)
        
        if commit_hash == remote_head:
            is_pushed = False  # Commits after remote_head are unpushed

    return commits

def run_push_check(config_path: str = "config.json") -> Dict:
    cfg = load_config(config_path)
    cwd = cfg.get("repo_path", ".")
    remote = cfg.get("remote", "origin")
    branch = cfg.get("branch", "main")
    tz_name = cfg.get("timezone", "Asia/Kolkata")
    today = get_today_str(tz_name)

    # Fetch remote
    run_git_command(["fetch", "-q", remote], cwd=cwd)
    
    code, remote_head, _ = run_git_command(["rev-parse", f"{remote}/{branch}"], cwd=cwd)
    if code != 0:
        remote_head = ""

    # Fetch local commits after remote head
    if remote_head:
        rev_range = f"{remote_head}..HEAD"
    else:
        rev_range = "HEAD"

    fmt = "%H%x1f%s%x1f%b%x1e"
    code, stdout, _ = run_git_command(["log", "--reverse", f"--format={fmt}", rev_range], cwd=cwd)
    
    if code != 0 or not stdout.strip():
        return {
            "pushed": False,
            "count": 0,
            "message": "No unpushed commits found.",
            "target": None,
            "unlabeled_found": False
        }

    raw_commits = stdout.split("\x1e")
    pending = []
    for raw in raw_commits:
        raw = raw.strip()
        if not raw:
            continue
        parts = raw.split("\x1f")
        commit_hash = parts[0]
        subject = parts[1] if len(parts) > 1 else ""
        body = parts[2] if len(parts) > 2 else ""
        rel_date = parse_release_date(body) or parse_release_date(subject)

        pending.append({
            "hash": commit_hash,
            "short_hash": commit_hash[:7],
            "subject": subject,
            "release_date": rel_date
        })

    target = None
    pushed_commits = []
    unlabeled_found = False

    for commit in pending:
        if commit["release_date"] is None:
            unlabeled_found = True
            break  # Stop at unlabeled commit
        if commit["release_date"] <= today:
            target = commit
            pushed_commits.append(commit)
        else:
            break  # Future dated commit, stop here

    if target:
        push_code, _, push_err = run_git_command(["push", remote, f"{target['hash']}:refs/heads/{branch}"], cwd=cwd)
        if push_code == 0:
            return {
                "pushed": True,
                "count": len(pushed_commits),
                "message": f"Successfully pushed {len(pushed_commits)} commit(s) up to {target['short_hash']} ({target['release_date']}).",
                "target": target,
                "pushed_commits": pushed_commits,
                "unlabeled_found": unlabeled_found
            }
        else:
            return {
                "pushed": False,
                "count": 0,
                "message": f"Git push failed: {push_err}",
                "target": None,
                "unlabeled_found": unlabeled_found
            }
    else:
        return {
            "pushed": False,
            "count": 0,
            "message": "No commits due for release today.",
            "target": None,
            "unlabeled_found": unlabeled_found
        }
