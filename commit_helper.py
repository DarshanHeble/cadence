import sys
import subprocess
import datetime
from typing import Optional
import pytz
from config_manager import load_config

def commit_with_date(message: str, date_str: Optional[str] = None, cwd: str = ".") -> int:
    cfg = load_config()
    if not cwd or cwd == ".":
        cwd = cfg.get("repo_path", ".")

    if not date_str:
        tz_name = cfg.get("timezone", "Asia/Kolkata")
        try:
            tz = pytz.timezone(tz_name)
            now = datetime.datetime.now(tz)
        except Exception:
            now = datetime.datetime.now()
        date_str = now.strftime("%Y-%m-%d")

    trailer = f"Release-Date: {date_str}"
    full_message = f"{message}\n\n{trailer}"

    # Stage all updated/new files (git add -A)
    subprocess.run(["git", "add", "-A"], cwd=cwd)

    # Execute git commit
    res = subprocess.run(["git", "commit", "-m", full_message], cwd=cwd)
    return res.returncode

def relabel_commit(commit_ref: str, date_str: str, cwd: str = ".") -> int:
    cfg = load_config()
    if not cwd or cwd == ".":
        cwd = cfg.get("repo_path", ".")

    # Get current commit message
    res = subprocess.run(["git", "log", "-1", "--format=%B", commit_ref], cwd=cwd, capture_output=True, text=True)
    if res.returncode != 0:
        print(f"Error: Commit ref '{commit_ref}' not found.")
        return res.returncode

    msg = res.stdout.strip()
    import re
    if re.search(r"Release-Date:\s*\d{4}-\d{2}-\d{2}", msg, re.IGNORECASE):
        new_msg = re.sub(r"Release-Date:\s*\d{4}-\d{2}-\d{2}", f"Release-Date: {date_str}", msg, flags=re.IGNORECASE)
    else:
        new_msg = f"{msg}\n\nRelease-Date: {date_str}"

    if commit_ref.upper() in ["HEAD", "@"]:
        commit_res = subprocess.run(["git", "commit", "--amend", "-m", new_msg], cwd=cwd)
        return commit_res.returncode
    else:
        print(f"Relabeling commit {commit_ref} with Release-Date: {date_str}...")
        # For non-HEAD commits, use git filter-repo or environment message edit
        commit_res = subprocess.run(["git", "commit", "--amend", "-m", new_msg], cwd=cwd)
        return commit_res.returncode

def main():
    args = sys.argv[1:]
    if not args:
        print("Usage: cadence commit \"commit message\" [--date YYYY-MM-DD]")
        sys.exit(1)

    message = args[0]
    date_str = None

    if "--date" in args:
        idx = args.index("--date")
        if idx + 1 < len(args):
            date_str = args[idx + 1]

    code = commit_with_date(message, date_str)
    sys.exit(code)

if __name__ == "__main__":
    main()

