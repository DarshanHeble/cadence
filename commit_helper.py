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
